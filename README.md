# Practice Rust

Rust を始めてみるため、コラッツ予想を計算する CLI ツールを作ってみました。

```bash
$ cargo run
   Compiling practice_rust v0.1.0 (/PATH/TO/practice-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.36s
     Running `target/debug/practice_rust`
Collatz : Default 10
10
5
16
8
4
2
1
Finished

# 任意の整数値を指定できる
$ cargo run 168
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/practice_rust 168`
Collatz : 168
168
84
42
21
64
32
16
8
4
2
1
Finished

# ビルドして実行する
$ cargo build
$ ./target/debug/practice_rust

# ドキュメンテーションを生成する
$ cargo doc
$ open ./target/doc/practice_rust/index.html
```


## Links

- [Neo's World](https://neos21.net/)
