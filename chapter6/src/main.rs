// chapter 6
// Enum & pattern matching
// 열거형(enumeration): 하나의 타입이 가질 수 있는 배리언트(variant)들을 열거해서 타입을 정의
// Option: 값이 있을 수 있고, 없을 수 있는 상태를 의미
// 열거형을 통한 match 표현식 및 if let 구문



fn first_way(){

    #[derive(Debug)]
    enum IpAddrKind{ // 열거형의 값은 베리언트의 값 중 하나만 된다.
        V4,
        V6
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String
    }

    // enum 선언 예시
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("Ip Kinds: {:?}, {:?}", four, six);

    // enum을 포함한 구조체 예시 1
    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    // enum을 포함한 구조체 예시 2
    // let loopback = IpAddr{
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1")
    // };

    println!("Home Ip kind: {:?}, addr: {}",home.kind, home.address );
}


fn second_way(){
    // 각 열거형 베리언트에 데이터 직접 넣는 방법
    #[derive(Debug)]
    enum IpAddr{
        V4(String), // 베리언트에 타입 선언이 가능하듯이, 구조체도 가능하다.
        V6(String)
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home Ip kind: {:?}", home );
    println!("Home Ip kind: {:?}", loopback );
}

fn third_way(){

    #[derive(Debug)]
    enum IpAddr{ // 열거형 값 넣을때, 각 베리언트마다 다른 타입과, 양을 정할 수 있다.
        V4(u8,u8,u8,u8),
        V6(String)
    }

    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home Ip kind: {:?}", home );
    println!("Home Ip kind: {:?}", loopback );
}


fn main() {
    println!("The First");
    first_way();

    println!("The Second");
    second_way();

    println!("The Third");
    third_way();
}
