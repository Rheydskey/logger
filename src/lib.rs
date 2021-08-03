mod test;

use colorful::{Color, Colorful};

#[macro_export]
macro_rules! warn {
    ($x:expr) => {
        let string = format!("{}/{}:{} [WARNING] {}",file!(), line!(), column!(), $x);
        println!("{}", string.color(Color::Yellow))
    };
}

#[macro_export]
macro_rules! info {
    ($x:expr) => {
        let string = format!("{}/{}:{} [INFO] {}", file!(), line!(), column!(), $x);
        println!("{}", string.color(Color::Blue))
    };
}

#[macro_export]
macro_rules! error {
    ($x:expr) => {
        let string = format!("{}/{}:{} [ERROR] {}", file!(), line!(), column!(), $x);
        println!("{}", string.color(Color::Red))
    };
}

#[macro_export]
macro_rules! success {
    ($x:expr) => {
        let string = format!("{}/{}:{} [SUCCESS] {}", file!(), line!(), column!(), $x);
        println!("{}", string.color(Color::Green))
    };
}