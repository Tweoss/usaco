use std::io::BufRead;

fn main() {
    let stdio = std::io::stdin();
    let stdio_lock = stdio.lock();
    let line = stdio_lock.lines().nth(1).unwrap().unwrap();
    let (mut odd, mut even) = line.split(' ').fold((0, 0), |a, c| {
        let value = c.parse::<u32>().unwrap();
        if value % 2 == 0 {
            (a.0, a.1 + 1)
        } else {
            (a.0 + 1, a.1)
        }
    });
    let mut odd_not_even = false;
    let mut counter = 0;
    loop {
        if odd_not_even {
            if odd > 0 && !(odd == 2 && even == 0) {
                odd -= 1;
            } else {
                break;
            }
        } else if even > 0 {
            even -= 1;
        } else if odd > 1 {
            odd -= 2;
        } else {
            break;
        }
        counter += 1;
        odd_not_even = !odd_not_even;
    }
    println!("{}", counter);
}
