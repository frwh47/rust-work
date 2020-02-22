#[test]
fn f3(){
    assert!(true);
    assert_eq!(1, 1);
}


#[test]
fn f1(){
    assert!(true);
    assert_eq!(1, 1);
}


pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::add_two;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }
}