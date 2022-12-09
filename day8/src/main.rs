use std::io::BufRead;

fn input(input: impl BufRead) -> (Vec<u8>, usize) {
    let mut rows = 0;

    let m = input
        .lines()
        .inspect(|_| rows += 1)
        .flat_map(|l| l.unwrap().into_bytes().into_iter().map(|b| b - b'0'))
        .collect();

    (m, rows)
}

fn idx(x: usize, y: usize, w: usize) -> usize {
    y * w + x
}

fn part1(m: &[u8], rows: usize) -> usize {
    let cols = m.len() / rows;
    let idx = |x, y| idx(x, y, cols);

    m.iter()
        .enumerate()
        .filter(|(i, &h)| {
            let (x, y) = (i % rows, i / rows);

            let l = (0..x).rev().all(|i| m[idx(i, y)] < h);
            let r = (x + 1..cols).all(|i| m[idx(i, y)] < h);
            let t = (0..y).rev().all(|j| m[idx(x, j)] < h);
            let b = (y + 1..rows).all(|j| m[idx(x, j)] < h);

            [l, r, t, b].iter().any(|&b| b)
        })
        .count()
}

fn part2(m: &[u8], rows: usize) -> usize {
    let cols = m.len() / rows;
    let idx = |x, y| idx(x, y, cols);

    m.iter()
        .enumerate()
        .map(|(i, &h)| {
            let (x, y) = (i % rows, i / rows);

            let l = (0..x)
                .rev()
                .position(|i| m[idx(i, y)] >= h)
                .map(|i| i + 1)
                .unwrap_or(x);
            let r = (x + 1..cols)
                .position(|i| m[idx(i, y)] >= h)
                .map(|i| i + 1)
                .unwrap_or(cols - x - 1);
            let t = (0..y)
                .rev()
                .position(|j| m[idx(x, j)] >= h)
                .map(|i| i + 1)
                .unwrap_or(y);
            let b = (y + 1..rows)
                .position(|j| m[idx(x, j)] >= h)
                .map(|i| i + 1)
                .unwrap_or(rows - y - 1);

            [l, r, t, b].into_iter().product()
        })
        .max()
        .unwrap_or(0)
}

fn main() {
    let input = input(std::io::stdin().lock());
    println!("Part 1: {}", part1(&input.0, input.1));
    println!("Part 2: {}", part2(&input.0, input.1));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn input_test() {
        let data = r#"30373
25512
65332
33549
35390"#;

        let expected = vec![
            3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
        ];

        assert_eq!(input(data.as_bytes()), (expected, 5));
    }

    #[test]
    fn part1_test() {
        let data = r#"30373
25512
65332
33549
35390"#;

        let data = input(data.as_bytes());
        assert_eq!(part1(&data.0, data.1), 21);
    }

    #[test]
    fn part2_test() {
        let data = r#"30373
25512
65332
33549
35390"#;

        let data = input(data.as_bytes());
        assert_eq!(part2(&data.0, data.1), 8);
    }
}
