fn main() {
    
    another_function(5);
    expression_statement();
    control_flow_main();
    let_if();
    loop_fn();
    counter_loop_main();
    while_main();
    for_main();
    for_range_main();
}

fn another_function(x:i32){
    println!("the value of x is: {x}");
}

fn expression_statement(){
    let y={
        let x =3;
        x+1
    };
    println!("the value of y is: {y}");
}

fn control_flow_main() {
   
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

}

fn let_if(){
    let condition =true;
    let number =if condition{5}else{6};
    println!("the value of number is: {number}");
}

fn loop_fn(){
    let mut counter=0;

    let result =loop{
        counter+=1;

        if counter ==10{
            break counter*2;
        }
    };
    println!("the result is {result}");
}

fn counter_loop_main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn for_range_main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}