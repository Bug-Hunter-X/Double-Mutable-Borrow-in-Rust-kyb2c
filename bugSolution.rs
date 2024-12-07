fn main() {
    let mut x = 5;
    { // Limiting the scope of the mutable borrow
        let y = &mut x; 
        *y = 10; 
    }
    let z = &x; //z is immutable now
    println!("{}, {}", x, *z);
}