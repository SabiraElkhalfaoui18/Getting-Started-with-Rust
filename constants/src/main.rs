// Constants are bound to a name and they are not allowed to change 
// diffrences between variables and constants
//Both are immutable by default, Variables can be mutable by adding the keyword
// mut but constants no 
fn main() {
    println!("Hello, world!");
    let mut x = 5;
    const Y: i32  = 10;
    println!("the value of the variable is {} and the constant is {}", x, Y);
    println!("The value of pi is {}", PI);
    println!("three hours in second is {}",THREE_HOURS_IN_SECONDS);
}

// You can declare a constant here with a type annotation
const PI: f32 = 3.1415922653;
const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;

