fn main() {
    let a = Box::new(5);
    let b = 5;
    assert_eq!(*a, b);
}