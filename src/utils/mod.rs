use std;

pub fn home_dir() -> String {
    match std::env::var("HOME") {
        Ok(val) => String::from(val),
        _ => String::new(),
    }
}
