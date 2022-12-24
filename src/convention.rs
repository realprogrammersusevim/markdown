use crate::file_management;
use regex::Regex;

pub fn convention(from: String, to: String, path: String) -> () {
    // TODO: Make sure I'm only renaming the file name, not the whole path
    // TODO: Make this whole thing safer with better error managment, maybe look into Anyhow for
    // error handling
    let files = file_management::get_files(path);
    for file in files {
        let split_file = split_filename(&file, &from);
        let new_file = join_filename(split_file, &to);
        file_management::rename_file(file, new_file).unwrap();
    }
}

/// Split the filename into the words
// I have no clue why the "'a" is needed. Something about static lifetimes
fn split_filename<'a>(file_name: &'a str, convention: &'a str) -> Vec<&'a str> {
    match convention {
        "camel" => camel_case_split(file_name),
        "snake" => snake_case_split(file_name),
        "kebab" => kebab_case_split(file_name),
        "space" => space_case_split(file_name),
        _ => panic!("Unknown convention"),
    }
}

fn join_filename<'a>(filename: Vec<&'a str>, convention: &'a str) -> String {
    match convention {
        "camel" => camel_case_join(filename),
        "snake" => snake_case_join(filename),
        "kebab" => kebab_case_join(filename),
        "space" => space_case_join(filename),
        _ => panic!("Unknown convention"),
    }
}

fn camel_case_split(file_name: &str) -> Vec<&str> {
    let re = Regex::new(r"[a-z]+|[A-Z][^A-Z]*").unwrap();
    re.find_iter(file_name).map(|m| m.as_str()).collect()
}

fn camel_case_join(filename: Vec<&str>) -> String {
    let mut result = String::new();
    for (i, word) in filename.iter().enumerate() {
        if i == 0 {
            result.push_str(word);
        } else {
            let first_char = word.chars().next().unwrap();
            let rest: String = word.chars().skip(1).collect();
            let joined = first_char.to_uppercase().to_string() + &rest;
            result.push_str(&joined);
        }
    }
    result
}

fn snake_case_split(file_name: &str) -> Vec<&str> {
    let re = Regex::new(r"[^_]+").unwrap();
    re.find_iter(file_name).map(|m| m.as_str()).collect()
}

fn snake_case_join(filename: Vec<&str>) -> String {
    let mut result = String::new();
    for (i, word) in filename.iter().enumerate() {
        if i == 0 {
            result.push_str(word);
        } else {
            result.push_str("_");
            result.push_str(word);
        }
    }
    result
}

fn kebab_case_split(file_name: &str) -> Vec<&str> {
    let re = Regex::new(r"[^-]+").unwrap();
    re.find_iter(file_name).map(|m| m.as_str()).collect()
}

fn kebab_case_join(file_name: Vec<&str>) -> String {
    let mut result = String::new();
    for (i, word) in file_name.iter().enumerate() {
        if i == 0 {
            result.push_str(word);
        } else {
            result.push_str("-");
            result.push_str(word);
        }
    }
    result
}

fn space_case_split(file_name: &str) -> Vec<&str> {
    let re = Regex::new(r"[^ ]+").unwrap();
    re.find_iter(file_name).map(|m| m.as_str()).collect()
}

fn space_case_join(file_name: Vec<&str>) -> String {
    let mut result = String::new();
    for (i, word) in file_name.iter().enumerate() {
        if i == 0 {
            result.push_str(word);
        } else {
            result.push_str(" ");
            result.push_str(word);
        }
    }
    result
}
