use chrono;
use clap::{ArgAction, Parser};
use std::fs;

#[derive(Debug, Parser)]
#[command(author, about, long_about = None)]
struct Args {
    #[arg(short, long, action=ArgAction::SetTrue, help="use a long listing format")]
    long: bool,
    path: String,
}

fn main() {
    let args = Args::parse();

    let paths = fs::read_dir(args.path).unwrap();
    if args.long {
        for path in paths {
            let file = match path {
                Ok(value) => value,
                Err(error) => panic!("{}", error),
            };
            let meta = fs::metadata(file.path()).unwrap();
            let dt: chrono::DateTime<chrono::Utc> = meta.created().unwrap().into();
            println!(
                "{} {} {} {}",
                format!("{:1$}", if meta.is_dir() { "d" } else { "-" }, 2),
                format!("{:1$}", meta.len(), 6),
                dt.format("%b %d %H:%M"),
                file.path().file_name().unwrap().to_str().unwrap()
            );
        }
    } else {
        let t: Vec<_> = paths
            .into_iter()
            .map(|p| p.unwrap().file_name().to_str().unwrap().to_string())
            .collect();
        println!("{}", t.join(" "));
    }
}
