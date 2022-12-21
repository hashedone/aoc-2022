use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::time::Instant;

fn parse_cost(l: &str) -> [i32; 4] {
    let ore = l
        .rfind("ore")
        .and_then(|i| {
            let s = l[..i].trim();
            s.rfind(' ').map(|i| s[i..].trim().parse().unwrap())
        })
        .unwrap_or(0);

    let clay = l
        .rfind("clay")
        .and_then(|i| {
            let s = l[..i].trim();
            s.rfind(' ').map(|i| s[i..].trim().parse().unwrap())
        })
        .unwrap_or(0);
    let obsidian = l
        .rfind("obsidian")
        .and_then(|i| {
            let s = l[..i].trim();
            s.rfind(' ').map(|i| s[i..].trim().parse().unwrap())
        })
        .unwrap_or(0);

    [ore, clay, obsidian, 0]
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Cache {
    resources: [u32; 4],
    bots: [u32; 4],
    time: u32,
}

fn main() {
    let blueprints = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split('.');
            let ore = parse_cost(parts.next().unwrap());
            let clay = parse_cost(parts.next().unwrap());
            let obsidian = parse_cost(parts.next().unwrap());
            let geode = parse_cost(parts.next().unwrap());

            [ore, clay, obsidian, geode]
        })
        .collect::<Vec<_>>();

    let mut cache = HashSet::new();
    let mut best24;
    let mut best32;
    let mut states = vec![];
    let mut p1 = 0;
    let mut p2 = 1;

    let t = Instant::now();
    for (i, bp) in blueprints.iter().enumerate() {
        cache.clear();
        best24 = 0;
        best32 = 0;
        states.push(((0, 0, 0, 0), [1, 0, 0, 0], 0));

        let m0 = *bp.iter().map(|[b0, _, _, _]| b0).max().unwrap();
        let m1 = *bp.iter().map(|[_, b1, _, _]| b1).max().unwrap();
        let m2 = *bp.iter().map(|[_, _, b2, _]| b2).max().unwrap();
        let m = [m0, m1, m2];
        while let Some(((r0, r1, r2, r3), [i0, i1, i2, i3], t)) = states.pop() {
            //       println!("{r0} {r1} {r2} {r3} / {i0} {i1} {i2} {i3} / {t}");
            if t == 24 {
                best24 = best24.max(r3);

                if i >= 3 {
                    continue;
                }
            }

            if t == 32 {
                best32 = best32.max(r3);
                continue;
            }

            if (r3 + (25 - t) * i3 + (24 - t) * (25 - t) / 2) <= best24
                && (r3 + (33 - t) * i3 + (32 - t) * (33 - t) / 2) <= best32
            {
                // theoretical best is too bad, ignore
                continue;
            }

            let i0 = i0.min(m0);
            let i1 = i1.min(m1);
            let i2 = i2.min(m2);

            let r0 = r0.min(t * m0 - (t - 1) * i0);
            let r1 = r1.min(t * m1 - (t - 1) * i1);
            let r2 = r2.min(t * m2 - (t - 1) * i2);

            if !cache.insert(((r0, r1, r2, r3), [i0, i1, i2, i3], t)) {
                continue;
            }

            for (i, [c0, c1, c2, c3]) in bp.iter().copied().enumerate().rev() {
                if c0 <= r0
                    && c1 <= r1
                    && c2 <= r2
                    && c3 <= r3
                    && (i == 3 || m[i] > [i0, i1, i2, i3][i])
                {
                    let mut bots = [i0, i1, i2, i3];
                    bots[i] += 1;
                    let state = (
                        (r0 - c0 + i0, r1 - c1 + i1, r2 - c2 + i2, r3 - c3 + i3),
                        bots,
                        t + 1,
                    );

                    states.push(state);
                }
            }

            let state = (
                (r0 + i0, r1 + i1, r2 + i2, r3 + i3),
                [i0, i1, i2, i3],
                t + 1,
            );
            states.push(state);
        }

        p1 += (i as i32 + 1) * best24;
        if i < 3 {
            p2 *= best32;
        }
    }

    println!("Part1: {p1}");
    println!("Part2: {p2}");
    println!("Time: {:?}", t.elapsed());
}
