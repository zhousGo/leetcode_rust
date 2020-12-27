fn reverse_string(s: &mut Vec<char>) {
    let length = s.len();
    let index = length / 2;
    let mut i = 0;
    while i < index {
        s.swap(i, length - 1 - i);
        i += 1;
    }
}

fn reverse_string2(s: &mut Vec<char>) {
    let n = s.len();
    let k = (s.len() / 2) as usize;
    for i in 0..k {
        s.swap(i, n - i - 1);
    }
}

#[test]
fn reverse_string_test() {
    let mut s = vec!['H', 'A', 'N', 'n', 'a', 'h'];
    println!("{:?}", &s);
    reverse_string2(&mut s);
    println!("{:?}", &s);
}
