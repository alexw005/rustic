use rand::Rng;

fn main() {
    ip_enum();
    message_enum();
    message_method();
    option_enum();
    match_coin();
    match_option();
    dice_roll();
    match_coin_two();
}

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn ip_enum() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);
}

#[derive(Debug)]
enum Messange {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Messange {
    fn call(&self) {
        println!("call: {:?}", self);
    }
}

fn message_enum() {
    let quit = Messange::Quit;
    let move_msg = Messange::Move { x: 1, y: 2 };
    let write = Messange::Write(String::from("none"));
    let change_color = Messange::ChangeColor(1, 2, 3);
    println!("quit: {:?}", quit);
    println!("move_msg: {:?}", move_msg);
    println!("write: {:?}", write);
    println!("change_color: {:?}", change_color);
}

fn message_method(){
    let m = Messange::Write(String::from("hello"));
    m.call();
}


fn option_enum(){
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);
}
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn match_coin(){
    
    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }

    println!("value_in_cents(Coin::Penny): {}", value_in_cents(Coin::Penny));
    println!("value_in_cents(Coin::Nickel): {}", value_in_cents(Coin::Nickel));
    println!("value_in_cents(Coin::Dime): {}", value_in_cents(Coin::Dime));
    println!("value_in_cents(Coin::Quarter): {}", value_in_cents(Coin::Quarter(UsState::Alaska)));
}

fn match_option(){
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five: {:?}", five);
    println!("six: {:?}", six);
    println!("none: {:?}", none);
}

fn dice_roll(){
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let dice_roll = rng.gen_range(1..7);
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() { println!("add_fancy_hat")}
    fn remove_fancy_hat() {println!("remove_fancy_hat")}
}

fn match_coin_two(){
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let coin :Coin = Coin::Quarter(UsState::Alabama);//rng.gen_range(1..25);
    let mut count =0;
    match coin {
        Coin::Quarter(state)=> println!("State quarter from {:?}!", state),
        _ => count+=1
    }
}