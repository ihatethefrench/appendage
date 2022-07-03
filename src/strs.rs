use std::time::UNIX_EPOCH;

pub fn info(str: &str) {
    println!("\x1b[2;22;33ma\x1b[0m \x1b[1;37m{}\x1b[0m", str);
}

pub fn error(str: &str) {
    eprintln!("\x1b[2;22;31me:\x1b[0m \x1b[1;91m{}\x1b[0m", str);
}

pub fn debug(str: &str) {
    eprintln!(
        "{} {}",
        std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        str
    );
}
