#[allow(dead_code)]
pub fn pattern_match(value: &str){
    let calc = |x: &str|{x.to_owned() + "と返しました。"};
    let result = match value {
        "test" => {
            calc("test")
        },
        "abc" => {
            calc("abc")
        },
        _ => {
            "想定外の値です。".to_owned()
        }
    };
    println!("{}", result);
}