
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn learn_enum() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    dbg!(four);
    dbg!(six);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    match home {
        IpAddr::V4(a, b, c, d) => println!("home: {}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(s) => println!("home: {}", s),
    }

    match loopback {
        IpAddr::V4(a, b, c, d) => println!("loopback: {}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(s) => println!("loopback: {}", s),
    }
}

fn learn_option() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    dbg!(some_number);
    dbg!(some_string);
    dbg!(absent_number);
}

#[derive(Debug)]
enum UsState {
    Alabama,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn learn_match() {
    let coin_penny = Coin::Penny;
    let coin_nickle = Coin::Nickel;
    let coin_dime = Coin::Dime;
    let coin_quarter = Coin::Quarter(UsState::Alabama);
    
    println!("the value of coin_penny in cents: {}", value_in_cents(coin_penny));
    println!("the value of coin_nickle in cents: {}", value_in_cents(coin_nickle));
    println!("the value of coin_dime in cents: {}", value_in_cents(coin_dime));
    println!("the value of coin_quarter in cents: {}", value_in_cents(coin_quarter));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    dbg!(six);
    dbg!(none);

    let dice_roll = 9;
    match dice_roll {
        3 => println!("you rolled a 3, so move 3 spaces"),
        7 => println!("you rolled a 7, so move 7 spaces"),
        other => println!("you rolled {}, so move {} spaces", other, other),
    }
    match dice_roll {
        3 => println!("you rolled a 3, so move 3 spaces"),
        7 => println!("you rolled a 7, so move 7 spaces"),
        _ => (), // do nothing
    }
}

fn learn_if_let_else() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("The maximum is not configured.");
    }

    let a: Option<i32> = None;
    let Some(b) = a else {
        println!("a is None.");
        return;
    };
    println!("a is Some and contains: {}", b);
}

pub fn learn_enums_match() {
    learn_enum();
    learn_option();
    learn_match();
    learn_if_let_else();
}
