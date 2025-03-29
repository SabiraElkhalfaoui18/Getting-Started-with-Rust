#[allow(dead_code)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let _home: IpAddr = IpAddr::V4(127, 0, 0, 1); // ✅ Correct
    let _loopback: IpAddr = IpAddr::V6(String::from("::1")); // ✅ Correct

    println!("IP Address created!");
}