// I/O boilerplate //

pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitAsciiWhitespace<'static>,
}

impl<R: std::io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    /// Use "turbofish" syntax `token::<T>()` to select data type of next token.
    ///
    /// # Panics
    /// Panics if there's an I/O error or if the token cannot be parsed as T.
    pub fn token<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = std::str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
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
fn solve<R: std::io::BufRead, W: std::io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
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
    let mut scan = UnsafeScanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    solve(&mut scan, &mut out);
}

#[cfg(test)]
mod test {
    use super::*;

    fn test(input: &[u8], target: &[u8]) {
        let mut scan = UnsafeScanner::new(input);
        let mut out = Vec::with_capacity(target.len());
        solve(&mut scan, &mut out);

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
