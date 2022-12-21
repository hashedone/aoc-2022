#![feature(array_windows)]

use smallvec::{smallvec, SmallVec};
use std::collections::{BinaryHeap, HashMap};
use std::io::BufRead;
use std::time::Instant;

fn input() -> HashMap<String, (i64, Vec<String>)> {
    std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (valve, tunnels) = &line.trim().split_once(';').unwrap();
            let valve = valve.trim().strip_prefix("Valve ").unwrap();
            let (name, flow) = valve.trim().split_once(' ').unwrap();
            let name = name.to_owned();
            let (_, flow) = flow.trim().split_once('=').unwrap();
            let flow = flow.trim().parse().unwrap();
            let (_, tunnels) = tunnels.trim().split_once("valve").unwrap();
            let tunnels = &tunnels[1..];
            let tunnels = tunnels
                .trim()
                .split(',')
                .map(|tunnel| tunnel.trim().to_owned())
                .collect();

            (name, (flow, tunnels))
        })
        .collect()
}

type Tunnels = SmallVec<[usize; 5]>;

fn preprocess(input: HashMap<String, (i64, Vec<String>)>) -> (Vec<(i64, Tunnels)>, usize) {
    let mut key: Vec<_> = input.keys().collect();
    key.sort();

    let input = input.iter().fold(
        vec![(0, smallvec![]); key.len()],
        |mut valves, (name, (rate, tunnels))| {
            let idx = key.binary_search(&name).unwrap();
            let tunnels = tunnels
                .iter()
                .map(|tunnel| key.binary_search(&tunnel).unwrap())
                .collect();

            valves[idx] = (*rate, tunnels);
            valves
        },
    );

    let start = key.binary_search(&&("AA".to_owned())).unwrap();
    (input, start)
}

fn build_distance_table(input: &[(i64, Tunnels)]) -> Vec<i64> {
    let valves = input.len();
    let mut dists = vec![i64::MAX; valves * valves];
    let mut queue = BinaryHeap::new();

    for idx in 0..valves {
        dists[idx * valves + idx] = 0;
    }

    for (idx, (_, tunnels)) in input.iter().enumerate() {
        for tunnel in tunnels {
            dists[idx * valves + tunnel] = 1;
        }

        queue.extend(tunnels.iter().map(|tunnel| (1, *tunnel)));

        while let Some((dist, tunnel)) = queue.pop() {
            let (_, next_tunnels) = &input[tunnel];
            for next_tunnel in next_tunnels {
                let idx = idx * valves + next_tunnel;
                if dists[idx] > dist + 1 {
                    dists[idx] = dist + 1;
                    queue.push((dist + 1, *next_tunnel));
                }
            }
        }
    }

    dists
}

// pub fn next_permutation(nums: &mut [(usize, i64)]) -> bool {
//     use std::cmp::Ordering;
//     // or use feature(array_windows) on nightly
//     let last_ascending = match nums.windows(2).rposition(|w| w[0] < w[1]) {
//         Some(i) => i,
//         None => {
//             nums.reverse();
//             return false;
//         }
//     };
//
//     let swap_with = nums[last_ascending + 1..]
//         .binary_search_by(|n| nums[last_ascending].cmp(n).then(Ordering::Less))
//         .unwrap_err(); // cannot fail because the binary search will never succeed
//     nums.swap(last_ascending, last_ascending + swap_with);
//     nums[last_ascending + 1..].reverse();
//     true
// }

fn part1(input: &[(i64, Tunnels)], dist_table: &[i64], start: usize) -> i64 {
    const TIME: i64 = 30;

    let cnt = input.len();
    let mut valves: Vec<_> = input
        .iter()
        .enumerate()
        .map(|(idx, (rate, _))| (idx, *rate))
        .filter(|(_, rate)| *rate > 0)
        .collect();

    valves.sort_by_key(|(_, rate)| *rate);

    let mut released = vec![false; valves.len()];

    // (valve_idx, flow, time_left)
    let mut stack: Vec<(usize, i64, i64)> = Vec::with_capacity(valves.len());
    let mut best = 0;
    let mut start_from = 0;

    loop {
        loop {
            // look a valve to add to the stack, so we still have time to release it
            let (prev, flow, time) = stack
                .last()
                .map(|(prev, flow, time)| (valves[*prev].0, *flow, *time))
                .unwrap_or((start, 0, TIME));

            let Some((next, flow, time)) = valves
            .iter()
            .enumerate()
            .skip(start_from)
            .find_map(|(valve, (idx, next_flow))| {
               let dist = dist_table[prev * cnt + *idx];

               if released[valve] || dist >= time {
                   return None;
               }

               let time = time - dist - 1;
               let max_flow = valves.iter().enumerate().filter_map(|(idx, (_, rate))| {
                   match released[idx] {
                       true => None,
                       false => Some(*rate),
                   }
               }).sum::<i64>() * time + flow;

               if max_flow < best {
                   return None;
               }

               let flow = flow + time * next_flow;

               Some((valve, flow, time))
            }) else {
                break;
            };

            released[next] = true;
            stack.push((next, flow, time));
            start_from = 0;
        }

        // Update on best solution
        best = best.max(stack.last().map(|(_, flow, _)| *flow).unwrap_or(0));

        // Backtrack once
        let Some((valve, _, _)) = stack.pop() else {
            break;
        };

        released[valve] = false;
        start_from = valve + 1;
    }

    best
}

fn part2(input: &[(i64, Tunnels)], dist_table: &[i64], start: usize) -> i64 {
    const TIME: i64 = 26;

    let cnt = input.len();
    let mut valves: Vec<_> = input
        .iter()
        .enumerate()
        .filter(|(_, (rate, _))| *rate > 0)
        .map(|(idx, _)| idx)
        .collect();

    valves.sort_by_key(|idx| input[*idx].0);

    let mut released: u16 = 0;
    // (my_valve, el_valve, flow, my_time, el_time, force, added_valve)
    let mut stack: Vec<(usize, usize, i64, i64, i64, usize, usize)> =
        Vec::with_capacity(valves.len());
    stack.push((start, start, 0, TIME, TIME, 0, 0));

    let mut best = 0;
    let mut start_from = 0;
    // 0 = none, 1 = me, 2 = elephant
    let mut force = 0;

    loop {
        loop {
            // look a valve to add to the stack, so we still have time to release it
            let (my_prev, el_prev, flow, my_time, el_time, _, _) = stack.last().unwrap();

            let Some((my_next, el_next, flow, my_time, el_time, next_force, added)) = valves
                .iter()
                .enumerate()
                .skip(start_from)
            .find_map(|(idx, valve)| {
                let (next_flow, _) = input[*valve];
               let dist_my = dist_table[my_prev * cnt + valve];
               let dist_el = dist_table[el_prev * cnt + valve];

               let next_my_time = my_time - dist_my - 1;
               let next_el_time = el_time - dist_el - 1;
               if (released & 2 << idx) > 0 || (next_my_time < 1 && next_el_time < 1) {
                   return None;
               }

               let force = if idx == start_from {
                   force
               } else {
                   0
               };

               let (my_next, el_next, my_time, el_time, t, force) = match force {
                   0 if next_my_time >= next_el_time => (*valve, *el_prev, next_my_time, *el_time, next_my_time, 2),
                   0 => (*my_prev, *valve, *my_time, next_el_time, next_el_time, 1),
                   1 if next_my_time > 0 => (*valve, *el_prev, next_my_time, *el_time, next_my_time, 0),
                   2 if next_el_time > 0 => (*my_prev, *valve, *my_time, next_el_time, next_el_time, 0),
                   _ => return None,
               };

               let flow = flow + t * next_flow;
               let mut left: Vec<_> = (0..valves.len()).filter(|idx| (released & 2 << idx) == 0).map(|idx| input[valves[idx]].0).collect();
               left.sort();
               let (_, _, max_flow) = left.into_iter().rev().fold((my_time - 2, el_time - 2, flow), |(my_time, el_time, flow), f| {
                   if my_time < 0 && el_time < 0 {
                       (my_time, el_time, flow)
                   } else if my_time > el_time {
                       (my_time - 2, el_time, flow + f * my_time)
                   } else {
                       (my_time, el_time - 2, flow + f * el_time)
                   }
               });
               if max_flow < best {
                   return None;
               }


               Some((my_next, el_next, flow, my_time, el_time, force, idx))
            }) else {
                break;
            };

            released |= 2 << added;
            stack.push((my_next, el_next, flow, my_time, el_time, next_force, added));
            start_from = 0;
            force = 0;
        }

        // Update on best solution
        let score = stack
            .last()
            .map(|(_, _, flow, _, _, _, _)| *flow)
            .unwrap_or(0);

        best = best.max(score);

        // Don't backtract if there is only one (root) element
        if stack.len() == 1 {
            break;
        }

        // Backtrack once
        let Some((_, _, _, _, _, next_force, added)) = stack.pop() else {
            break;
        };

        released &= !(2 << added);
        force = next_force;

        match force {
            0 => start_from = added + 1,
            _ => start_from = added,
        }
    }

    best
}

fn main() {
    let t0 = Instant::now();
    let input = input();
    let t1 = t0.elapsed();
    let (input, start) = preprocess(input);
    let dist_table = build_distance_table(&input);
    let t2 = t0.elapsed();

    println!("Part 1: {}", part1(&input, &dist_table, start));
    let t3 = t0.elapsed();
    println!("Part 2: {}", part2(&input, &dist_table, start));
    let t4 = t0.elapsed();

    println!(
        "Times: input: {t1:?}, preprocess: {:?}, part1: {:?}, part2: {:?}, total: {:?}",
        t2 - t1,
        t3 - t2,
        t4 - t3,
        t4
    );
}
