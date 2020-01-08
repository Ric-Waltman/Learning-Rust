enum IpAddrVersion { V4, V6 }

// This enum definition includes associated data types (like bundling in a struct)
enum IpAddr {
    V4(String),
    V6(String)
}

// Data types do not have to match between enum variants. Even different struct
// types can be used for each variant
enum IpAddrImproved {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum ComplexEnum {
    Quit,                       // no data type
    Move { x: i32, y: i32 },    // anonymous struct
    Write(String),              // String
    ChangeColor(i32, i32, i32), // tuple
}

impl ComplexEnum {
    fn call(&self) {
        println!("ComplexEnum call: {:#?}", self);
    }
}

fn main() {
    let _four = IpAddrVersion::V4;
    let _home = IpAddr::V4(String::from("127.0.0.1"));
    let _home2 = IpAddrImproved::V4(127, 0, 0, 1);
    let c = ComplexEnum::Write(String::from("a string for you"));
    c.call();
}

fn route(ip_version: IpAddrVersion) {}
