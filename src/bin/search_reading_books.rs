// I/O boilerplate

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

/// There are n books, and Kotivalo and Justiina are going to read them all. For each book, you know the time it takes to read it.
///
/// They both read each book from beginning to end, and they cannot read a book at the same time. What is the minimum total time required?
///
/// <b>Input</b>
///
/// The first input line has an integer n: the number of books.
///
/// The second line has n integers t<sub>1</sub>,t<sub>2</sub>,...,t<sub>n</sub>: the time required to read each book.
///
/// <b>Output</b>
///
/// Print one integer: the minimum total time.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ t<sub>i</sub> ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');

    let n = unsafe { u32::to_posint(iter.next().unwrap_unchecked()) };
    let mut max = 0;
    let mut sum = 0;

    for int in (0..n).map(|_| unsafe { u64::to_posint(iter.next().unwrap_unchecked()) }) {
        sum += int;
        max = max.max(int);
    }

    writeln!(out, "{}", sum.max(max << 1)).unwrap();
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
3
2 8 3
";
        let target = b"\
16
";

        test(input, target);
    }

    #[test]
    fn test_no_delay() {
        let input = b"\
4
2 8 3 4
";
        let target = b"\
17
";

        test(input, target);
    }
}
