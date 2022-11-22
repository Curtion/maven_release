use std::io;

mod args;
mod parse;
mod utils;

fn main() {
    let path = args::get_path();
    let dirs = parse::get_pom_all(&path);
    let services = parse::get_service_all(&dirs);
    let mut input = String::new();
    println!("请输入需要更新的服务名!");
    io::stdin().read_line(&mut input).expect("解析失败,请重试!");
    let service = services.iter().find(|service| service.name == input.trim());
    let version = match service {
        Some(service) => {
            let version = parse::get_last_version_for_service("artifactId>".to_string(), &service.path);
            println!("当前版本为:{}", version);
            Some(version)
        }
        None => {
            println!("没有找到该服务!");
            None
        }
    };
    match version {
        Some(last_tag) => {
            let mut input_type = String::new();
            println!("请首先确认保持git仓库的暂存区中没有pom.xml文件, 该操作可以在程序出错时恢复配置!");
            println!("------------------------------");
            println!("{} -> {}", input.trim(), last_tag);
            println!("请输入要更新的版本类型!");
            println!("1. 大版本");
            println!("2. 小版本");
            println!("3. 修复版本");
            io::stdin()
                .read_line(&mut input_type)
                .expect("输入解析错误!");
            let input_type = input_type.trim().parse::<i32>().expect("输入格式错误!");
            let version = utils::version_add_one(input_type, &last_tag);
            let service = service.unwrap();
            parse::set_self_version(&service, &version);
            parse::set_brother_version(&service, &services, &version);
        }
        None => {}
    }
}
