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

/// Given the structure of a company, your task is to calculate for each employee the number of their subordinates.
///
/// <b>Input</b>
///
/// The first input line has an integer n: the number of employees. The employees are numbered 1,2,...,n, and employee 1 is the general director of the company.
///
/// After this, there are n-1 integers: for each employee 2,3,...,n their direct boss in the company.
///
/// <b>Output</b>
///
/// Print n integers: for each employee 1,2,...,n the number of their subordinates.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let n: usize = scan.token();

    let mut bosses = vec![0; n];
    let mut immediate_subord_cnt = vec![0; n]; // -1 = processed
    let mut total_subord_cnt = vec![0_u32; n];

    for (employee, boss) in (1..n).map(|worker| (worker, scan.token::<usize>() - 1)) {
        unsafe {
            *bosses.get_unchecked_mut(employee) = boss;
            *immediate_subord_cnt.get_unchecked_mut(boss) += 1;
        }
    }

    for i in 1..n {
        let mut boss = i;
        unsafe {
            while *immediate_subord_cnt.get_unchecked(boss) == 0 && boss != 0 {
                *immediate_subord_cnt.get_unchecked_mut(boss) -= 1;
                let next_boss = *bosses.get_unchecked(boss);
                *total_subord_cnt.get_unchecked_mut(next_boss) +=
                    total_subord_cnt.get_unchecked(boss) + 1;
                *immediate_subord_cnt.get_unchecked_mut(next_boss) -= 1;
                boss = next_boss;
            }
        }
    }

    for sub in total_subord_cnt {
        write!(out, "{sub} ").unwrap();
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
1 1 2 3
";
        let target = b"\
4 1 1 0 0 ";

        test(input, target);
    }

    #[test]
    fn test_reversed() {
        let input = b"\
6
3 4 1 1 4
";
        let target = b"\
5 0 1 3 0 0 ";

        test(input, target);
    }

    #[test]
    fn test_mix() {
        let input = b"\
5
4 5 3 1
";
        let target = b"\
4 0 2 1 3 ";

        test(input, target);
    }

    #[test]
    fn test_empty() {
        let input = b"\
2
1
";
        let target = b"\
1 0 ";

        test(input, target);
    }
}
