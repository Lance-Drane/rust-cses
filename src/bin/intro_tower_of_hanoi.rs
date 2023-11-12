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
fn solve<R: std::io::BufRead, W: std::io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let discs = scan.token::<u8>();

    writeln!(out, "{}", (1 << discs) - 1).ok();
    recurse(out, b'1', b'3', b'2', discs);
}

// supporting recursive function, just swap the positions around
fn recurse<W: std::io::Write>(out: &mut W, from: u8, to: u8, swap: u8, disc: u8) {
    if disc == 1 {
        out.write(&[from, b' ', to, b'\n']).ok();
        return;
    }
    recurse(out, from, swap, to, disc - 1);
    out.write(&[from, b' ', to, b'\n']).ok();
    recurse(out, swap, to, from, disc - 1);
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

    #[test]
    fn test_example() {
        let input: &[u8] = b"\
2
";
        let target: &[u8] = b"\
3
1 2
1 3
2 3
";

        let mut scan = UnsafeScanner::new(input);
        let mut out = Vec::with_capacity(target.len());
        solve(&mut scan, &mut out);

        assert_eq!(out, target);
    }

    #[test]
    fn test_example_short() {
        let input: &[u8] = b"\
1
";
        let target: &[u8] = b"\
1
1 3
";

        let mut scan = UnsafeScanner::new(input);
        let mut out = Vec::with_capacity(target.len());
        solve(&mut scan, &mut out);

        assert_eq!(out, target);
    }

    #[test]
    fn test_example_odd() {
        let input: &[u8] = b"\
3
";
        let target: &[u8] = b"\
7
1 3
1 2
3 2
1 3
2 1
2 3
1 3
";

        let mut scan = UnsafeScanner::new(input);
        let mut out = Vec::with_capacity(target.len());
        solve(&mut scan, &mut out);

        assert_eq!(out, target);
    }

    #[test]
    fn test_example_even() {
        let input: &[u8] = b"\
4
";
        let target: &[u8] = b"\
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

        let mut scan = UnsafeScanner::new(input);
        let mut out = Vec::with_capacity(target.len());
        solve(&mut scan, &mut out);

        assert_eq!(out, target);
    }

    #[test]
    fn test_example_five() {
        let input: &[u8] = b"\
5
";
        let target: &[u8] = b"\
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

        let mut scan = UnsafeScanner::new(input);
        let mut out = Vec::with_capacity(target.len());
        solve(&mut scan, &mut out);

        assert_eq!(out, target);
    }
}
