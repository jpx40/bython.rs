use regex::Regex;
#[inline]
fn ends_with(word: &str, suffix: &str) -> bool {
    word.ends_with(suffix)
}

fn change_filename(name: &str, output_name: Option<&str>) -> String {
    if let Some(output_name) = output_name {
        return output_name.to_string();
    } else if ends_with(name, ".by") {
        return name.to_string().replace(".by", ".py");
    } else {
        return name.to_string() + ".py";
    }
}

fn parse_imports(file_name: &str) -> Vec<String> {
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

    import_with_suffix
}
