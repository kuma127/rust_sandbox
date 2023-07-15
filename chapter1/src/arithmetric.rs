#[allow(dead_code)]
pub fn symbol(){
    let mut result = 10 + 20;
    println!("10 + 20 = {}", result);
    result = 20 - 10;
    println!("20 - 10 = {}", result);
    result = 10 * 20;
    println!("10 * 20 = {}", result);
    result = 10 / 5;
    println!("10 / 5 = {}", result);
    result = 10 % 3;
    println!("10 % 3 = {}", result);
}

#[allow(dead_code)]
pub fn methods(x: i32, y: i32){
    // トレイトの宣言
    use std::ops::{Add, Sub, Mul, Div, Rem};
    // 各メソッドを使って計算する
    println!("{} + {} = {}", x, y, x.add(y));
    println!("{} - {} = {}", x, y, x.sub(y));
    println!("{} * {} = {}", x, y, x.mul(y));
    println!("{} / {} = {}", x, y, x.div(y));
    println!("{} % {} = {}", x, y, x.rem(y));
}

#[allow(dead_code)]
pub fn overflow(){
    let x: u8 = 100;
    let y: u8 = 200;
    let result = x + y;
    println!("{} + {} = {}", x, y, result);
}