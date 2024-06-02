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

// These numbers can be cached here, as you get the same values everytime.
//
// First number = query digit breakpoint (10<sup>n</sup>)
// Second number = starting sum of next breakpoint (for example: the three-digit numbers start at position 190)
const BREAKPOINTS: [(usize, usize); 19] = [
    (0, 0),
    (10, 10),
    (100, 190),
    (1_000, 2_890),
    (10_000, 38_890),
    (100_000, 488_890),
    (1_000_000, 5_888_890),
    (10_000_000, 68_888_890),
    (100_000_000, 788_888_890),
    (1_000_000_000, 8_888_888_890),
    (10_000_000_000, 98_888_888_890),
    (100_000_000_000, 1_088_888_888_890),
    (1_000_000_000_000, 11_888_888_888_890),
    (10_000_000_000_000, 128_888_888_888_890),
    (100_000_000_000_000, 1_388_888_888_888_890),
    (1_000_000_000_000_000, 14_888_888_888_888_890),
    (10_000_000_000_000_000, 158_888_888_888_888_890),
    (100_000_000_000_000_000, 1_688_888_888_888_888_890),
    (1_000_000_000_000_000_000, 17_888_888_888_888_888_890),
];

/// Consider an infinite string that consists of all positive integers in increasing order:
///
/// 12345678910111213141516171819202122232425...
///
/// Your task is to process q queries of the form: what is the digit at position k in the string?
///
/// <b>Input</b>
///
/// The first input line has an integer q: the number of queries.
///
/// After this, there are q lines that describe the queries. Each line has an integer k: a 1-indexed position in the string.
///
/// <b>Output</b>
///
/// For each query, print the corresponding digit.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ q ≤ 1000</li>
/// <li>1 ≤ k ≤ 10<sup>18</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let queries: u16 = scan.token();
    for _ in 0..queries {
        let query_position: usize = scan.token();

        let (num_digits, (start_target, start_position)) = unsafe {
            BREAKPOINTS
                .iter()
                .enumerate()
                .rfind(|(_, x)| x.1 <= query_position)
                .map(|(idx, x)| (idx + 1, x))
                .unwrap_unchecked()
        };
        let target_number = start_target + (query_position - start_position) / num_digits;
        let nth_digit = num_digits - (query_position - start_position) % num_digits - 1;
        #[allow(clippy::cast_possible_truncation)]
        writeln!(
            out,
            "{}",
            (target_number / (10_usize.pow(nth_digit as u32))) % 10
        )
        .unwrap();
    }
}

// entrypoints //

fn main() {
    let scan = UnsafeScanner::new(std::io::stdin());
    let mut out = std::io::BufWriter::with_capacity(32_768, std::io::stdout().lock());
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
3
7
19
12
";
        let target = b"\
7
4
1
";

        test(input, target);
    }

    #[test]
    fn test_bounds() {
        let input = b"\
22
1
9
10
11
187
188
189
190
191
192
193
2886
2887
2888
2889
2890
2891
2892
2893
2894
999999999999999999
1000000000000000000
";
        let target = b"\
1
9
1
0
8
9
9
1
0
0
1
8
9
9
9
1
0
0
0
1
5
3
";

        test(input, target);
    }
}
