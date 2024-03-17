use regex::Regex;
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::prelude::*;
use std::iter;
use std::path::Path;
use std::{fs, u32};
#[inline]
fn ends_with(word: &str, suffix: &str) -> bool {
    word.ends_with(suffix)
}

fn change_filename(name: &str, output_name: Option<&str>) -> String {
    if let Some(output_name) = output_name {
        return output_name.to_string();
    } else if ends_with(&name, ".by") {
        return name.to_string().replace(".by", ".py");
    } else {
        return name.to_string() + ".py";
    }
}

pub fn parse_imports(file_name: &str) -> Result<Vec<String>, String> {
    let mut results: Vec<String> = vec![];
    let re1 = Regex::new(r#"(import\s)[\w.]+(;|\s|$)"#).unwrap();
    let re2 = Regex::new(r#"(from\s)[\w.]+(import\s)[\w.]+(;|\s|$)"#).unwrap();
    let mut import_with_suffix: Vec<String> = Vec::new();
    let mut infile_str = String::new();

    let infile: String = std::fs::read_to_string(file_name).expect("Unable to read file");

    for line in infile.lines() {
        infile_str.push_str(line);
        infile_str.push('\n');
    }
    let imports1: Vec<&str> = re1
        .find_iter(infile_str.as_str())
        .map(|m| m.as_str())
        .collect();
    let imports2: Vec<&str> = re2
        .find_iter(infile_str.as_str())
        .map(|m| m.as_str())
        .collect();
    import_with_suffix.extend(imports1.iter().map(|m| m.to_string()));
    import_with_suffix.extend(imports2.iter().map(|m| m.to_string()));
    // for import in import_with_suffix.iter() {
    //     let mut words = import.split_whitespace();
    // }
    Ok(import_with_suffix)
}

pub fn parse_file(
    filpath: &str,
    add_true_line: bool,
    file_name_prefix: &str,
    output_name: Option<&str>,
    change_imports: Option<HashMap<String, String>>,
) -> Result<(), String> {
    let mut results: Vec<String> = vec![];
    let mut infile_str = String::new();
    let mut infile_str_indented = String::new();
    let mut infile_str_raw = String::new();
    let mut infile: String = std::fs::read_to_string(filpath).expect("Unable to read file");

    let base_name: String = std::path::Path::new(filpath)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let dir_name: String = std::path::Path::new(filpath)
        .to_str()
        .unwrap()
        .to_string()
        .trim_end_matches(base_name.as_str())
        .to_string();
    let mut infile: String = std::fs::read_to_string(filpath).expect("Unable to read file");
    let tmp = file_name_prefix.to_string() + &change_filename(base_name.as_str(), output_name);
    let mut output_name = std::path::Path::new(tmp.as_str());
    let mut outfile = std::fs::File::create(output_name).expect("Unable to create file");
    let mut indentation_level: u32 = 0;
    let indentation_sign: &str = "    ";

    if add_true_line {
        outfile
            .write_all("true=True; false=False;\n".as_bytes())
            .unwrap();
    }
    for line in infile.lines().into_iter() {
        infile_str.push_str(line);
    }

    for u in infile_str_raw.split("\n") {
        let mut line: String = u.to_string();
        let comment_regex = regex::Regex::new(r#"[ \t]*(#.*$)"#).unwrap();
        let mut m = comment_regex.find(&line);
        let mut m2 = String::new();
        let mut add_comment = String::new();
        match m {
            Some(m) => {
                m2 = regex::Regex::new(r#"[\"'].*#.*[\"']"#)
                    .unwrap()
                    .find(m.as_str())
                    .unwrap()
                    .as_str()
                    .to_string();
            }
            None => m = None,
        }
        // if m != None {
        //     let add_comment = m.unwrap();
        // }

        match m {
            Some(m) => {
                add_comment = m.as_str().to_string();
            }
            None => add_comment = "".to_string(),
        }
        if line.is_empty() {
            infile_str_indented += &indentation_sign
                .repeat(usize::try_from(indentation_level).unwrap())
                .to_string();
            infile_str_indented += &add_comment.trim_start();
            infile_str_indented += "\n";
            continue;
        }
        line = line.trim_start().to_string();

        if line.contains("}") {
            indentation_level -= 1;
        }
        for _i in 0..usize::try_from(indentation_level).unwrap() {
            line = indentation_sign.to_string() + &line;
        }
        if line.contains("{") {
            indentation_level += 1;
        }
        line = line.replace("{", ":");
        line = line.replace("}", "");
        line = line.replace("\n:", ":");
        // line = Regex::new(r#"\n:", ":"#)
        //     .unwrap()
        //     .find(&line)
        //     .unwrap()
        //     .as_str()
        //     .to_string();
        infile_str_indented += &indentation_sign
            .repeat(usize::try_from(indentation_level).unwrap())
            .to_string();
        infile_str_indented += add_comment.trim_start();
        infile_str_indented += "\n";

        infile_str_indented += &line.replace("else if", "elif").to_string();
        infile_str_indented += &line.replace(";\n", "\n").to_string();
        if let Some(imports) = change_imports.clone() {
            for (_k, module) in imports.iter() {
                let _tmp = Regex::new(format!(r#"(import\s)[\w.]+("{module}")"#).as_str())
                    .unwrap()
                    .captures(module)
                    .unwrap();
                infile_str_indented = infile_str_indented.replace(
                    format!("import {module}").as_str(),
                    format!("{} as {}", module.as_str(), imports[module]).as_str(),
                );
            }
        }
    }
    outfile.write_all(infile_str_indented.as_bytes()).unwrap();
    Ok(())
}
