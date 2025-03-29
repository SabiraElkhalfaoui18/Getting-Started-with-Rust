// Shadowing
// In Rust book about shadowing: you can declare a new variable with the same name as a previous variable
// the first variable is shadowed by the second one
// Shadowing is not the same as marking a variable as mut.


fn main() {
    let x = 5;// result is 5
    let x = x + 1;// result is 6
    {
        let x = x * 2;// result 12
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x in the main function is: {x}");
}
