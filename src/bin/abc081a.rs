use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        a: Chars,
    }

    let mut ans = 0;
    for val in a.iter() {
        ans = ans + val.to_digit(10).unwrap();
    }
    println!("{}", ans);
}
