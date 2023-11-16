// I/O boilerplate //

pub struct UnsafeScanner {
    // not actually dead code, needed for buf_iter to work
    #[allow(dead_code)]
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitAsciiWhitespace<'static>,
}

impl UnsafeScanner {
    pub fn new<R: std::io::BufRead>(mut reader: R) -> Self {
        let mut buf_str = vec![];
        unsafe {
            reader.read_to_end(&mut buf_str).unwrap_unchecked();
        }
        let buf_iter = unsafe {
            let slice = std::str::from_utf8_unchecked(&buf_str);
            std::mem::transmute(slice.split_ascii_whitespace())
        };
        // optional memory clear
        buf_str.clear();

        Self { buf_str, buf_iter }
    }

    /// Use "turbofish" syntax `token::<T>()` to select data type of next token.
    ///
    /// # Panics
    /// Panics if there's an I/O error or if the token cannot be parsed as T.
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

    writeln!(out, "{counter}").ok();
}

// entrypoints //

fn main() {
    let scan = UnsafeScanner::new(std::io::stdin().lock());
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
