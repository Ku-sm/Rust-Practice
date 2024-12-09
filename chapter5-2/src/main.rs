fn rectangle(){

    let width1 = 30;
    let height1 = 50;

    println!("
        The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width:u32, height:u32) -> u32 {
    width*height // return 없이도, 문장 뒤 ; 없으면 return 값으로 인식됨
}


// 튜플 리팩토링
fn area_tuple(dimension:(u32,u32)) -> u32{
    dimension.0 * dimension.1
}

fn rectangle_tuple(){

    let rect1 = (40,50);

    println!("
        The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );
}

// 구조체 리팩토링
#[derive(Debug)] // 디버깅 정보 출력 기능, 외부 속성 작성 필요
struct Rectangle{
    width:u32,
    height:u32
}

fn area_struct(rec:&Rectangle) -> u32{
    rec.width * rec.width
}

fn rectangle_struct(){

    let rect1 = Rectangle{width:50, height:50}; // 구조체는 중괄호로 선언

    println!("
        The area of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );

    println!("rect1 is {:?}",rect1); //  :? 는 디버그 출력 형식 사용 명시를 의미
}

fn main() {
    rectangle();
    rectangle_tuple();
    rectangle_struct();
}
