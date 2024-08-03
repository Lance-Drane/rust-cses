// I/O boilerplate //

use std::io::Read;

pub trait PosInt {
    fn to_posint(buf: &[u8]) -> Self;
}

macro_rules! impl_int {
    (for $($t:ty),+) => {
        $(impl PosInt for $t {
            #[allow(clippy::cast_lossless, clippy::cast_possible_wrap)]
            fn to_posint(buf: &[u8]) -> Self {
                unsafe {
                    buf.iter()
                        .map(|byte| (byte & 15) as $t)
                        .reduce(|acc, digit| acc * 10 + digit)
                        .unwrap_unchecked()
                }
            }
        })*
    }
}
impl_int!(for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

// problem //

/// You are given a map of a building, and your task is to count the number of its rooms. The size of the map is n×mn \times mn×m squares, and each square is either floor or wall. You can walk left, right, up, and down through the floor squares.
///
/// <b>Input</b>
///
/// The first input line has two integers n and m: the height and width of the map.
///
/// Then there are nnn lines of mmm characters describing the map. Each character is either . (floor) or # (wall).
///
/// <b>Output</b>
///
/// Print one integer: the number of rooms.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n,m ≤ 1000</li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');

    let height = unsafe { usize::to_posint(iter.next().unwrap_unchecked()) };
    let width = unsafe { usize::to_posint(iter.next().unwrap_unchecked()) };

    // boundaries for 1D array
    let upper_height_bound = height * width - width;
    let upper_width_bound = width - 1;

    let mut grid = [0; 1_000_000];
    for (idx, row) in (0..height).map(|idx| (idx, unsafe { iter.next().unwrap_unchecked() })) {
        grid[(width * idx)..(width * (idx + 1))].copy_from_slice(row);
    }

    let mut counter = 0_u32;

    for idx in 0..(height * width) {
        // switch cells of newly visited nodes from '.' to '#'.
        if grid[idx] == b'#' {
            continue;
        }

        grid[idx] = b'#';
        let mut stack = vec![idx];
        while let Some(curr_idx) = stack.pop() {
            // top
            if curr_idx >= width {
                let next_idx = curr_idx - width;
                if grid[next_idx] == b'.' {
                    grid[next_idx] = b'#';
                    stack.push(next_idx);
                }
            }

            // right
            if curr_idx % width != upper_width_bound {
                let next_idx = curr_idx + 1;
                if grid[next_idx] == b'.' {
                    grid[next_idx] = b'#';
                    stack.push(next_idx);
                }
            }

            // bottom
            if curr_idx < upper_height_bound {
                let next_idx = curr_idx + width;
                if grid[next_idx] == b'.' {
                    grid[next_idx] = b'#';
                    stack.push(next_idx);
                }
            }

            // left
            if curr_idx % width != 0 {
                let next_idx = curr_idx - 1;
                if grid[next_idx] == b'.' {
                    grid[next_idx] = b'#';
                    stack.push(next_idx);
                }
            }
        }

        counter += 1;
    }

    writeln!(out, "{counter}").unwrap();
}

// entrypoints //

fn main() {
    let mut buf_str = vec![];
    std::io::stdin().lock().read_to_end(&mut buf_str).unwrap();
    let mut out = std::io::stdout().lock();
    solve(&buf_str, &mut out);
}

#[cfg(test)]
mod test {
    use super::*;

    fn test(input: &[u8], target: &[u8]) {
        let mut out = Vec::with_capacity(target.len());
        solve(input, &mut out);

        assert_eq!(out, target);
    }

    #[test]
    fn test_example() {
        let input = b"\
5 8
########
#..#...#
####.#.#
#..#...#
########
";
        let target = b"\
3
";

        test(input, target);
    }

    #[test]
    fn test_example_2() {
        let input = b"\
3 10
..........
#########.
..........
";
        let target = b"\
1
";

        test(input, target);
    }

    #[test]
    fn test_example_3() {
        let input = b"\
3 10
..........
.#########
..........
";
        let target = b"\
1
";

        test(input, target);
    }
}
