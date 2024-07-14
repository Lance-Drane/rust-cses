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

/// You are in a book shop which sells n different books. You know the price and number of pages of each book.
///
/// You have decided that the total price of your purchases will be at most x. What is the maximum number of pages you can buy? You can buy each book at most once.
///
/// <b>Input</b>
///
/// The first input line contains two integers n and x: the number of books and the maximum total price.
///
/// The next line contains n integers h<sub>1</sub>,h<sub>2</sub>,...,h<sub>n</sub>: the price of each book.
///
/// The next line contains n integers s<sub>1</sub>,s<sub>2</sub>,...,s<sub>n</sub>: the number of pages of each book.
///
/// <b>Output</b>
///
/// Print one integer: the maximum number of pages.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 1000</li>
/// <li>1 ≤ x ≤ 10<sup>5</sup></li>
/// <li>1 ≤ h<sub>i</sub>,s<sub>i</sub> ≤ 10<sup>6</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');

    let n = unsafe { u16::to_posint(iter.next().unwrap_unchecked()) };
    let max_price = unsafe { usize::to_posint(iter.next().unwrap_unchecked()) };

    let prices: Vec<usize> = (0..n)
        .map(|_| unsafe { usize::to_posint(iter.next().unwrap_unchecked()) })
        .collect();

    let mut cache = vec![0; max_price + 1];

    for price in prices {
        let page = unsafe { u32::to_posint(iter.next().unwrap_unchecked()) };
        let mut cache_cp = cache.as_mut_slice();
        while cache_cp.len() > price {
            let (left, right) = cache_cp.split_at_mut(price);
            for (a, b) in left.iter_mut().zip(right.iter()) {
                *a = (*b + page).max(*a);
            }
            cache_cp = right;
        }
    }

    writeln!(out, "{}", cache[0]).unwrap();
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
4 10
4 8 5 3
5 12 8 1
";
        let target = b"\
13
";

        test(input, target);
    }
}
