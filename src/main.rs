use reqwest::blocking::*;

fn main() {
    println!("Hello, world!");

    let mut buff = [0; 8];
    getrandom::getrandom(&mut buff).unwrap();
    println!("{:?}", buff);

    let body = get("https://www.rust-lang.org").unwrap().text().unwrap();

    #[cfg(debug_assertions)]
    println!("Debug mode");

    #[cfg(not(debug_assertions))]
    println!("Release mode");

    println!("body = {:#?}", body.lines().collect::<Vec<_>>()[0..10].to_vec());

    println!("Goodbye, world!")
}
