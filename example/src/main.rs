fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1) {
        None => println!("コマンドライン引数が指定されていません"),
        Some(arg1) => {
            match arg1.parse::<i32>() {
                Ok(n) => println!("{}", add_up_to_multiple_of_4(n)),
                Err(_) => println!("コマンドライン引数の形式が不正です")
            }
        }
    }
}

fn add_up_to_multiple_of_4(x: i32) -> bool {
    x.to_string().chars().fold(0, |acc, c| c as i32 - 48 + acc) % 4 == 0
}
