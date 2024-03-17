mod parser;
use clap::Parser;
use log::log_enabled;
use std::{collections::HashMap, process};
use std::{fs, fs::File, path::Path};

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    compile: Option<bool>,
    #[arg(short, long)]
    keep: Option<String>,
    #[arg(short, long)]
    output: Option<Vec<String>>,
    #[arg(short, long)]
    lower_true: Option<bool>,
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
    let mut import_files: Vec<String> = Vec::new();
    let mut parse_que: Vec<String> = Vec::new();
    let mut import_translations: Option<HashMap<String, String>> = None;
    let mut translation: HashMap<String, String> = HashMap::new();
    let mut current_file_name = String::new();
    let mut outputname: Option<String>;
    let mut output_file = String::new();
    let python_command = "python3";

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
    match args.input.clone() {
        Some(x) => parse_que = x,
        None => parse_que = Vec::new(),
    }
    if let Some(x) = &args.compile {
        if let Some(y) = args.args.clone() {
            for arg in y {
                parse_que.push(arg);
            }
        }
    }
    for i in 0..parse_que.len() {
        match parser::parse_imports(&parse_que[i]) {
            Ok(x) => import_files.push(x[i].clone()),
            Err(_) => {
                panic!("failed to parse file");
                process::exit(0);
            }
        }
        for import_file in 0..import_files.len() {
            if Path::new(&import_files[import_file]).is_file()
                || !parse_que.contains(&import_files[import_file])
            {
                parse_que.push(import_files[import_file].clone())
            };
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
    match Parsing::new(
        args.clone(),
        parse_que.clone(),
        current_file_name,
        path_prefix.clone(),
        import_translations,
    )
    .parse()
    {
        Ok(_) => log::info!("success"),
        Err(_) => {
            log::info!("failed");
            for file in 0..parse_que.len() {
                let _ = fs::remove_file(
                    path_prefix.clone() + parser::change_filename(&parse_que[file], None).as_str(),
                );
            }
        }
    }
    match args.input {
        Some(x) => {
            let filename = Path::new(&x[0])
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();
        }
        None => {
            println!("no input")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Parsing {
    args: Args,
    output_file: Option<String>,
    parse_que: Vec<String>,
    outputname: Option<String>,
    current_file_name: String,
    path_prefix: String,
    import_translations: Option<HashMap<String, String>>,
}
impl Parsing {
    pub fn new(
        args: Args,
        parse_que: Vec<String>,
        current_file_name: String,
        path_prefix: String,
        import_translations: Option<HashMap<String, String>>,
    ) -> Self {
        Parsing {
            args: args,
            output_file: None,
            parse_que: parse_que,
            outputname: None,
            path_prefix,
            current_file_name: current_file_name,
            import_translations,
        }
    }
    pub fn parse(self) -> Result<Parsing, String> {
        let mut parse_que = self.parse_que;
        let mut outputname = self.outputname;
        let mut args = self.args;
        let mut current_file_name = self.current_file_name;

        for file in 0..parse_que.len() {
            current_file_name = parse_que[file].clone();
            match &args.output {
                None => outputname = None,
                Some(x) => {
                    if Path::new(&x[0]).is_dir() {
                        let new_file_name = parser::change_filename(
                            Path::new(&parse_que[file])
                                .file_name()
                                .unwrap()
                                .to_string_lossy()
                                .to_string()
                                .as_str(),
                            None,
                        );
                    }
                }
            }
            match parser::parse_file(
                parse_que[file].as_str(),
                args.lower_true.unwrap(),
                self.path_prefix.as_str(),
                outputname.clone(),
                self.import_translations.clone(),
            ) {
                Ok(_) => log::info!("parsed file: {}", parse_que[file].as_str()),
                Err(_) => panic!("failed to parse file: {}", parse_que[file].as_str()),
            }
        }
        Ok(Parsing {
            outputname: outputname,
            parse_que: parse_que,
            args: args,
            output_file: None,
            path_prefix: self.path_prefix,
            current_file_name: current_file_name,
            import_translations: self.import_translations,
        })
    }
}

pub fn get_filename(file: &str) -> String {
    // let mut tmp: Vec<u8> = file.as_bytes().to_vec();

    // let output = String::from_utf8().unwrap();
    let output = file.split(".").collect::<Vec<&str>>()[0].to_string();
    output
}
