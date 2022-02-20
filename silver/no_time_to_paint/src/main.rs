use std::io::BufRead;

fn build_solution<T>(input: T) -> Vec<usize>
where
    T: IntoIterator<Item = char>,
{
    let mut stack: Vec<char> = Vec::new();
    let mut count = 0;
    let mut out = vec![0];
    for c in input {
        while !stack.is_empty() && c < *stack.last().unwrap() {
            stack.pop();
        }
        if stack.last().map(|x| *x < c).unwrap_or(true) {
            count += 1;
            stack.push(c);
        }
        out.push(count);
    }
    out
}

fn main() {
    let stdio = std::io::stdin();
    let stdio_lock = stdio.lock();
    let mut lines = stdio_lock.lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut first_line = first_line
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let (_, q) = (first_line.next().unwrap(), first_line.next().unwrap());
    let input = lines.next().unwrap().unwrap();
    let (paints_left, paints_right) = (
        build_solution(input.chars()),
        build_solution(input.chars().rev()),
    );
    println!(
        "{}",
        (0..q)
            .map(|_| {
                let line = lines.next().unwrap().unwrap();
                let mut line = line.split_whitespace();
                let a = line.next().unwrap().parse::<usize>().unwrap() - 1;
                let b = line.next().unwrap().parse::<usize>().unwrap();
                format!("{}", paints_left[a] + paints_right[input.len() - b])
            })
            .collect::<Vec<_>>()
            .join("\n")
    );
}
