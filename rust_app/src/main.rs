use libhello::add;

// include!(env!("CARGO_BIN_FILE_LIBHELLO_side"));


fn main() {
    println!("2 + 2 = {}", add(2, 2));

    // let side_bin = std::env::var_os("CARGO_BIN_FILE_LIBHELLO_SIDE_side_bin").unwrap();
    // println!("side_bin = {}", side_bin.to_str().unwrap());

    for (key, value) in std::env::vars() {
        if key.starts_with("CARGO_BIN") {
            eprintln!("{key}: {value}");
        }
    }
}
