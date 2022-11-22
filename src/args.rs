use clap::Parser;
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "")]
    path: String,
}

pub fn get_path() -> String {
    let args = Args::parse();
    let mut path = args.path;
    if path == "" {
        let dir = env::current_dir().expect("获取当前目录失败");
        path = dir.to_str().unwrap().to_string();
    }
    path
}
