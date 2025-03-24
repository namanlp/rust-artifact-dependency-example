fn main() {
    println!("Printing works!");
    // Try to find CARGO_BIN env variables, but can't find any
    for (key, value) in std::env::vars() {
        if key.starts_with("CARGO_BIN") {
            println!("{key}: {value}");
        }
    }
    let status = std::process::Command::new(env!("CARGO_BIN_FILE_LIBHELLO_side")).status().unwrap();

    if !status.success() {
        eprintln!("failed!");
        std::process::exit(1);
    } else {
        println!("OK");
    }
}
