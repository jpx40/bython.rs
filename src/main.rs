mod parser;
use std::process;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    compile: Option<bool>,
    #[arg(short, long)]
    keep: Option<String>,
    #[arg(short, long)]
    output: Option<String>,
    #[arg(short, long)]
    lower_true: Option<String>,
    #[arg(long)]
    input: Option<Vec<String>>,
    #[arg(long)]
    args: Option<Vec<String>>,
}

fn main() {
    let args = Args::parse();
    let mut path_prefix = String::new();
    let mut path_suffix = String::new();
    let mut include = String::new();
    let mut include_prefix = String::new();
    let mut parse_que: Vec<String> = Vec::new();

    if let Some(x) = &args.output {
        if let Some(y) = args.compile {
            if y {
                //exit
                process::exit(0);
            }
        }
    }

    match &args.compile {
        Some(x) => {
            path_prefix = "".to_string();
        }
        None => {
            path_prefix = "python_".to_string();
        }
    }
    match &args.keep {
        Some(x) => {
            path_prefix = "".to_string();
        }
        None => {
            path_prefix = "python_".to_string();
        }
    }
    match args.input {
        Some(x) => parse_que = x,
        None => parse_que = Vec::new(),
    }
    if let Some(x) = &args.compile {
        if let Some(y) = args.args {
            for arg in y {
                parse_que.push(arg);
            }
        }
    }
    let i: usize = 0;
    while i < parse_que.len() {}
    {}
}
