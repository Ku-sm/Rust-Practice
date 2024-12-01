use std::io;

fn main() {
    println!("Hello, world!");

    change_degree();

    fibbo();
}

fn input_processing(buf: &mut String){
    io::stdin()
        .read_line(buf) //버퍼 앞에 mut 필요
        .expect("Failed to read line");
}

fn change_degree() {

    let mut input = String::new();
    println!("온도 타입을 입력해주세요, F or C");
    input_processing(&mut input);

    let degree_type = input.clone();
    let degree_type = degree_type.trim();

    println!("Degree Type Input: {degree_type}");

    if degree_type == "C" {
        println!("F온도 입력해주세요!");
        let mut number_input = String::new();
        input_processing(&mut number_input);
        let input_degree:f64 = number_input.trim().parse().expect("Plase input a number!");

        let changed_degree = (input_degree - 32.0) / 1.8 ;
        println!("Converted C to F : {input_degree}'C -> {changed_degree}'F");
    }
    else if degree_type == "F" {
        println!("F온도 입력해주세요!");
        let mut number_input = String::new();
        input_processing(&mut number_input);
        let input_degree:f64 = number_input.trim().parse().expect("Plase input a number!");

        let changed_degree = input_degree*1.8+32.0;
        println!("Converted F to C : {input_degree}'F -> {changed_degree}'C");
    }
    else {
        println!("Degree Type Rrr");
    }
}

fn fibbo(){
    let mut number = String::new();

    println!("Please input natural number");
    input_processing(&mut number);

    let number : usize = number.trim().parse().expect("Plase Input a Number!");


    if number <=0 {
        println!("0 보다 큰 숫자 입력 바람!");
        panic!();
    }

    if number < 3 {
        println!("Fibbo number: 1")
    }
    else{
        let mut fibbo_arr: [i64;100] = [0;100];
        fibbo_arr[0] = 1;
        fibbo_arr[1] = 1;

        let mut idx: usize = 2;

        while idx < number {
            // println!("{idx}");
            fibbo_arr[idx] = fibbo_arr[idx-1] + fibbo_arr[idx-2];
            idx = idx + 1
        }

        let fibbo_num = fibbo_arr[number-1];
        println!(" {number} 번째 fibbo num: {fibbo_num} ")
    }
}


