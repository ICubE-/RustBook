fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;
    
    fn route(_ip_kind: IpAddrKind) {}
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }
    
    // let _home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let _loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    enum IpAddr {
        V4(String),
        V6(String),
    }

    let _home = IpAddr::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr::V6(String::from("::1"));

    enum Message {
        _Quit,
        _Move { x: i32, y: i32 },
        Write(String),
        _ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // method body
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    let _some_number = Some(5);
    let _some_char = Some('e');

    let _absent_number: Option<i32> = None;
}
