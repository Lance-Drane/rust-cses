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

/// A factory has n machines which can be used to make products. Your goal is to make a total of t products.
///
/// For each machine, you know the number of seconds it needs to make a single product. The machines can work simultaneously, and you can freely decide their schedule.
///
/// What is the shortest time needed to make t products?
///
/// <b>Input</b>
///
/// The first input line has two integers n and t: the number of machines and products.
///
/// The next line has n integers k<sub>1</sub>,k<sub>2</sub>,...,k<sub>n</sub>: the time needed to make a product using each machine.
///
/// <b>Output</b>
///
/// Print one integer: the minimum time needed to make t products.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ t ≤ 10<sup>9</sup></li>
/// <li>1 ≤ k<sub>i</sub> ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');

    let n = unsafe { u32::to_posint(iter.next().unwrap_unchecked()) };
    let products = unsafe { u64::to_posint(iter.next().unwrap_unchecked()) };
    let machines: Vec<u64> = (0..n)
        .map(|_| unsafe { u64::to_posint(iter.next().unwrap_unchecked()) })
        .collect();

    let mut low = 0;
    let mut high = 1_000_000_000_000_000_000;
    'bsearch: while low <= high {
        let mid = (low + high) >> 1;
        let mut sum = 0;
        for machine in &machines {
            sum += mid / *machine;
            if sum >= products {
                high = mid - 1;
                continue 'bsearch;
            }
        }
        low = mid + 1;
    }

    writeln!(out, "{low}").unwrap();
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
3 7
3 2 5
";
        let target = b"\
8
";

        test(input, target);
    }

    #[test]
    fn test_example_2() {
        let input = b"\
3 8
3 2 7
";
        let target = b"\
9
";

        test(input, target);
    }

    #[test]
    fn test_small_products() {
        let input = b"\
6 1
8 7 1 5 4 1
";
        let target = b"\
1
";

        test(input, target);
    }

    #[test]
    fn test_small_products_2() {
        let input = b"\
6 3
8 7 1 5 4 1
";
        let target = b"\
2
";

        test(input, target);
    }
}
