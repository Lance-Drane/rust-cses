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

/// You are given an array containing n integers. Your task is to determine the longest increasing subsequence in the array, i.e., the longest subsequence where every element is larger than the previous one.
///
/// A subsequence is a sequence that can be derived from the array by deleting some elements without changing the order of the remaining elements.
///
/// <b>Input</b>
///
/// The first line contains an integer n: the size of the array.
///
/// After this there are n integers x<sub>1</sub>,x<sub>2</sub>,...,x<sub>n</sub>: the contents of the array.
///
/// <b>Output</b>
///
/// Print the length of the longest increasing subsequence.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ k<sub>i</sub> ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');

    let n: u32 = unsafe { u32::to_posint(iter.next().unwrap_unchecked()) };
    let mut elements = Vec::with_capacity(n as usize);

    for item in (0..n).map(|_| unsafe { u32::to_posint(iter.next().unwrap_unchecked()) }) {
        match item.cmp(elements.last().unwrap_or(&0)) {
            std::cmp::Ordering::Less => {
                // find lower bound, it will always exist
                let mut low = 0;
                let mut high = elements.len();
                let mut size = high;
                while low < high {
                    let mid = low + (size >> 1);
                    if unsafe { *elements.get_unchecked(mid) } < item {
                        low = mid + 1;
                    } else {
                        high = mid;
                    }
                    size = high - low;
                }
                unsafe { *elements.get_unchecked_mut(low) = item };
            }
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => {
                elements.push(item);
            }
        }
    }

    writeln!(out, "{}", elements.len()).unwrap();
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
8
7 3 5 3 6 2 9 8
";
        let target = b"\
4
";

        test(input, target);
    }

    #[test]
    fn test_repeats() {
        let input = b"\
4
1 1 1 1
";
        let target = b"\
1
";

        test(input, target);
    }

    #[test]
    fn test_shifts() {
        let input = b"\
10
3 8 3 8 1 5 10 5 8 10
";
        let target = b"\
4
";

        test(input, target);
    }
}
