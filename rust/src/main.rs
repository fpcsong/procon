#![allow(non_snake_case)]

fn read_line() -> Vec<String> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace().map(|s| s.to_owned()).collect()
}

fn main() {
    println!("{}", 0)
}
