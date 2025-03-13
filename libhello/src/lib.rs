pub fn add(left: u64, right: u64) -> u64 {
    unsafe {say_hello();}
    left + right
}

unsafe extern "C" {
    fn say_hello();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
