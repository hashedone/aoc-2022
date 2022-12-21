use std::collections::HashMap;
use std::time::Instant;

// Shapes are "up side down" - low indexes determines bottom of the shape.
//
// Every row is u8 bitmask (left-to-right reading from low bits)
const SHAPES: [&[u8]; 5] = [
    // ####
    &[0b1111],
    // .#.
    // ###
    // .#.
    &[0b10, 0b111, 0b10],
    // ###
    // ..#
    // ..#
    &[0b111, 0b100, 0b100],
    // #
    // #
    // #
    // #
    &[0b1; 4],
    // ##
    // ##
    &[0b11; 2],
];

fn collision(shape: &[u8], surface: &[u8], x: usize, y: usize) -> bool {
    // Rigth edge check
    if shape.iter().any(|&row| (row << x) > 0b1111111) {
        return true;
    }

    // Taking lines from surface starting from y position of the shape, and then shifting the shape
    // discriminant by x position. If any lines pair described like that has `1` on the same bit,
    // that is a collision spot.
    shape
        .iter()
        .zip(&surface[y..])
        .any(|(s, srf)| (s << x) & srf > 0)
}

fn push(dir: char, shape: &[u8], surface: &[u8], x: usize, y: usize) -> usize {
    let newx = match dir {
        '>' => x + 1,
        '<' if x > 0 => x - 1,
        _ /* cannot move */ => return x,
    };

    // i - xoffset, j - yoffset
    // Colission detection - if there is part of a shape on this offset, and either:
    // * `newx + i` is out of surface
    // * there is already anything on `newx + i` on surface
    // we have a collision which means we cannot move. We return the old x.
    match collision(shape, surface, newx, y) {
        true => x,
        false => newx,
    }
}

// Returns new y coordinate, but also information if it changed, so the new brick should be spawned
fn fall(shape: &[u8], surface: &[u8], x: usize, y: usize) -> (usize, bool) {
    if y == 0 {
        return (0, true);
    }

    // i - xoffset, j - yoffset
    // Colission detection - if there is part of a shape on this offset, and either:
    // * `newx + i` is out of surface
    // * there is already anything on `newx + i` on surface
    // we have a collision which means we cannot move. We return the old x.
    match collision(shape, surface, x, y - 1) {
        true => (y, true),
        false => (y - 1, false),
    }
}

// Shape is blocked - fix it on the surface
fn fix(shape: &[u8], surface: &mut [u8], x: usize, y: usize) {
    for (s, srf) in shape.iter().zip(&mut surface[y..]) {
        *srf |= s << x;
    }
}

#[allow(unused)]
fn draw(shape: &[u8], surface: &[u8], x: usize, y: usize) {
    let h = surface.iter().position(|row| *row > 0).unwrap_or(0) + 8;
    let h = h.min(surface.len());

    // i - xoffset, j - yoffset
    for j in (0..h).rev() {
        for i in 0..7 {
            if surface[j] & (1 << i) > 0 {
                print!("#");
            } else if (x..x + 4).contains(&i)
                && (y..y + shape.len()).contains(&j)
                && (shape[j - y] << x) & (1 << i) > 0
            {
                print!("@");
            } else {
                print!(".");
            }
        }

        println!();
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct State {
    // boxed slice is a bit memory-cheaper than Vec, and we will never modify it
    // As box is a continous memory anyway, there is no benefit in packing it into some u128 or so
    //    board: Box<[u8]>,
    next_shape: usize,
    in_pos: usize,
    height: usize,
}

// impl State {
//     #[allow(unused)]
//     fn new(surface: &[u8], next_shape: usize, in_pos: usize) -> Self {
//         let i = surface
//             .iter()
//             .enumerate()
//             .rev()
//             .find_map(|(i, row)| Some(i).filter(|_| *row > 0))
//             .unwrap_or(0);
//
//         Self {
//             board: surface[..=i].into(),
//             next_shape,
//             in_pos,
//         }
//     }
// }

fn solve(input: &str) {
    const STEPS1: usize = 2022;
    const STEPS2: usize = 1000000000000;

    let t0 = Instant::now();
    let mut t1 = t0.elapsed();
    let mut t2 = t0.elapsed();

    let mut part1 = 0;
    let mut part2 = 0;

    // The playing surface. Alwyas 7-wide (by definition), height is extending while the tower is
    // growing. `false` means empty space, `true` is fixed rock.
    //
    // The row is of length 7 so we keep it as `u8` bitflag
    let mut surface = [0u8; 8000];
    let mut next_shape = 1;
    let mut shape = SHAPES[0];
    let mut x = 2;
    let mut y = 3;
    let mut fixed_cnt = 0;
    let mut removed_lines = 0;
    let mut top = 0;
    let mut bottom = 0;

    // Hash map of board states after fixing the shape. State contains the packed board (with no
    // empty lines above top) + the next shape to be spawned. The value of the map is how many lines
    // we removed when reaching the state, and the number of fixed rocks.
    //
    // If the state is reached again, it means we have the loop - we can calculate the cycle length
    // (current_rocks_fixed - state_rocks_fixed) and it is clear, that every `cycle_length` steps
    // the state would be repeated with `removed_lines` incremented by `state_removed_lines`.
    // let mut states: HashMap<State, (usize, usize)> = HashMap::new();
    let mut states: HashMap<State, (usize, usize)> = HashMap::new();

    for (idx, dir) in input.chars().enumerate().cycle() {
        // Note: (x, y) is bottom right current shape coordinate, and they are going bot-to-top,
        // and left-to-right.
        x = push(dir, shape, &surface[bottom..], x, y);
        let (newy, spawn) = fall(shape, &surface[bottom..], x, y);
        y = newy;

        if spawn {
            fix(shape, &mut surface[bottom..], x, y);
            top = top.max(y + shape.len());

            // Checking if I fill any line - if so, removing all lines below it (incliding found
            // line).
            if let Some(idx) = (0..shape.len())
                .rev()
                .find(|j| surface[bottom + y + j] == 0b1111111)
            {
                removed_lines += y + idx + 1;
                bottom += y + idx + 1;
                top -= y + idx + 1;
            }

            shape = SHAPES[next_shape];
            next_shape = (next_shape + 1) % SHAPES.len();
            x = 2;
            y = top + 3;

            // We might need to extend the surface if the tower grew (always keeping the 4 spaces
            // above for the next shape)
            //            surface.resize(bottom + y + shape.len(), 0);
            fixed_cnt += 1;

            //            let state = State::new(&surface, next_shape, idx);
            let state = State {
                next_shape,
                in_pos: idx,
                height: top,
            };

            // If instert returns `Some`, we met the state again - we can calculate the cycle
            if let Some((cycle_removed, cycle_fixed)) =
                states.insert(state, (removed_lines, fixed_cnt))
            {
                let cycle_length = fixed_cnt - cycle_fixed;
                let cycle_lines_removed = removed_lines - cycle_removed;

                let left = match fixed_cnt {
                    i if i < STEPS1 => STEPS1 - i,
                    _ => STEPS2 - fixed_cnt,
                };

                let cycles = left / cycle_length;

                if cycles > 0 {
                    println!("Jumping {cycles} forward on {idx} iteration while {fixed_cnt} fixed, {cycle_length} rocks per cycle, {removed_lines} lines per cycle");
                }
                // Just jump forward as much as we can, removing lines immediately
                fixed_cnt += cycles * cycle_length;
                removed_lines += cycles * cycle_lines_removed;
            }

            if fixed_cnt == STEPS1 {
                part1 = top + removed_lines;
                t1 = t0.elapsed();
            }

            if fixed_cnt >= STEPS2 {
                part2 = top + removed_lines;
                t2 = t0.elapsed();

                break;
            }
        }
    }

    println!("Part 1: {part1}, time: {t1:?}");
    println!("Part 2: {part2}, time: {:?}", t2 - t1);
    println!("Total time: {t2:?}");
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.chars().filter(|c| *c == '<' || *c == '>').collect();
    solve(&input);
}
