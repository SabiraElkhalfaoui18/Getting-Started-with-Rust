// Control Flow in Rust
// Conditions [If ... Else]
// Repeating actions. [Loops]
// If Else [If expression] [Else expression]


fn main() {
    // let age: u16 = 15;
    // if age >= 18 {
    //     println!("You can drive  a car!");
    // } else {
    //     println!("You can't drive a car!");
    
    // }
    // Mutiple conditions with else if:
    // let number = 9;
    // if number % 4 == 0{
    //     println!("number is divisible by 4");   
    // } else if number % 3 ==0 {
    //     println!("number is divisble by 3");
    // } else if number % 2 == 0 {
    //     println!(" number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }

    //Using if in a let statement
    let condition = true;
    let number = if condition {5} else {6};
    println!("Number: {number}");
    

}
