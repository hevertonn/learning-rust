#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrEnum {
    V4(String),
    V6(String),
}

fn main() {
    let my_ip = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("10.0.0.1"),
    };

    let my_room_ip = IpAddrEnum::V4(String::from("192.168.0.1"));
    let my_second_room_ip = IpAddrEnum::V6(String::from("10.0.0.0"));

    println!("{:?}", my_room_ip);
}
