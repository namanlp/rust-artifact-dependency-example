fn main() {
    println!("Printing works!");
    // Try to find CARGO_BIN env variables, but can't find any
    for (key, value) in std::env::vars() {
        if key.starts_with("CARGO_BIN") {
            println!("{key}: {value}");
        }
    }
}
