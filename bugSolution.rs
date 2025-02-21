fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Safe way to access elements within the vector's bounds.
    if let Some(&value) = vec.get(2) {
        println!("Value at index 2: {}", value);
    } else {
        println!("Index out of bounds");
    }
} 