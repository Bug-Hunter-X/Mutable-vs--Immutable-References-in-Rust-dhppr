fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x; // z is an immutable reference to x

    *y += 1; // Modify x through y
    println!("x = {}", x); // x is now 6

    // This line will cause a compile-time error:
    // println!("z = {}", *z); // Cannot dereference z as it's immutable 
}