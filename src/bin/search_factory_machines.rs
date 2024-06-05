// I/O boilerplate //

pub struct UnsafeScanner<'a> {
    // not actually dead code, needed for buf_iter to work
    #[allow(dead_code)]
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitAsciiWhitespace<'a>,
}

impl UnsafeScanner<'_> {
    pub fn new<R: std::io::Read>(mut reader: R) -> Self {
        // note that even for stdin-heavy problems, allocating initial capacity is less efficient due to how read_to_end works
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
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let n: u32 = scan.token();
    let products: u64 = scan.token();
    let machines: Vec<u64> = (0..n).map(|_| scan.token()).collect();

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
    let scan = UnsafeScanner::new(std::io::stdin());
    let mut out = std::io::stdout().lock();
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
