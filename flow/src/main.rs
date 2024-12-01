

fn main() {
    let number = 3;

    // 조건문도 match와 같이 갈래 (arm)으로 불린다.
    // if의 조건식은 반드시 bool 타입이어야 한다.
    if number < 5 {
        println!("condition was true");
    }
    else {
        println!("condition was false");
    }
    // ** 러스트는 정수 타입을 bool 타입으로 변환하지 않는다. **

    zero_check(number);

    let number = 6;
    div_check(number);

    if_allocation();

    loop_example();

    double_loop();

    while_loop();

    for_loop();

    for_loop_with_range();
}

fn zero_check(x:i32){
    if x != 0 {
        println!("x was something other than zero");
    }

}

fn div_check(x:i32){
    if x % 4 == 0 {
        println!("x is divisible by 4");
    }
    else if x % 3 == 0 {
        println!("x is divisible by 3");
    }
    else if x % 2 == 0 {
        println!("x is divisible by 2");
    }
    else {
        println!("x is not divisible by 4, 3, or 2");
    }
}

// if는 표현식이기 때문에 다음과 같이 사용할 수 있다.
// 단 중괄호 안 마지막 표현식이 반환되어야 한다.
fn if_allocation(){
    let condition = true;
    let number = if condition {5} else {6};

    // let number = if condition {5} else {"six"}; 두 표현식의 타입이 다르면 컴파일 에러
    // 변수가 가질 수 있는 타입은 하나이기 때문이다.

    println!("The value of number is: {}", number);
}

fn loop_example(){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}


fn double_loop(){
    let mut counter = 0;

    'counting_up: loop {
        println!("count = {counter}");
        let mut remaining = 10;

        loop{
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }

            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        counter += 1;
    }
    println!("End of double loop");
    println!("counter = {counter}");
}

fn while_loop(){
    let mut number =30;

    while number !=0 {
        if number == 28 {
            break;
        }
        println!("{}", number);
        number -= 1;
    }

}

fn for_loop(){
    let a = [10, 20, 30, 40, 50];

    // let mut index = 0;
    // while index < 5 {
    //     println!("the value is: {}", a[index]);
    //     index += 1;
    // }

    for element in a {
        println!("the value is: {}", element);
    }
}

fn for_loop_with_range(){
    for number in (1..4).rev(){ // .rev()는 reverse의 약자로 역순으로 순회한다.
        println!("{}", number);
    }
}