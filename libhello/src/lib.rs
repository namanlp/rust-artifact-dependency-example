pub fn add(left: usize, right: usize) -> usize {
    unsafe {say_hello();}
    unsafe {adding_num(left, right)}
}

unsafe extern "C" {
    fn say_hello();
    pub fn adding_num(a:usize, b:usize) -> usize;
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
