fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let x = vec[0]; // Copying the value instead of borrowing
    vec.push(3);
    println!("{}", x); 
}

//Alternative using cloning
fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let x = vec[0].clone(); // Cloning value
    vec.push(3);
    println!("{}", x); 
}