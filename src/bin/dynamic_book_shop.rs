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
    /// Panics if there are no more tokens or if the token cannot be parsed as T.
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
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let n: u16 = scan.token();
    let max_price: usize = scan.token();

    let prices: Vec<usize> = (0..n).map(|_| scan.token()).collect();

    let mut cache = vec![0; max_price + 1];

    for price in prices {
        let page: u32 = scan.token();
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
