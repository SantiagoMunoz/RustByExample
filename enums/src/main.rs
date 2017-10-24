enum IpAddrKind{
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let ip1 = IpAddrKind::V4(127, 0, 0, 1);
    let ip2 = IpAddrKind::V6(String::from("::1"));
}
