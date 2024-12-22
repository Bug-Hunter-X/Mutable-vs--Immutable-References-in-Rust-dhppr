fn main() {
    let mut x = 5;
    { // Limit the scope of the mutable reference
        let y = &mut x;
        *y += 1; //Modify x through y
    }
    let z = &x; // Now it's safe to create an immutable reference
    println!("x = {}", x); // x is now 6
    println!("z = {}", *z); // This is now allowed
} 