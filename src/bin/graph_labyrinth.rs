// I/O boilerplate //

pub struct UnsafeScanner<'a> {
    // not actually dead code, needed for buf_iter to work
    #[allow(dead_code)]
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitAsciiWhitespace<'a>,
}

impl UnsafeScanner<'_> {
    pub fn new<R: std::io::Read>(mut reader: R) -> Self {
        let mut buf_str = vec![];
        unsafe {
            reader.read_to_end(&mut buf_str).unwrap_unchecked();
        }
        let buf_iter = unsafe {
            let slice = std::str::from_utf8_unchecked(&buf_str);
            std::mem::transmute(slice.split_ascii_whitespace())
        };

        Self { buf_str, buf_iter }
    }

    /// Use "turbofish" syntax `token::<T>()` to select data type of next token.
    ///
    /// # Panics
    /// Panics if there's no more tokens or if the token cannot be parsed as T.
    pub fn token<T: std::str::FromStr>(&mut self) -> T {
        unsafe {
            self.buf_iter
                .next()
                .unwrap_unchecked()
                .parse()
                .unwrap_unchecked()
        }
    }
}

// problem //

use std::collections::VecDeque;

/// You are given a map of a labyrinth, and your task is to find a path from start to end. You can walk left, right, up and down.
///
/// <b>Input</b>
///
/// The first input line has two integers n and m: the height and width of the map.
///
/// Then there are n lines of m characters describing the labyrinth. Each character is . (floor), # (wall), A (start), or B (end). There is exactly one A and one B in the input.
///
/// <b>Output</b>
///
/// First print "YES", if there is a path, and "NO" otherwise.
///
/// If there is a path, print the length of the shortest such path and its description as a string consisting of characters L (left), R (right), U (up), and D (down). You can print any valid solution.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n,m ≤ 1000</li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let height: usize = scan.token();
    let width: usize = scan.token();

    // boundaries for 1D array
    let upper_height_bound = height * width - width;
    let upper_width_bound = width - 1;

    let mut grid = [0; 1_000_000];
    for (idx, row) in (0..height).map(|idx| (idx, scan.token::<String>().into_bytes())) {
        grid[(width * idx)..(width * (idx + 1))].copy_from_slice(&row);
    }

    let start = unsafe { grid.iter().position(|&x| x == b'A').unwrap_unchecked() };

    let mut queue = VecDeque::from([start]);
    let mut end = usize::MAX; // initialize to non-existant index in the grid

    while let Some(curr_idx) = queue.pop_front() {
        // top
        if curr_idx >= width {
            let next_idx = curr_idx - width;
            match grid[next_idx] {
                b'.' => {
                    grid[next_idx] = b'U';
                    queue.push_back(next_idx);
                }
                b'B' => {
                    grid[next_idx] = b'U';
                    end = next_idx;
                    break;
                }
                _ => {}
            }
        }

        // right
        if curr_idx % width != upper_width_bound {
            let next_idx = curr_idx + 1;
            match grid[next_idx] {
                b'.' => {
                    grid[next_idx] = b'R';
                    queue.push_back(next_idx);
                }
                b'B' => {
                    grid[next_idx] = b'R';
                    end = next_idx;
                    break;
                }
                _ => {}
            }
        }

        // bottom
        if curr_idx < upper_height_bound {
            let next_idx = curr_idx + width;
            match grid[next_idx] {
                b'.' => {
                    grid[next_idx] = b'D';
                    queue.push_back(next_idx);
                }
                b'B' => {
                    grid[next_idx] = b'D';
                    end = next_idx;
                    break;
                }
                _ => {}
            }
        }

        // left
        if curr_idx % width != 0 {
            let next_idx = curr_idx - 1;
            match grid[next_idx] {
                b'.' => {
                    grid[next_idx] = b'L';
                    queue.push_back(next_idx);
                }
                b'B' => {
                    grid[next_idx] = b'L';
                    end = next_idx;
                    break;
                }
                _ => {}
            }
        }
    }

    if end == usize::MAX {
        out.write(b"NO\n").ok();
    } else {
        let mut letter = grid[end];
        let mut path = vec![];
        while letter != b'A' {
            path.push(letter);
            end = match letter {
                b'U' => end + width,
                b'R' => end - 1,
                b'D' => end - width,
                // it's always going to be 'L' at this point
                _ => end + 1,
            };
            letter = unsafe { *grid.get_unchecked(end) };
        }
        path.reverse();
        write!(out, "YES\n{}\n", path.len()).ok();
        out.write(&path).ok();
        out.write(b"\n").ok();
    }
}

// entrypoints //

fn main() {
    let scan = UnsafeScanner::new(std::io::stdin());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    solve(scan, &mut out);
}

#[cfg(test)]
mod test {
    use super::*;

    fn test(input: &[u8], target: &[u8]) {
        let scan = UnsafeScanner::new(input);
        let mut out = Vec::with_capacity(target.len());
        solve(scan, &mut out);

        assert_eq!(out, target);
    }

    #[test]
    fn test_example() {
        let input = b"\
5 8
########
#.A#...#
#.##.#B#
#......#
########
";
        let target = b"\
YES
9
LDDRRRRRU
";

        test(input, target);
    }

    #[test]
    fn test_no() {
        let input = b"\
5 8
########
#.A#..##
#.##.#B#
#.....##
########
";
        let target = b"\
NO
";

        test(input, target);
    }

    #[test]
    fn test_open_room() {
        let input = b"\
5 5
.....
.B...
..##.
..#A.
.....
";
        let target = b"\
YES
6
RUULLL
";

        test(input, target);
    }
}
