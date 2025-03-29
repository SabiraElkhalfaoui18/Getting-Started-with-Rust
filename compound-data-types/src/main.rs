//Compound Data Types
// arrays, tuples, slices, and strings (slice string)


fn main() {
//Arrays: contains a list of homogenous elements
    let numbers: [i32;5] = [1,2,3,4,5];
    println!("Number Array {:?}", numbers);
    let fruits :[&str;3] = ["Apple", "Banana","Orage"];
    println!("Fuits Array: {:?}", fruits);
    println!("Fuits Array: {:?}", fruits[2]);

// Tuples contains heterogenous collection of elements of fixed size
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple {:?}", human);

    let my_mix_tuple = ("coco".to_string(), 23, true, [1,2,3,4,5]);
    println!("My mix tuple is {:?}", my_mix_tuple);

// Slices: [1,2,3,4,5]    
    let number_slices:&[i32] = &[1,2,3,4,5];
    println!("Number Slices {:?}", number_slices);
    let animal_slices: &[&str] = &["lion", "Elephant", "Crocodile"];
    let book_slices :&[&String] = &[&"IT".to_string(), &"Harry Potter".to_string()];
    println!("Animal Slices: {:?}", animal_slices);
    println!("Book Slices: {:?}", book_slices);

//String VS string slices (&str)  
    let mut stone_cold: String = String::from("hell, ");
    println!("Stone Cold says: {}", stone_cold);
    stone_cold.push_str("Yeah!");
    println!("Stone Cold says: {}", stone_cold);
    
// B- &str (String Slice), which is a reference 

     let string : String = String::from("Hello World");
     let slice: &str = &string[0..5];
     println!("Slice Value: {}", slice);


}
// In following function we cannot find the value of slice "cannot find value"
//"slice" in this scope
//fn print(){
//    println!("Slice:{}", slice);
//}