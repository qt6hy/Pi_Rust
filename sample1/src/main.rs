use std::{time};

fn main() {

    let now = time::Instant::now(); // 時間計測

    let base:i64=10000;
    let mut n:i64 = 8400;
    let mut i;
    let mut out=0;
    let mut denom;

    let mut numerator: [i64;8400]=[base/5;8400]; //分子の初期化

    while n > 0 {
        let mut temp=0;
        i=n-1;
        while i>0{
            denom=2*i-1;
            temp = temp*i+numerator[i as usize]*base;
            numerator[i as usize]=temp%denom;
            temp = temp/denom;
            i-=1;
        }
        print!("{:>04}",out+temp/base);
        out = temp % base;
        n -= 14;
    }

    let erapsed = now.elapsed(); // 経過時間
    println!("\n{:?}", erapsed);
}
