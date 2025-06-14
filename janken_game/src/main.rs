use rand::prelude::*;
use std::io;
fn main() {
    println!("じゃんけんゲームへようこそ！");
    println!("グー・チョキ・パーのどれかを入力してください");
    let mut user_hand = String::new();
    io::stdin().read_line(&mut user_hand).unwrap();
    let user_hand = user_hand.trim();
    let hands = ["グー", "チョキ", "パー"];

    let mut rng = rand::rng();
    let computer_index = rng.random_range(0..3);
    let computer_hand = hands[computer_index];

    println!("あなた：{}", user_hand);
    println!("コンピュータ：{}", computer_hand);

    match (user_hand, computer_hand) {
        (u, c) if u == c => println!("あいこ！"),
        ("グー", "チョキ") | ("チョキ", "パー") | ("パー", "グー") => {
            println!("あなたの勝ち🎉🎉")
        }
        ("グー", "パー") | ("チョキ", "グー") | ("パー", "チョキ") => {
            println!("あなたの負け・・・😭💦")
        }
        _ => println!("入力が無効です"),
    }
}
