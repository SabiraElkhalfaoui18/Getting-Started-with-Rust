fn main() {
    // 1
    //let s : String = "whatever".to_string();
    //2 
    //let s : String = String::from("whatever");
    // Mutate the variable [push to it]
    let mut s : String = String::from("foo");
    s.push_str("bar");
    //s.push('!')
    println!("the value of s = {}",s);
}