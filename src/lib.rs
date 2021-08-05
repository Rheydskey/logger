mod test;


#[macro_export]
macro_rules! warn {
    ($x:expr) => {
        let string = format!("{}:{}:{} [WARNING] {}",file!(), line!(), column!(), $x);
        println!("\x1b[0;33m{}\x1b[0;30m", string)
    };
}

#[macro_export]
macro_rules! info {
    ($x:expr) => {
        let string = format!("{}:{}:{} [INFO] {}", file!(), line!(), column!(), $x);
        println!("\x1b[0;34m{}\x1b[0;30m", string)
    };
}

#[macro_export]
macro_rules! error {
    ($x:expr) => {
        let string = format!("{}:{}:{} [ERROR] {}", file!(), line!(), column!(), $x);
        println!("\x1b[0;31m{}\x1b[0;30m", string)
    };
}

#[macro_export]
macro_rules! success {
    ($x:expr) => {
        let string = format!("{}:{}:{} [SUCCESS] {}", file!(), line!(), column!(), $x);
        println!("\x1b[0;32m{}\x1b[0;30m", string)
    };
}