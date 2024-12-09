#[derive(Debug)]
enum Message{
    Quit,                       // 연관 데이터 없음
    Move{x:i32, y:i32},         // 구조체처럼 이름 있는 필드 있음
    Write(String),              // 하나의 String을 보유
    CahngeColor(i32,i32,i32)    // 세개의 i32를 가짐
}


impl Message{
    fn call(&self) -> String {
        let mes:String;
        match self{
            Message::Write(mes) => return mes.to_string(), // enum 베리언트 값을 match를 통해 반환이 가능하다
            others => return String::from("None")
        }
    }

}

fn first_practice(){
    println!("The First Practice");
    let m = Message::Write(String::from("hello"));
    println!("Message Value: {}", m.call());

    // ----------------------------------------------------
    // Option

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None; // Option<i32> 의 빈 변수 생성

    let x:i8 = 5;
    let y:Option<i8> = Some(5); // x[i8]와 y[Option<i8>]는 서로 다른 타입으로 덧셈 불가
    // 즉 Option<T>에서 T로 변환하기 위해서 변환 과정이 필요하다, T는 타입 변수
}
// 값의 존재 혹은 부재의 개념에 관한 열거형
// <T>는 제너릭 타입 매개변수 -> 10장
// enum Option<T>{
//     None,
//     Some(T)
// }

// ----------------------------------------------
// enum과 match 조합 예시

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents(Coin:Coin) -> u8 {
    match Coin {
        Coin::Penny => {
            println!("Lucky Penny!, 1!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn second_practice(){
    println!("The Second Practice");

    let coin1 = Coin::Penny;

    let value:u8 = value_in_cents(coin1);
    println!("The Value of Coin is {}", value);
}

// -----------------------------------------------

#[derive(Debug)]
enum UsState{
    Alabama,
    California,
    Utha,
    Alaska
}

enum Coin2{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents2(coin:Coin2) -> u8 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            // debug와 :?를 통해서 출력 필요, 안에 들어있는 UsState의 enum값은 변수로 사용 가능해진다.
            // 즉 enum의 값을 반환도 가능하다는 뜻
            println!("Coin2's {:?} Quarter value matching!", state);
            25
        },
    }

}

fn third_practice(){
    println!("The Third Practice");

    let coin2 = Coin2::Quarter(UsState::Alaska);
    let value: u8 = value_in_cents2(coin2);
    println!("The Value of Coin is {}", value);
}


// -----------------------------------------------
fn plus_one_1(x:Option<i32>) -> Option<i32> {
    match x {
        // match는 모든 입력이 가능한 범위에 대해서 정의가 필요 (i32와 None 둘 다)
        // 따라서 다른 예외상황이 발생하지 않도록 하는 것이 중요!
        Some(i) => Some(i+1),
        None => None
    }
}


fn forth_practice(){
    let five:Option<i32> = Some(5);
    let six = plus_one_1(five);
    let none = plus_one_1(None);

    println!("five:{:?}, six:{:?}, None:{:?}", five, six, none);

}

// -----------------------------------------------


fn fifth_practice(){

    let dice_roll:u8  = 9;
    match dice_roll {
        // 모든 정수형에 대해서 정의 불가능하다
        3 => add_fency_hat(),
        7 => remove_fency_hat(),
        o => move_player(o), // 나머지 값 처리
        // other 이름에 대해서 영향 X, 나머지 값을 처리할 변수가 필요할 뿐
        // 나머지 값에 대해서 사용 안할때, 포괄 패턴 "_" 를 통해서 매칭이 가능하다.
    }

    match dice_roll {
        // 모든 정수형에 대해서 정의 불가능하다
        3 => add_fency_hat(),
        7 => remove_fency_hat(),
        _ => reroll() // 포괄패턴 사용
        // 나머지 값에 대해서 사용 안할때, 포괄 패턴 "_" 를 통해서 매칭이 가능하다.
        // 아무런 작동을 하고 싶지 않다면, () 만 사용 가능하다.
    }

    match dice_roll {
        // 모든 정수형에 대해서 정의 불가능하다
        3 => add_fency_hat(),
        7 => remove_fency_hat(),
        _ => () // 나머지 값 무시
        // 아무런 작동을 하고 싶지 않다면, () 만 사용 가능하다.
    }

    println!("The Fifth practice done!");

    fn add_fency_hat(){}
    fn remove_fency_hat(){}
    fn move_player(num_space:u8){}
    fn reroll(){}

}
// -----------------------------------------------
fn main() {
    // 돌아가는 코드인지 체크
    first_practice();
    second_practice();
    third_practice();
    forth_practice();
    fifth_practice();
}

