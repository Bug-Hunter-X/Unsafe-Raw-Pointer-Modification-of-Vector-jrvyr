fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 10; // Safe and standard way to modify the vector
    println!("v: {:?}", v);
}