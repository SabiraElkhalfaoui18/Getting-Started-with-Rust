// Collection Types Vectors vec<T>
//
//fn main() {
    // let _v : Vec<i32> = Vec::new();
    // // Mcro to create a vector of numbers
    // let mut _v : Vec<i32> = vec![1,2,3];
    // _v.push(5);
    // _v.push(6);
    // _v.push(7);
    // println!("The numbers vector is {:?}", _v);

//     let _v: Vec<i32> = vec![1,2,3,4,5];
//     let _third : &i32 = &_v[2]; //Direct indexing

//     println!("The third element is : {_third}");

//     let third: Option<&i32> = _v.get(5);
//     match third {
//         Some(third) => println!("The third element is {third}"),
//         None => println!("There is no third element."),
//     }

// }

fn main() {
    // 1
    //let s : String = "whatever".to_string();
    //2 
    //let s : String = String::from("whatever");
    // Mutate the variable [push to it]
    let mut s : String = String::from("foo");
    s.push_str("bar");
    s.push('!');
    println!("the value of s = {}",s);

    let salam: String = String::from("السلام عليكم");
    let salut: String = String::from("Salut");

    // If you want to combine strings, use the + operator
    let s1: String = String::from("Hello, ");
    let s2: String = String::from("world!");
    let s3: String = s1 + &s2; 
    println!("The value of s3 = {}",s3);

    // Formatting Strings
    let full_message = format!("{salam} {salut} ");
    println!("{full_message}");
}