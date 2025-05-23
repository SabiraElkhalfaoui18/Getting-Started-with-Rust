// Structs are used to name and package together related values similar to tuples
//
#![allow(warnings)]
fn main() {
    // tuple
    let rect : (i32,i32) = (200,500);// this is the rectangle tuple that contains the width and height

    //Struct
    struct Book{
        title: String,
        author: String,
        pages: i32,
        available: bool,

    }
    struct User{
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    
    let mut user1: User = User{
        active: true,
        username: String::from ("someusername"),
        email: String::from ("someusername@m.com"),
        sign_in_count: 1,
    };
    //user1.email = String::from("anotheremail@m.com");
    println!("User email {}", user1.email);

    // Return a struct from a function
    fn build_user(email: String, username: String) -> User{
        User{
            active: true,
            email,
            username,
            sign_in_count: 1,
        }
    }
    // Create instances from other instances
    let user2 : User = User{
        email: String::from("another@m.com"),
        ..user1
    };
    // Tuple Structs
    struct Color(i32,i32,i32);
    struct Point(i32,i32,i32);
    let black: Color = Color(0,0,0);
    let white: Color = Color(255,255,255);

    // Unit-like Struct
    struct AlwaysEqual;
    let subject: AlwaysEqual = AlwaysEqual;



}
