// 版本号新增
pub fn version_add_one(vtype: i32, version: &str) -> String {
    let version = version.to_string();
    let version = version.split(".");
    let version = version.collect::<Vec<&str>>();
    let mut version = version
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let mut version = version
        .iter_mut()
        .map(|x| x.parse::<i32>().unwrap_or(0))
        .collect::<Vec<i32>>();
    if version.len() != 3 {
        version = vec![1, 0, 0];
    }
    if vtype == 1 {
        version[0] += 1;
        version[1] = 0;
        version[2] = 0;
    } else if vtype == 2 {
        version[1] += 1;
        version[2] = 0;
    } else if vtype == 3 {
        version[2] += 1;
    }
    let version = version
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let version = version.join(".");
    version
}
