fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let ptr = vec.as_ptr();

    // This is unsafe because the vector's length is 3, but we're accessing
    // the 4th element.
    unsafe {
        println!("Value at index 3: {}", *ptr.add(3));
    }
}