use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };

    let mut accept = true;

    let mut state = 0u64;

    for c in s.into_iter() {
        match state {
            0 => {
                state = if c == 'x' {1} else {2};
            },

            // 1文字目がxだったとき
            1 => {
                state = if c == 'o' {2} else {4};
            },

            // oだったとき
            2 => {
                if c == 'x' {
                    state = 3;
                } else {
                    accept = false;
                    break;
                }
            },

            3 => {
                if c == 'x' {
                    state = 4;
                } else {
                    accept = false;
                    break;
                }
            },

            4 => {
                if c == 'o' {
                    state = 2;
                } else {
                    accept = false;
                    break;
                }
            },

            _ => (),
        }
    }

    println!("{}", if accept {"Yes"} else {"No"});
}
