// Enum is a versatile tool used to represent a type that can take on one of several possible variants
//

fn main() {
    // enum IpAddKind{
    //     V4,
    //     V6,
    // }

    // let _four: IpAddKind =  IpAddKind::V4;
    // let _six: IpAddKind =  IpAddKind::V6;

    // fn route( _ip_kind: IpAddKind){}

    // route(IpAddKind::V4);
    // route(IpAddKind::V6);

    // Using structs

    // struct IpAddr{
    //     kind: IpAddKind,
    //     address: String,
    // }

    // let home: IpAddr = IpAddr{
    //     kind: IpAddKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    // let loopback: IpAddr = IpAddr{
    //     kind: IpAddKind::V6,
    //     address: String::from ("::1"),

    // };
    // Using Enums
    enum IpAddr{
        V4(u8,u8,u8,u8),
        V6(String),
    }
    let _home: IpAddr = IpAddr::V4(127,0,0,1);
    let _loopback: IpAddr = IpAddr::V6(String::from("::1"));
}
