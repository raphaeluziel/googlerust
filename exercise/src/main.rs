/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    let mut i = 0;
    loop {
        if n == 1 {
            break;
        } else if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        i += 1
    }
    i + 1
}

fn collatz_length_solution(mut n: i32) -> u32 {
    let mut len = 1;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }
    len
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
}

fn main() {
    let mut a = 1;
    let rp = 30;

    while a < rp {
        println!("n = {a}");
        println!("Length:          {}", collatz_length(a));
        println!("Length_solution: {}", collatz_length_solution(a));
        println!();
        a += 1;
    }
}
