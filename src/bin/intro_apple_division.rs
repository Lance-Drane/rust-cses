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

/// There are n apples with known weights. Your task is to divide the apples into two groups so that the difference between the weights of the groups is minimal.
///
/// <b>Input</b>
///
/// The first input line has an integer n: the number of apples.
///
/// The next line has n integers p<sub>1</sub>,p<sub>2</sub>,...,p<sub>n</sub>: the weight of each apple.
///
/// <b>Output</b>
///
/// Print one integer: the minimum difference between the weights of the groups.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 20</li>
/// <li>1 ≤ p<sub>i</sub> ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let n: usize = scan.token();
    let mut apples = [0_i64; 20];
    for apple in &mut apples[..n] {
        *apple = scan.token();
    }
    apples.sort_unstable_by(|a, b| b.cmp(a));

    let mut weights = [0_i64; 20];
    let mut weight_sum = 0;
    for (weight, apple) in weights[(20 - n)..].iter_mut().zip(apples[..n].iter().rev()) {
        weight_sum += apple;
        *weight = weight_sum;
    }
    weights.reverse();

    writeln!(out, "{}", min_diff(0, &apples, &weights, 0, n - 1)).unwrap();
}

fn min_diff(
    iter: usize,
    apples: &[i64; 20],
    weights: &[i64; 20],
    last_subset_diff: i64,
    len: usize,
) -> i64 {
    unsafe {
        if iter == len {
            // base case, both subsets are filled after the final call
            return (last_subset_diff - apples.get_unchecked(iter)).abs();
        }
        if last_subset_diff + apples.get_unchecked(iter) >= *weights.get_unchecked(iter + 1) {
            // we no longer need to fill subset 1, exclusively fill subset 2
            return std::cmp::min(
                last_subset_diff + apples.get_unchecked(iter) - weights.get_unchecked(iter + 1),
                min_diff(
                    iter + 1,
                    apples,
                    weights,
                    (last_subset_diff - apples.get_unchecked(iter)).abs(),
                    len,
                ),
            );
        }
        // still need to fill both subsets at this point
        std::cmp::min(
            min_diff(
                iter + 1,
                apples,
                weights,
                last_subset_diff + apples.get_unchecked(iter),
                len,
            ),
            min_diff(
                iter + 1,
                apples,
                weights,
                (last_subset_diff - apples.get_unchecked(iter)).abs(),
                len,
            ),
        )
    }
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
5
3 2 7 4 1
";
        let target = b"\
1
";

        test(input, target);
    }

    #[test]
    fn test_one_group() {
        let input = b"\
1
37
";
        let target = b"\
37
";

        test(input, target);
    }

    #[test]
    fn test_large_addition() {
        let input = b"\
20
1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000
";
        let target = b"\
0
";

        test(input, target);
    }
}
