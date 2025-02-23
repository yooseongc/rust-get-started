
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {

    let config_max = Some(3u8);

    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {max}"),
    //     _ => ()
    // }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);

    // match coin {
    //     Coint::Quarter(state) => println!("State quarter from {state:?}!"),
    //     _ => count += 1,
    // }

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
}
