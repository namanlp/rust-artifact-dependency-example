fn main() {
    let status = std::process::Command::new(env!("CARGO_BIN_FILE_LIBHELLO_side")).status().unwrap();

    if !status.success() {
        eprintln!("failed!");
        std::process::exit(1);
    } else {
        println!("OK");
    }
}
