//Error Handling techniques [2 approaches]

// fn main() {
//     // Approach 1 OPTION<T>
//     enum Option<T>{ // Define the generic Option type
//         Some(T), // Represents a value
//         None, // Represents no value (the absence of  value)

//     }
//     // Approach 2
//     enum Result<T,E>{ // Define the generic Result type
//         Ok(T), // Represents a value
//         Err(E), // Represents an error
//     }


//     println!("Hello, world!");
// }

// Approach  1


    // Approach 1
    // enum Option<T>{ // Define the generic Option type
    //     Some(T), // Represents a value
    //     None,// Represents no value
    // }
    
    // fn divide(numerator: f64, denominator: f64) -> Option<f64>{
    //     if denominator == 0.0 {
    //         None
    //     } else {
    //         Some(numerator/denominator)
    //     }
    // }    
    
    // fn main() {
    //     let result = divide(10.0,2.0);
    //     match result{
    //         Some(x) => println!("Result: {}",x),
    //         None => println!("Cannot divide by Zero!"),
    //     }
    // }

// Approach 2: example 
fn divideresult(numerator: f64, denominator: f64) -> Result<f64, String>{
    if denominator == 0.0 {
         Err("Cannot divide by 0".to_string())
    } else {
        Ok(numerator/denominator)
    }
}   
fn main(){
    match divideresult(100.23, 73.98){
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }

}