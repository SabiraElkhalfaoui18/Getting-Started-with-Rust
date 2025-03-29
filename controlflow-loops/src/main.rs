// Rust provides three types of loops:
//Loop: it is an uncoditional loop, it continues runnig until we stop it!!
// while
// for 


fn main() {
    //loop keyword
    // loop{
    //     println!("Hello, world!");
    //     //break;
    // }
// loop 
    // let mut counter = 0;

    // let result = loop{
    //     counter +=1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {result}");
// A loop label in Rust is a way to name a loop so you can explicitly 
//reference it, often used with nested loops to control which loop to 
//break or continue.
    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {count}");
// While loop
    // let mut number = 3;
    // while number !=0 {
    //     println!("{number}");
    //     number -=1;
    //     //break;
    // }
    // println!("Hey");
// # Looping Through a collection with for loop
    let a = [1,2,3,4,5,6];
    let b = ["a","b","c","d","e"];

    for element in a {
        println!("{element}");
    }
    for letter in b {
        println!("{letter}");
    }
}
