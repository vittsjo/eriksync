use std;

pub fn home_dir() -> String {
    String::from(std::env::var("HOME").unwrap_or_default())
}

pub fn expand_user(path: &std::path::Path) -> std::path::PathBuf {
    let mut path_str = String::from(path.to_str().unwrap_or_default());

    if path_str.len() > 0 && path_str.chars().nth(0).unwrap() == '~' {
        path_str = path_str.replacen("~", &home_dir(), 1);
    }

    std::path::PathBuf::from(&path_str)
}
