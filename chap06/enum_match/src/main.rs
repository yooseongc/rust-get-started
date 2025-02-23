
// note: add `#[derive(Debug)]` to `UsState` or manually `impl Debug for UsState
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

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny   => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel  => 5,
        Coin::Dime    => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            // note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
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

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
// fn move_player(num_spaces: u8) {}
fn reroll() {}

fn main() {
    let five = Some(5);
    let six  = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other)
        // _ => reroll(),
        _ => (),
    }
}
