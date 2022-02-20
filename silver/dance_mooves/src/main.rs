use std::io::BufRead;

fn main() {
    let stdio = std::io::stdin();
    let stdio_lock = stdio.lock();
    let mut lines = stdio_lock.lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut first_line = first_line
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let (n, k) = (first_line.next().unwrap(), first_line.next().unwrap());

    let swaps = (0..k)
        .map(|_| {
            let line = lines.next().unwrap().unwrap();
            let mut line = line.split_whitespace();
            let a = line.next().unwrap().parse::<usize>().unwrap() - 1;
            let b = line.next().unwrap().parse::<usize>().unwrap() - 1;
            (a, b)
        })
        .collect::<Vec<_>>();
    let mut first_round = vec![vec![]; n];
    let mut indices = (0..n).collect::<Vec<_>>();
    first_round.iter_mut().enumerate().for_each(|(i, row)| {
        row.push(indices[i]);
    });
    for swap in swaps {
        let (index_a, index_b) = swap;
        first_round[indices[index_a]].push(index_b);
        first_round[indices[index_b]].push(index_a);
        indices.swap(index_a, index_b);
    }

    let mut results = vec![None; n];
    for i in 0..n {
        if results[i].is_some() {
            continue;
        }

        // follow a cycle, building up a list of visited indices
        let mut visited = first_round[i].clone();
        let mut past_indices = vec![i];
        let mut current_index = indices[i];
        while current_index != i {
            visited.extend_from_slice(&first_round[current_index]);
            past_indices.push(current_index);
            current_index = indices[current_index];
        }
        visited.sort_unstable();
        visited.dedup();
        for index in &past_indices {
            results[*index] = Some(visited.len());
        }
    }
    results.iter().for_each(|x| {
        println!("{}", x.expect("no result"));
    });
}
