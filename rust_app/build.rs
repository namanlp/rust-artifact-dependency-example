
// Print message as a warning
macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo::warning={}", format!($($tokens)*))
    }
}
fn main() {
    let side_bin = std::env::var_os("CARGO_BIN_FILE_LIBHELLO_side").unwrap();

    p!("HELLO FROM BUILD SCRIPT FILE");
    p!("{:?}", side_bin);

    let status = std::process::Command::new(side_bin).status().unwrap();

    p!("Hello status {status}");//, status.code().unwrap());

    if !status.success() {
        eprintln!("failed!");
        std::process::exit(1);
    } else {
        p!("OK");
    }

    // p!("side_bin = {:?}", side_bin);

    // Warn with all the


    for (key, value) in std::env::vars() {
        if key.starts_with("CARGO_BIN") {
            p!("{key} : {value}");
        }
    }
}

