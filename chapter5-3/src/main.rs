// method

#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32
}

// Rectangle 구조체에 area 메서드 정의
// impl (implement): 구현, 내부 블럭에 작성된 함수는 모두 연관함수에 해당
// impl 블럭은 여러개로 분리 가능, 단 하나만 존재하지 않아도 됨
impl Rectangle{
    fn area(&self) -> u32{ //실제로는 self:&self를 줄인 형태
        self.width*self.height
    }

    fn width(&self) -> bool { // getter의 경우 사용하는 경우 많음, 인수와 메서드 이름 동일
        self.width > 0
    }

    fn can_hold(&self, other:&Rectangle) -> bool { // 다른 요소와 비교
        (self.width > other.width) && (self.height > other.height)
    }

    // self를 첫 매개변수로 갖지 않는 연관함수 정의 가능 -> String::from 함수가 예시
    fn square(size:u32) -> Self { // Self 키워드는 impl 뒤에 적혀있는 타입의 별칭
        Self{
            width:size,
            height:size
        }
    }
}


fn main() {
    let rect1 = Rectangle{
        width:30,
        height:50
    };

    println!("Area: {}", rect1.area());

    if rect1.width() {
        println!("The Rectangle has a non-zero width, it's {}", rect1.width());
    }

    let rect2 = Rectangle{
        width:10,
        height:40
    };

    let rect3 = Rectangle{
        width:60,
        height:45
    };

    println!("Can rect1 hold rect2?: {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3?: {}", rect1.can_hold(&rect3));

    // 연관함수 호출시, 구조체명에 ::을 붙여서 호출, 연관함수는 구조체 네임스페이스 안에 존재하기 때문
    let square1 = Rectangle::square(20);
    println!("Square1's area: {}", square1.area());

}


