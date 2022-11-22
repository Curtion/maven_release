use regex::{Captures, Regex};
use std::collections::HashSet;
use std::io::Write;
use std::{fs, path::Path};
use std::{fs::File, io::Read};

use crate::utils;

pub struct Service {
    pub name: String,
    pub path: String,
}
pub struct PService {
    pub name: String,
    pub path: String,
    pub parent: String,
}

pub fn get_pom_all(path: &str) -> Vec<Service> {
    let mut paths = Vec::new();
    let dirs = fs::read_dir(path).expect("读取目录失败");
    for entry in dirs {
        if let Ok(entry) = entry {
            let path = entry.path();
            let is_dir = path.is_dir();
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            let path = path.to_str().unwrap().to_string();
            if is_dir {
                paths.push(Service {
                    name: file_name,
                    path,
                });
            }
        }
    }
    paths
}

pub fn get_service_all(dirs: &Vec<Service>) -> Vec<PService> {
    let mut paths = Vec::new();
    for dir in dirs {
        let path = Path::new(&dir.path).join("pom.xml");
        if !Path::new(&path).exists() {
            continue;
        }
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let contents = contents
            .trim()
            .lines()
            .map(|part| {
                part.trim()
                    .split_inclusive(char::is_whitespace)
                    .filter(|part| !part.trim().is_empty())
                    .collect()
            })
            .collect::<Vec<String>>()
            .join("");
        let re = Regex::new(r"<module>([\s\S]*?)</module>").unwrap();
        match re.captures(&contents) {
            Some(_) => {
                for cap in re.captures_iter(&contents) {
                    paths.push(PService {
                        name: cap[1].to_string(),
                        path: Path::new(&dir.path)
                            .join(cap[1].to_string())
                            .to_str()
                            .unwrap()
                            .to_string(),
                        parent: dir.path.clone(),
                    });
                }
            }
            None => {
                paths.push(PService {
                    name: dir.name.clone(),
                    path: dir.path.clone(),
                    parent: String::new(),
                });
            }
        }
    }
    paths
}

pub fn get_last_version_for_service(str_start: String, path: &str) -> String {
    let path = Path::new(path).join("pom.xml");
    if !Path::new(&path).exists() {
        return String::new();
    }
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let contents = contents
        .trim()
        .lines()
        .map(|part| {
            part.trim()
                .split_inclusive(char::is_whitespace)
                .filter(|part| !part.trim().is_empty())
                .collect()
        })
        .collect::<Vec<String>>()
        .join("");
    let re = Regex::new(&(str_start + r"<version>([\s\S]*?)</version>")).unwrap();
    match re.captures(&contents) {
        Some(cap) => cap[1].to_string(),
        None => String::new(),
    }
}

pub fn set_self_version(service: &PService, version: &str) {
    let path = Path::new(&service.path).join("pom.xml");
    if !Path::new(&path).exists() {
        panic!("修改{}版本失败, pom.xml文件不存在", service.name);
    }
    let mut file = File::open(&path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let re = Regex::new(r"artifactId>(\s+)<version>([\s\S]*?)</version>").unwrap();
    let data = re.replace(&contents, |caps: &Captures| {
        format!("artifactId>{}<version>{}</version>", &caps[1], version)
    });
    match File::create(&path).unwrap().write_all(data.as_bytes()) {
        Ok(_) => {}
        Err(err) => panic!("修改{}版本失败:{}", service.name, err),
    }
}

pub fn set_brother_version(service: &PService, services: &Vec<PService>, version: &str) {
    let mut list = HashSet::new();
    for item in services {
        if item.parent != "" {
            list.insert(&item.parent);
        }
    }
    for item in list {
        println!("受影响的服务:{}", item);
        let path = Path::new(&item).join("pom.xml");
        if !Path::new(&path).exists() {
            panic!("修改{}版本失败, pom.xml文件不存在", item);
        }
        let mut file = File::open(&path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let re = Regex::new(
            &(r"<artifactId>".to_string()
                + &service.name
                + r"</artifactId>(\s+)<version>([\s\S]*?)</version>"),
        )
        .unwrap();
        let last_version = get_last_version_for_service(
            r"<artifactId>".to_string() + &service.name + r"</artifactId>",
            item,
        );
        if last_version == "" {
            continue;
        }
        let data = re.replace(&contents, |caps: &Captures| {
            format!(
                "<artifactId>{}</artifactId>{}<version>{}</version>",
                service.name, &caps[1], version
            )
        });
        match File::create(&path).unwrap().write_all(data.as_bytes()) {
            Ok(_) => {
                set_parent_version(&PService {
                    name: item.clone(),
                    path: String::new(),
                    parent: item.to_string(),
                });
            }
            Err(err) => panic!("修改{}版本失败:{}", item, err),
        }
    }
}

fn set_parent_version(service: &PService) {
    let path = Path::new(&service.parent).join("pom.xml");
    if !Path::new(&path).exists() {
        panic!("修改{}版本失败, pom.xml文件不存在", service.name);
    }
    let mut file = File::open(&path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let re = Regex::new(r"packaging>(\s+)<version>([\s\S]*?)</version>").unwrap();
    let version = get_last_version_for_service("packaging>".to_string(), &service.parent);
    let version = utils::version_add_one(3, &version);
    let data = re.replace(&contents, |caps: &Captures| {
        format!("packaging>{}<version>{}</version>", &caps[1], version)
    });
    match File::create(&path).unwrap().write_all(data.as_bytes()) {
        Ok(_) => {
            set_parent_children_version(service, &version);
        }
        Err(err) => panic!("修改{}父级版本失败:{}", service.name, err),
    }
}

fn set_parent_children_version(service: &PService, version: &str) {
    let dirs = get_service_all(&vec![Service {
        name: service.name.clone(),
        path: service.parent.clone(),
    }]);
    for dir in dirs {
        let path = Path::new(&dir.path).join("pom.xml");
        if !Path::new(&path).exists() {
            panic!("修改{}版本失败, pom.xml文件不存在", dir.name);
        }
        let mut file = File::open(&path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let re = Regex::new(r"groupId>(\s+)<version>([\s\S]*?)</version>").unwrap();
        let data = re.replace(&contents, |caps: &Captures| {
            format!("groupId>{}<version>{}</version>", &caps[1], version)
        });
        match File::create(&path).unwrap().write_all(data.as_bytes()) {
            Ok(_) => {}
            Err(err) => panic!("修改{}版本失败:{}", dir.name, err),
        }
    }
}
