fn main() {
    println!("Hello, world!");

    another_function();
    another_function2(5); //러스트 기본 정수형은 i32
    print_labeled_measurement(5, 'm');

    example_function();

    let x = five();
    let x = plus_one(x);
    println!("The value of x is: {}", x);

}


// main 함수 뒤에 작성해도 동작됨, 앞에서 정의해도 동작함
fn another_function() {
    println!("Another function.");
}

// 함수의 각 매개변수는 반드시 타입을 명시해야함
fn another_function2( x:i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value:i32, uint_label:char){
    println!("The measurement is: {value}{uint_label}");
}


// 구문과 표현식
// 구문은 값을 반환하지 않는다.
// 표현식은 값을 반환한다.

// 예시
fn example_function() {
    let y = 6; // 구문

    // let x = (let y=6); // 컴파일 에러, 구문은 결과값 반환하지 않기 때문에 x에 할당할 수 없다.

    let y = { // 스코프 블록도 표현식이다.
        let x = 3;
        x + 1 // 표현식
    };
    // x+1의 마지막 세미콜론으로 끝나지 않는다. 이는 표현식은 종결을 나타내는 세미콜론을 사용하지 않는다.
    // 표현식 끝에 세밐콜론을 붙이면 구문이 되어 값을 반환하지 않는다.

    println!("The value of y is: {}", y);
}


// 반환을 가지는 함수
// 반환값의 타입은 -> 뒤에 명시한다.
fn five() -> i32 {
    5 // 반환값은 표현식이다. 따라서 세미콜론을 붙이지 않는다.
}

// 세미콜론을 붙이면 반환값이 없는 구문이 된다.
// 반환값이 없는 함수가 되며 이는 i32와 달라 컴파일 오류를 만든다.
fn plus_one(x:i32) -> i32 {
    x+1
}