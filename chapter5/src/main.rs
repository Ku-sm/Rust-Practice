struct User {
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64
} // 마지막 세미콜론 없음


fn struct_practice() {
    let user1 = User{
        active:true,
        username:String::from("SomeUserName1234"), // &str과 String을 통한 소유권 차이 존재
        email:String::from("SomeUserEmail1111"),
        sign_in_count:1
    };

    let mut user2 = User{
        active:true,
        username:String::from("SomeUserName1234"),
        email:String::from("SomeUserEmail1111"),
        sign_in_count:1
    };

    user2.email = String::from("NewUserEmail2222");

    let user3 = build_user(String::from("User3Email"), String::from("User3name"));

    let user4 = User{
        email:String::from("User4Email"),
        ..user1
    }; // user1의 값으로 새로운 user4를 생성, 단 user1의 정보는 소유권으로 인해 사라짐

    print!("User3's information: {}\n", user3.username);
    // print!("User1's information: {}", user1.username); // value borrowed here after move 에러 발생
    print!("User4's information: {}\n", user4.username);
}

fn build_user(email:String, username:String) -> User {
    return User{
        active:true,
        username:username, // 필드와 입력 표기
        email, // 필드명과 입력 변수명이 같으면 생략 가능하다 (축약형)
        sign_in_count:1
    };
}



#[derive(Debug)]
struct Color(i32,i32,i32); // 튜플 구조체, 구조체 안 변수명 없음
struct Point(i32,i32,i32);

fn tuple_struct_practice() {
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    print!("Color info: {:?}", black);
}


// 필드 없는 유사 구조체
struct AlwaysEqual;
fn no_field_struct_practice(){
    let subject = AlwaysEqual;
    print!("No Field Struct Practice Run!");
}


fn main() {
    // struct_practice();
    // tuple_struct_practice();
    no_field_struct_practice();
}