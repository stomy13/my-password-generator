use clap::Parser;
use rand::{thread_rng, Rng};

/// シンプルなパスワードジェネレーター
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 生成するパスワードの長さ
    #[arg(long, default_value_t = 20)]
    length: usize,

    /// 大文字を含める
    #[arg(long, default_value_t = true)]
    uppercase: bool,

    /// 小文字を含める
    #[arg(long, default_value_t = true)]
    lowercase: bool,

    /// 数字を含める
    #[arg(long, default_value_t = true)]
    numbers: bool,

    /// 特殊文字を含める
    #[arg(long, default_value_t = true)]
    special: bool,
}

fn generate_password(
    length: usize,
    use_upper: bool,
    use_lower: bool,
    use_num: bool,
    use_special: bool,
) -> String {
    let mut rng = thread_rng();
    let mut password = String::new();
    let mut charset = String::new();

    if use_upper {
        charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if use_lower {
        charset.push_str("abcdefghijklmnopqrstuvwxyz");
    }
    if use_num {
        charset.push_str("0123456789");
    }
    if use_special {
        charset.push_str("!@#$%^&*()-_=+[]{}|;:,.<>?");
    }

    if charset.is_empty() {
        // 何も指定されていない場合はデフォルトで英数字
        charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".to_string();
    }

    for _ in 0..length {
        let idx = rng.gen_range(0..charset.len());
        password.push(charset.chars().nth(idx).unwrap());
    }

    password
}

fn main() {
    let args = Args::parse();

    let password = generate_password(
        args.length,
        args.uppercase,
        args.lowercase,
        args.numbers,
        args.special,
    );

    println!("生成されたパスワード: {}", password);
}
