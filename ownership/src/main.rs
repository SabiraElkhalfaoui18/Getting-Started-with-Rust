// Ownership, Borrowing and References
// Ownership
//------------------------------------------------------
// The variable s1 owns the String "Rust"
// fn main() {
//     let s1 = String::from("Rust"); 
//     let len = calculate_length(&s1); // Passing a reference, not ownership
//     println!("Length of '{}' is {}", s1, len);
// }
// fn calculate_length(s: &String) -> usize {// unsigned size
//     s.len()

// }
//--------------------------------------------------------
//One owner at a time.

// fn main(){
//     let s1 = String::from("Rust");
//     let s2 = s1; // Ownership of s1 is transferred to s2
//     println!("{}",s2);
// }

//When a variable goes out of scope, Rust automatically deallocates the memory.
fn main(){
    let s1 = String::from ("Rust");
    let len = calculate_length(&s1);  // This will cause an error if don't use &s1
    println!("Length of '{}' is {}", s1, len);
}
// s1 goes out of scope and its value will be dropped
fn calculate_length(s: &String) -> usize {// unsigned size
    s.len()
    
}