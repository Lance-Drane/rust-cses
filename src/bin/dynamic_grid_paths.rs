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
            std::mem::transmute::<std::str::SplitAsciiWhitespace<'_>, std::str::SplitAsciiWhitespace<'_>>(slice.split_ascii_whitespace())
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

const MODULO: u32 = 1_000_000_007;

/// Consider an n * n grid whose squares may have traps. It is not allowed to move to a square with a trap.
///
/// Your task is to calculate the number of paths from the upper-left square to the lower-right square. You can only move right or down.
///
/// <b>Input</b>
///
/// The first input line has an integer n: the size of the grid.
///
/// After this, there are n lines that describe the grid. Each line has n characters: . denotes an empty cell, and * denotes a trap.
///
/// <b>Output</b>
///
/// Print the number of paths modulo 10^9+7.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 1000</li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let dimension: usize = scan.token();
    let size = dimension * dimension;
    let mut counters = vec![0_u32; size];

    for (i, point) in scan.token::<String>().into_bytes().into_iter().enumerate() {
        if point == b'*' {
            break;
        }
        unsafe {
            *counters.get_unchecked_mut(i) = 1;
        }
    }

    if counters[0] == 0 {
        out.write_all(b"0\n").unwrap();
        return;
    }
    let mut first_col_terminate = false;

    for (i, row) in (dimension..size)
        .step_by(dimension)
        .map(|idx| (idx, scan.token::<String>().into_bytes()))
    {
        if !first_col_terminate {
            if unsafe { *row.get_unchecked(0) } == b'*' {
                first_col_terminate = true;
            } else {
                unsafe {
                    *counters.get_unchecked_mut(i) = 1;
                }
            }
        }
        for (j, _) in row
            .into_iter()
            .enumerate()
            .skip(1)
            .filter(|(_, c)| *c == b'.')
        {
            let idx = i + j;
            let mut count = unsafe {
                counters.get_unchecked(idx - 1) + counters.get_unchecked(idx - dimension)
            };
            if count >= MODULO {
                count -= MODULO;
            }
            unsafe {
                *counters.get_unchecked_mut(idx) = count;
            }
        }
    }

    writeln!(out, "{}", counters[size - 1]).unwrap();
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
4
....
.*..
...*
*...
";
        let target = b"\
3
";

        test(input, target);
    }

    #[test]
    fn test_one() {
        let input = b"\
1
.
";
        let target = b"\
1
";

        test(input, target);
    }

    #[test]
    fn test_start_trap() {
        let input = b"\
2
*.
..
";
        let target = b"\
0
";

        test(input, target);
    }
}
