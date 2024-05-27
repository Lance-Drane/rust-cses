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

const MODULO: u32 = 1_000_000_007;

/// Consider a money system consisting of n coins. Each coin has a positive integer value. Your task is to calculate the number of distinct ordered ways you can produce a money sum x using the available coins.
///
/// For example, if the coins are {2,3,5} and the desired sum is 9, there are 3 ways:
/// <ul>
/// <li>2+2+5</li>
/// <li>3+3+3</li>
/// <li>2+2+2+3</li>
/// </ul>
///
/// <b>Input</b>
///
/// The first input line has two integers n and x: the number of coins and the desired sum of money.
///
/// The second line has n distinct integers c1, c2, ..., c<sub>n</sub>: the value of each coin.
///
/// <b>Output</b>
///
/// Print one integer: the number of ways modulo 10<sup>9</sup> + 7.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 100</li>
/// <li>1 ≤ x ≤ 10<sup>6</sup></li>
/// <li>1 ≤ c<sub>i</sub> ≤ 10<sup>6</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let capacity: u8 = scan.token();
    let target: usize = scan.token();
    let mut cache = vec![0; target + 1];
    cache[0] = 1;

    for coin in (0..capacity).map(|_| scan.token::<usize>()) {
        let mut cache_cp = cache.as_mut_slice();
        while cache_cp.len() > coin {
            let (left, right) = cache_cp.split_at_mut(coin);
            for (a, b) in left.iter().zip(right.iter_mut()) {
                *b += *a;
                if *b >= MODULO {
                    *b -= MODULO;
                }
            }
            cache_cp = right;
        }
    }

    writeln!(out, "{}", cache[target]).unwrap();
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
3 9
2 3 5
";
        let target = b"\
3
";

        test(input, target);
    }
}
