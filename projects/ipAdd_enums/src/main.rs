fn main() {
    let mut x = 42;
    let y = &mut x; // y is a mutable reference to x
    
    // Dereference y to get a mutable reference to x and change its value
    y = 24;
    
    println!("Value of x: {}", x); // x has been changed to 24
}
