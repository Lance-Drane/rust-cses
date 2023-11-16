// I/O boilerplate //

pub struct UnsafeScanner {
    // not actually dead code, needed for buf_iter to work
    #[allow(dead_code)]
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitAsciiWhitespace<'static>,
}

impl UnsafeScanner {
    pub fn new<R: std::io::BufRead>(mut reader: R) -> Self {
        let mut buf_str = vec![];
        unsafe {
            reader.read_to_end(&mut buf_str).unwrap_unchecked();
        }
        let buf_iter = unsafe {
            let slice = std::str::from_utf8_unchecked(&buf_str);
            std::mem::transmute(slice.split_ascii_whitespace())
        };
        // optional memory clear
        buf_str.clear();

        Self { buf_str, buf_iter }
    }

    /// Use "turbofish" syntax `token::<T>()` to select data type of next token.
    ///
    /// # Panics
    /// Panics if there's an I/O error or if the token cannot be parsed as T.
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

const MODULO: u64 = 1_000_000_007;

/// Consider a money system consisting of n coins. Each coin has a positive integer value. Your task is to produce a sum of money x using the available coins in such a way that the number of coins is minimal.
///
/// For example, if the coins are {2,3,5} and the desired sum is 9, there are 8 ways:
/// <ul>
/// <li>2+2+5</li>
/// <li>2+5+2</li>
/// <li>5+2+2</li>
/// <li>3+3+3</li>
/// <li>2+2+2+3</li>
/// <li>2+2+3+2</li>
/// <li>2+3+2+2</li>
/// <li>3+2+2+2</li>
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
    let mut coins: Vec<usize> = (0..capacity).map(|_| scan.token::<usize>()).collect();
    coins.sort_unstable_by(|a, b| b.cmp(a));
    let mut cache = vec![0_u64; target + 1];
    cache[0] = 1;

    for idx in *coins.last().unwrap()..=target {
        cache[idx] = coins
            .iter()
            .skip_while(|coin| **coin > idx)
            .map(|coin| cache[idx - coin])
            .sum::<u64>()
            % MODULO;
    }

    writeln!(out, "{}", cache[target]).ok();
}

// entrypoints //

fn main() {
    let scan = UnsafeScanner::new(std::io::stdin().lock());
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
8
";

        test(input, target);
    }

    #[test]
    fn test_overlap() {
        let input = b"\
12 74057
1 2 74012 74005 74003 73999 73998 73997 73996 73995 73994 73993
";
        let target = b"\
0
";

        test(input, target);
    }

    #[test]
    fn test_overflow() {
        let input = b"\
100 1000
389 101 552 795 876 269 887 103 154 689 542 920 128 541 44 657 310 531 656 567 386 536 900 374 929 505 255 376 384 709 311 404 699 86 512 518 321 916 408 935 568 662 731 933 238 331 833 235 423 352 205 669 413 152 695 713 878 614 109 164 387 3 287 823 485 716 556 323 924 57 35 705 643 77 200 944 768 490 589 339 701 190 714 349 252 303 74 526 186 644 453 251 429 170 777 216 22 825 514 379
";
        let target = b"\
834994040
";

        test(input, target);
    }
}
