//! Practice Rust : コラッツ予想を計算する CLI ツール

// コマンドライン引数を受け取るための標準ライブラリ
use std::env;

/// メイン関数
fn main() {
  // let は不変。let mut とすると可変 (ミュータブル) になる
  let args: Vec<String> = env::args().collect();
  
  // コマンドライン引数が1つあればそれを利用する
  if args.len() > 1 {
    let arg_value = &args[1];
    // 引数を正の整数値に変換できなければ終了する
    let num: u32 = arg_value.parse().unwrap_or_else(|error| {
      println!("Invalid Arg Value : {:?}", error);
      std::process::exit(1);
    });
    println!("Collatz : {}", num);
    collatz(num);
  }
  else {
    let default_num = 10;
    println!("Collatz : Default {}", default_num);
    collatz(default_num);
  }
  println!("Finished");
}

/// コラッツの数列を計算する関数
fn collatz(num: u32) {
  println!("{}", num);  // ! 付きだとマクロ
  if num % 2 != 0 && num > 1 {
    collatz(3 * num + 1);
  }
  else if num % 2 == 0 {
    collatz(num / 2);
  }
}
