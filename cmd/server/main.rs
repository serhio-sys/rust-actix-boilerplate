use core::panic;

use config::logger::init_logger;
use internal::{ container::container::new, infra::{ database::migration::migrate, http::server } };

pub fn check_inclusion(mut s1: String, s2: String) -> bool {
    s1 = sort(s1);
    for i in 0..s2.len() - s1.len() + 1 {
        if s1.eq(&sort(s2[i..i + s1.len()].to_string())) {
            return true;
        }
    }
    return false;
}

pub fn sort(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    return String::from_iter(chars);
}

#[actix_web::main]
async fn main() {
    init_logger();

    if let Err(e) = migrate() {
        panic!("{}", e.to_string());
    }

    match new() {
        Ok(container) => {
            match server::start_server(container).await {
                Ok(res) => res,
                Err(e) => panic!("{}", e.to_string()),
            }
        }
        Err(e) => panic!("{}", e.to_string()),
    }
}
