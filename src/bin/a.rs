fn main() {
    proconio::input! {
        n: u64,
    }

    println!("AGC{0: <03}", if n < 42 {n} else {n + 1});
}
