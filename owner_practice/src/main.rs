fn main() {
    let s = String::from("Hello");

    takes_owner(s.clone()); // s를 직접 주면 s변수가 drop되면서 사라지므로, s.clone으로 복사체를 전달한다.
    let length:usize = takes_owner2(&s);

    let x = 5;
    makes_copy(x);

    println!("{s}");
    println!("{length}");
    println!("{x}");

    let mut ss = String::from("Hello");
    change(&mut ss);

    println!("{ss}");
}


fn takes_owner(some_string:String) {
    println!("{some_string}");
}

fn takes_owner2(string_addr:&String)-> usize{
    string_addr.len()
}

fn makes_copy(some_int:i32){
    println!("{some_int}");

}

fn change(some_string: &mut String){
    some_string.push_str(", world!");
}