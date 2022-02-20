use std::io::BufRead;

fn main() {
    let stdio = std::io::stdin();
    let stdio_lock = stdio.lock();
    let mut iterator = stdio_lock.lines().skip(1).take(2);
    let (first_line, second_line) = (
        iterator.next().unwrap().unwrap(),
        iterator.next().unwrap().unwrap(),
    );
    let mut first = first_line
        .split(' ')
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let second = second_line
        .split(' ')
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    first.sort_unstable();
    let mut total = 1;
    // start with tall cows first
    while let Some(cow) = first.pop() {
        let available_slots = second
            .iter()
            .fold(0, |acc, stall| if cow <= *stall { acc + 1 } else { acc })
            // this number of stalls is already occupied by previous cows
            - (second.len() - (first.len() + 1));
        total *= available_slots;
        if total == 0 {
            break;
        }
    }
    println!("{}", total);
}
