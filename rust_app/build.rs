
// Print message as a warning
macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo::warning={}", format!($($tokens)*))
    }
}
fn main() {
    // List the CARGO_BIN files found
    for (key, value) in std::env::vars() {
        if key.starts_with("CARGO_BIN") {
            p!("{key} : {value}");
        }
    }

    // Execute the `Side` binary file, works fine
    let side_bin = std::env::var_os("CARGO_BIN_FILE_LIBHELLO_side").unwrap();
    let status = std::process::Command::new(side_bin).status().unwrap();

    if !status.success() {
        eprintln!("failed!");
        std::process::exit(1);
    } else {
        p!("OK");
    }
}

