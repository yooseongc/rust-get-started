fn main() {
    
    // structs give you a way of grouping together related fields and data,
    // enums give you a away of saying a value is one of a possible set of values.
    
    // let _four = IpAddrKind::V4;
    // let _six = IpAddrKind::V6;

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // let  home = IpAddr::V4(String::from("127.0.0.1"));
    // let home = IpAddr::V4(127, 0, 0, 1);
    // let loopback = IpAddr::V6(String::from("::1"));

    // let m = Message::Write(String::from("hello"));
    // m.call();

    let some_number = Some(5);
    let some_char = Some('e');
    
    let absent_number: Option<i32> = None;
    

}

// enum IpAddrKind {
//     V4,
//     V6,
// }

// fn route(ip_kind: IpAddrKind) {

// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

enum IpAddr {
    // V4(String),
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // do something
    }

}