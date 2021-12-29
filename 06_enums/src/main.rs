enum IpAddr {
  V4(u8, u8, u8, u8),   // cada variante del enum puede definir sus propios datos (que pueden ser de cualquier tipo)
  V6(String)
}

#[derive(Debug)]
enum Message {
  Quit,
  Move { x: i32, y: i32},
  Write(String),
  ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        println!("{:#?}", self);
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn main() {
    let ipv4 = IpAddr::V4(127, 0, 0, 1);
    let ipv6 = IpAddr::V6(String::from("::1"));

    let message1 = Message::Write(String::from("hello"));
    message1.call();

    let message2 = Message::Move { x: 150, y: 100 };
    message2.call();

    println!("The value of a Nickel is {}", value_in_cents(Coin::Nickel));
    println!("The value of a Quarter is {}", value_in_cents(Coin::Quarter(UsState::Alaska)));
    println!("The value of a Quarter is {}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    let five = Some(5);
    let six = plus_one(five);
    println!("six: {:?}", six);
    let none = plus_one(None);
    println!("none: {:?}", none);

    let u8_value = 0u8;
    match u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => () //  `_` matchea cualquier valor (es el `default`). `()` es el "unit value" y no hace nada
    }

    // Para casos en los que solo quiero operar por un único valor del enum, se puede usar la sintaxis "if let".
    // Ejemplo:
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => ()
    }

    // es equivalente a:
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {                         // Se bindea el valor al matchear
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}