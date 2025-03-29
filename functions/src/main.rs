//Functions
fn main() {
    hello_rust();
    rectangle_height(12.0);
    rectangle_width(5.0);
    println!("The result is {:.2}", devision(30.5,13.2));
    let x: f32 = 6.0;
    let y: f32 = 3.0;
    let area = rectangle_area(x,y);
    println!("Rectangle area is {}", area);
    student_id("Bob".to_string(), 19, 60);

    let x = {
        let h : u32 = 10;
        let w : u32 = 12;
        h*w
    };
        println!("Result is {}", x);

    let m = multiply(2,-4);
    println!("The results is {}", m);
    println!("The results is {}", multiply(6,4));
}
// Hoisting, you can call this function anywhere
fn hello_rust(){
    println!("Hello, Rust");

}
// Functions with an input value 
fn rectangle_height(height: f32){
    println!("The rectangle height is {} ", height);
}

fn rectangle_width(width: f32){
    println!("The rectangle width is {} ", width);
}
// Function with many insputs
fn student_id(name: String, age: u32, grade: u32) {
    println!("The student name is {}, his age is {}, his grade is {}", name, age, grade);

}
// Functions returning values
fn multiply(a: i32, b: i32) ->i32 {
    a*b
}

fn rectangle_area(height: f32, width: f32) -> f32{
    height*width
}

// last example
fn devision(a: f32,b: f32) ->f32{
    a / b
}