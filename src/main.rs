use std::io::{self, Write};

fn main() {
    println!("攝氏華氏轉換器 v0.1.0");
    println!("請選擇模式：");
    println!("1. 攝氏 -> 華氏");
    println!("2. 華氏 -> 攝氏");
    print!("> ");
    io::stdout().flush().unwrap();
    loop {
        let mut mode_in = String::new();
        match io::stdin().read_line(&mut mode_in) {
            Ok(x) => x,
            Err(_) => {
                println!("接收輸入錯誤");
                continue;
            }
        };
        let mode_in = match mode_in.trim().parse::<u8>() {
            Ok(x) => x,
            Err(_) => {
                println!("請輸入有效數字(u8)");
                continue;
            }
        };
        match mode_in {
            1 => {
                println!("攝氏轉華氏");
            },
            2 => {
                println!("華氏轉攝氏");
            },
            _ => {
                println!("模式錯誤，請輸入有效模式(1 or 2)");
                continue;
            }
        };
        println!("轉換後：{}度", c_f_converter(mode_in));
        break;
    }
}

fn c_f_converter(mode: u8) -> f64 {
    println!("請輸入欲轉換的{}氏溫度：", if mode == 1 {"攝"} else {"華"});
    print!("> ");
    io::stdout().flush().unwrap();

    let deg_in = loop {
        let mut deg_in = String::new();
        match io::stdin().read_line(&mut deg_in) {
            Ok(x) => x,
            Err(_) => {
                println!("接收輸入錯誤");
                continue;
            }
        };
        let deg_in = match deg_in.trim().parse::<f64>() {
            Ok(x) => x,
            Err(_) => {
                println!("請輸入有效數字(f64)");
                continue;
            }
        };
        break deg_in;
    };

    if mode == 1_u8 {
        ((deg_in * 9_f64) / 5_f64) + 32_f64
    } else {
        ((deg_in - 32_f64) * 5_f64) / 9_f64
    }
}