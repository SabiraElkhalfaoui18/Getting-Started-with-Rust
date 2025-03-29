// Primitive data types

fn main() {
    let x: i32 = -50;
    let y: u64 = 10090;
    println!("Signed integer {}", x);
    println!("Unsigned integer {}", y);

// diff bet i32 (32 bits) and i64(64 bits)
// range
    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;
    println!("Maximum value of i32: {}", e);
    println!("Maximum value of i64 {}", i);

// floats  [Floating Point Types]
// f32, f64
    let pi: f64 = 3.14;
    println!("The value of pi is {}", pi);
// Boolean values: true, false
    let value : bool = true;
    println!("The Value is {}", value);
    let is_raining: bool = true;
println!("Is it raining? {}", is_raining);
// Charachter Type - char
    let letter : char = 'z';
    println!("the last letter of the alphabet: {}", letter);
}
