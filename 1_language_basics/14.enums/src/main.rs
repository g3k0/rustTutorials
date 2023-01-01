enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: four,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: six,
        address: String::from("::1"),
    };

    route(four);
    route(six);


    // a more convenient way to handle this logic
    enum IpAddr {
        V4(u8, u8, u8, u8), // in enums types can be different
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // -------------------
    // This code illustrates that you can put any kind of data inside an enum variant
    struct Ipv4Addr {
        // --snip--
    }
    
    struct Ipv6Addr {
        // --snip--
    }
    
    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }
    // ---------------------------------
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}

fn route(ip_kind: IpAddrKind) {

}

