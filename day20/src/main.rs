use std::collections::VecDeque;
use std::io::BufRead;
use std::time::Instant;

fn mix(data: &mut VecDeque<(usize, isize)>) {
    for i in 0..data.len() {
        //        println!(" {data:?}");
        let i = data.iter().position(|(n, _)| *n == i).unwrap();
        let d = data[i].1;

        let n = data.remove(i).unwrap();
        let i = i as isize + d;
        let i = i.rem_euclid(data.len() as isize) as usize;

        data.insert(i, n);
    }
    //    println!(" {data:?}");
}

fn main() {
    let input: VecDeque<(usize, isize)> = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter(|l| !l.is_empty())
        .filter_map(|l| l.parse().ok())
        .enumerate()
        .collect();

    let mut data = input.clone();
    let t = Instant::now();

    mix(&mut data);

    let zero = data.iter().position(|(_, v)| *v == 0).unwrap();
    let p1 = data[(zero + 1000) % data.len()].1
        + data[(zero + 2000) % data.len()].1
        + data[(zero + 3000) % data.len()].1;
    let t1 = t.elapsed();

    let mut data = input;
    for (_, d) in &mut data {
        *d *= 811589153;
    }

    for _ in 0..10 {
        mix(&mut data);
    }

    let zero = data.iter().position(|(_, v)| *v == 0).unwrap();
    let p2 = data[(zero + 1000) % data.len()].1
        + data[(zero + 2000) % data.len()].1
        + data[(zero + 3000) % data.len()].1;
    let t2 = t.elapsed();

    println!("Part1: {p1}, time: {t1:?}");
    println!("Part2: {p2}, time: {:?}", t2 - t1);
    println!("Total time: {t2:?}");
}
