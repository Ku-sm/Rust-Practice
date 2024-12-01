fn main() {
    // Scalar Types

    // 정수형 타입
    // 부호가 있는 경우: i8, i16, i32, i64, i128, isize -> 0 ~ 2^n-1 범위까지
    // 부호가 없는 경우: u8, u16, u32, u64, u128, usize -> - 2^(n-1) ~ 2^(n-1)-1 범위까지
    // arch: isize, usize의미하며, 컴퓨터 아키텍처에 따라 32비트 또는 64비트로 할당됨
    // 러스트의 기본 정수형 타입은 i32이다.

    // 숫자 사이에 _를 사용하여 가독성을 높일 수 있다.
    let x = 98_222;
    println!("The value of x is: {x}"); // 98222로 10진수 출력

    let x = 0xff;
    println!("The value of x is: {x}"); // 16진수 변수, 출력은 10진수로

    let x = 0o77;
    println!("The value of x is: {x}"); // 8진수 변수

    let x = 0b1111_0000;
    println!("The value of x is: {x}"); // 2진수 변수

    let x = b'A';
    println!("The value of x is: {x}"); // 바이트 변수

    //----------------------------------------------
    // 부동소수점 타입, IEEE-754 표준에 따라 f32, f64 타입을 제공
    // f32, f64, 기본 타입은 f64

    let x = 2.0; // f64
    let y:f32 = 3.0; // f32

    let sum = x + y;
    println!("The value of sum is: {sum}"); // 이게 되네;;;

    //----------------------------------------------
    // 사칙연산

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;

    let quotient = 56.7 / 32.2;
    let truncated = -5/3; // -1

    let remainder = 43 % 5;

    println!("{sum}, {difference}, {product}, {quotient}, {truncated}, {remainder}");

    //----------------------------------------------
    // 불리언 타입, 1바이트 크기, true, false 값만 가질 수 있다.

    let t = true;
    let f: bool = false;

    println!("{t}, {f}");

    //----------------------------------------------
    // 문자 타입, char, 4바이트 크기, 유니코드 스칼라 값으로 표현된다.
    // 문자는 작은 따옴표로 표현한다.

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("{c}, {z}, {heart_eyed_cat}");

    //----------------------------------------------
    // 복합 타입, 튜플과 배열
    // 튜플은 다양한 타입의 여러 값을 묶어 하나의 복합 타입으로 만드는 일반적인 방법
    // 고정된 크기를 가지며, 한 번 선언되면 크기를 변경할 수 없다.

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 튜플은 하나의 복합요소로, 변수 tup는 튜플 전체가 바인딩 된다.
    // 따라서, 튜플의 요소에 접근하려면 패턴 매칭을 사용해야 한다.
    let (x, y, z) = tup;
    println!("{x}, {y}, {z}");

    // 튜플의 요소에 접근하는 다른 방법은 인덱스를 사용하는 것이다.
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("{five_hundred}, {six_point_four}, {one}");


    // 배열은 같은 타입의 요소들로 이루어진다.
    // 배열의 크기는 고정되어 있으며, 스택에 할당된다.
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5]; // 배열의 타입과 크기를 명시적으로 표현
    let a = [3; 5]; // [3, 3, 3, 3, 3]으로 초기화 [초기값;크기]

    let first = a[0];
    let second = a[1];

    println!("{first}, {second}");
    // 유효하지 않은 인덱스에 접근하면 컴파일 오류가 아닌 런타임 오류가 발생한다.
}
