fn main() {
    struct_call();
    tuple_struct();
    unit_like_struct();
    example_struct();
    method_syntax();
}

fn struct_call(){
    struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("user2: {}", user2.email);
    println!("user2 active: {}", user2.active);
}

fn tuple_struct(){
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black: {}, {}, {}", black.0, black.1, black.2);
    println!("origin: {}, {}, {}", origin.0, origin.1, origin.2);
}

fn unit_like_struct(){
    struct UnitLikeStruct;
    let unit_like_struct = UnitLikeStruct;
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn example_struct(){
    let scale =2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(&rect1)
    // );
    println!("rect1 : {:#?}", rect1);

    dbg!(&rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn area(&self)-> u32{
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn method_syntax(){
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );
    if rect2.width() {
        println!("The rectangle has a nonzero width; it is {}", rect2.width);
    }

}