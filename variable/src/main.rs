
// 변수의 불변성으로 인해서 커파일 오류 발생
// fn main() {
//     let mut x = 5;
//     println!("The value of x is {x}");
//     x = 6;
//     println!("The value of x is {x}");
// }

// 변수의 변화가 존재함을 mut로 명시
fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    // 상수 선언
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // 섀도잉
    let y=5;
    let y = y+1;
    { //스코프 내부
        let y = y*2;
        println!("The value of y in the inner scope is: {y}"); //출력 12
    }
    println!("The value of y is: {y}"); // 출력 6
    // 섀도잉은 변수를 mut로 표시하는 것과 다르다.
    // 실수로 let 없이 변수에 값을 재할당하면 오류가 발생한다.
    // let을 사용하면, 값을 변형하면서 변형이 완료 된 후에는 불변으로 유지된다.
    // mut과 섀도잉의 다른 차이점은, 섀도잉은 새로운 타입으로 변환이 가능하다.

    let spaces = "   ";
    let spaces = spaces.len(); // 오류 발생 안함

    // let mut spaces = "   ";
    // spaces = spaces.len(); // 오류 발생
    println!("The value of spaces is: {spaces}");

    // 데이터 타입
    let guess:u32 = "38".parse().expect("not a number");
    println!("The value of guess is {guess}");

    // let guess = "38".parse().expect("not a number"); // 타입 지정 필요하다고 컴파일 오류 발생
    // println!("The value of guess is {guess}");
}
