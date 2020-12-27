fn is_palindrome(x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }
    let mut x = x;
    let mut temp = 0;
    while x > temp {
        temp = temp * 10 + x % 10;
        x /= 10;
    }
    return temp == x || x == temp / 10;
}

fn is_palindrome_01(x: i32) -> bool {
    let mut s = x.to_string();
    let mut temp = String::new();
    for i in 0..s.len() {
        temp.push(s.pop().unwrap());
    }
    return x.to_string() == temp;
}

#[test]
fn test() {
    println!("{}", is_palindrome(101));
    println!("{}", is_palindrome_01(101));
}