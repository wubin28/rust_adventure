use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let x = 5;
    let y = 3.0;
    let z = "Hello, World!";

    println!("The type of x is: {}", type_of(x)); // The type of x is: i32
    println!("The type of y is: {}", type_of(y)); // The type of y is: f64
    println!("The type of z is: {}", type_of(z)); // The type of z is: &str
}
