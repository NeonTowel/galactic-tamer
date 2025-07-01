use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let name = args
        .iter()
        .position(|arg| arg == "-name")
        .and_then(|i| args.get(i + 1))
        .map(|name| name.as_str())
        .unwrap_or("world");

    println!("Hello, {}!", name);
}
