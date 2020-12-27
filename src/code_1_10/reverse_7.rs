fn reverse(x: i32) -> i32 {
    x.abs()
        .to_string()
        .chars().rev()
        .collect::<String>()
        .parse::<i32>()
        .unwrap_or(0) * x.signum()
}

fn reverse_01(x: i32) -> i32 {
    let max = (i32::MAX - 7) / 10;
    let min = (i32::MIN + 8) / 10;

    let mut x = x;
    let mut r = 0;

    while x != 0 {
        let m = x % 10;
        if r > max || (r == max && m > 7) || r < min || (r == min && m < -8) {
            return 0;
        }
        r = r * 10 + m;
        x = (x - m) / 10;
    }
    r
}

#[test]
fn reverse_test() {
    let num = reverse(-12120);
    println!("{}", num);
    let reverse01 = reverse_01(-12120);
    println!("{}", reverse01);
}