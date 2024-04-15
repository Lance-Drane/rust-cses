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

/// The Tower of Hanoi game consists of three stacks (left, middle and right) and n round disks of different sizes. Initially, the left stack has all the disks, in increasing order of size from top to bottom.
///
/// The goal is to move all the disks to the right stack using the middle stack. On each move you can move the uppermost disk from a stack to another stack. In addition, it is not allowed to place a larger disk on a smaller disk.
///
/// Your task is to find a solution that minimizes the number of moves.
///
/// <b>Input</b>
///
/// The only input line has an integer n: the number of disks.
///
/// <b>Output</b>
///
/// First print an integer k: the minimum number of moves.
///
/// After this, print k lines that describe the moves. Each line has two integers a and b: you move a disk from stack a to stack b.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 16</li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let discs = scan.token::<u8>();

    writeln!(out, "{}", (1 << discs) - 1).ok();
    recurse(out, b'1', b'3', b'2', discs);
}

// supporting recursive function, just swap the positions around
fn recurse<W: std::io::Write>(out: &mut W, from: u8, to: u8, swap: u8, disc: u8) {
    if disc == 1 {
        out.write_all(&[from, b' ', to, b'\n']).ok();
        return;
    }
    recurse(out, from, swap, to, disc - 1);
    out.write_all(&[from, b' ', to, b'\n']).ok();
    recurse(out, swap, to, from, disc - 1);
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
2
";
        let target = b"\
3
1 2
1 3
2 3
";

        test(input, target);
    }

    #[test]
    fn test_example_short() {
        let input = b"\
1
";
        let target = b"\
1
1 3
";

        test(input, target);
    }

    #[test]
    fn test_example_odd() {
        let input = b"\
3
";
        let target = b"\
7
1 3
1 2
3 2
1 3
2 1
2 3
1 3
";

        test(input, target);
    }

    #[test]
    fn test_example_even() {
        let input = b"\
4
";
        let target = b"\
15
1 2
1 3
2 3
1 2
3 1
3 2
1 2
1 3
2 3
2 1
3 1
2 3
1 2
1 3
2 3
";

        test(input, target);
    }

    #[test]
    fn test_example_five() {
        let input = b"\
5
";
        let target = b"\
31
1 3
1 2
3 2
1 3
2 1
2 3
1 3
1 2
3 2
3 1
2 1
3 2
1 3
1 2
3 2
1 3
2 1
2 3
1 3
2 1
3 2
3 1
2 1
2 3
1 3
1 2
3 2
1 3
2 1
2 3
1 3
";

        test(input, target);
    }
}
