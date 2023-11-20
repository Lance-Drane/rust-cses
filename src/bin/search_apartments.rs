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
        // optional memory clear
        buf_str.clear();

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

/// There are n applicants and m free apartments. Your task is to distribute the apartments so that as many applicants as possible will get an apartment.
///
/// Each applicant has a desired apartment size, and they will accept any apartment whose size is close enough to the desired size.
///
/// <b>Input</b>
///
/// The first input line has three integers n, m, and k: the number of applicants, the number of apartments, and the maximum allowed difference.
///
/// The next line contains n integers a1, a2, ..., a<sub>n</sub>: the desired apartment size of each applicant. If the desired size of an applicant is x, he or she will accept any apartment whose size is between x - k and x + k.
///
/// The last line contains m integers b1, b2, ..., b<sub>m</sub>: the size of each apartment.
///
/// <b>Output</b>
///
/// Print one integer: the number of applicants who will get an apartment.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n,m ≤ 2 * 10<sup>5</sup></li>
/// <li>0 ≤ k ≤ 10<sup>9</sup></li>
/// <li>1 ≤ a<sub>i</sub> , b<sub>i</sub> ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let n: u32 = scan.token();
    let m: u32 = scan.token();
    let k: i32 = scan.token();

    let mut applicants: Vec<i32> = (0..n).map(|_| scan.token()).collect();
    applicants.sort_unstable();
    let mut apartments: Vec<i32> = (0..m).map(|_| scan.token()).collect();
    apartments.sort_unstable();

    let mut counter = 0;
    let mut apt_idx = apartments.len();

    'applicant: for applicant in applicants.iter().rev() {
        let u_bound = applicant + k;
        let l_bound = applicant - k;

        while apt_idx > 0 {
            let apartment = apartments[apt_idx - 1];
            if apartment <= u_bound {
                if apartment >= l_bound {
                    // Case 1: found valid apartment
                    counter += 1;
                    apt_idx -= 1;
                }
                // we either found a valid apartment, or we have:
                // Case 2: the largest remaining apartment is too small
                // either way, we're done processing this applicant
                continue 'applicant;
            }
            // Case 3: apartment too high for highest applicant.
            // nobody else will want this apartment, get rid of it and continue looping
            apt_idx -= 1;
        }
        // no apartments remaining
        break;
    }

    writeln!(out, "{counter}").ok();
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
4 3 5
60 45 80 60
30 60 75
";
        let target = b"\
2
";

        test(input, target);
    }
}
