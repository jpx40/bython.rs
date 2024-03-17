mod parser;
use std::{collections::HashMap, process};

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
    let mut include_suffix = String::new();
    let mut import_files = Vec::new();
    let mut parse_que: Vec<String> = Vec::new();
    let mut import_translations: Option<HashMap<String, String>> = None;
    let mut translation: HashMap<String, String> = HashMap::new();

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
    for i in 0..parse_que.len() {
        match parser::parse_imports(&parse_que[i]) {
            Ok(x) => import_files.push(x),
            Err(_) => panic!("failed to parse file"),
        }
    }
    if path_prefix != "".to_string() {
        for file in 0..parse_que.len() {
            let file_val: String = path_prefix.to_string() + &get_filename(&parse_que[file]);

            match import_translations {
                Some(x) => translation = x.clone(),
                None => {
                    let nothing = "Nothing".to_string();
                }
            }
            translation.insert(file_val, parse_que[file].clone());
            import_translations = Some(translation.clone());
        }
    } else {
        import_translations = None;
    }
}

pub fn get_filename(file: &str) -> String {
    // let mut tmp: Vec<u8> = file.as_bytes().to_vec();

    // let output = String::from_utf8().unwrap();
    let output = file.split(".").collect::<Vec<&str>>()[0].to_string();
    output
}
