use clap::Parser;
use rand::rngs::OsRng;
use rand::Rng;

/// シンプルなパスワードジェネレーター
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 生成するパスワードの長さ
    #[arg(long, default_value_t = 20)]
    length: usize,

    /// 大文字を含める
    #[arg(long)]
    uppercase: bool,

    /// 小文字を含める
    #[arg(long)]
    lowercase: bool,

    /// 数字を含める
    #[arg(long)]
    numbers: bool,

    /// 特殊文字を含める
    #[arg(long)]
    special: bool,
}

const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &str = "0123456789";
const SPECIAL: &str = "!@#$%^&*()-_=+[]{}|;:,.<>?";

fn generate_password(
    length: usize,
    use_upper: bool,
    use_lower: bool,
    use_num: bool,
    use_special: bool,
) -> String {
    let mut charset = String::new();
    if use_upper {
        charset.push_str(UPPERCASE);
    }
    if use_lower {
        charset.push_str(LOWERCASE);
    }
    if use_num {
        charset.push_str(NUMBERS);
    }
    if use_special {
        charset.push_str(SPECIAL);
    }

    if charset.is_empty() {
        // 何も指定されていない場合はデフォルトで全文字種を含める
        charset = format!("{}{}{}{}", UPPERCASE, LOWERCASE, NUMBERS, SPECIAL);
    }

    let mut rng = OsRng;
    let password: String = (0..length)
        .map(|_| {
            let idx: usize = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect();
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
