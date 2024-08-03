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
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');

    let n = unsafe { u32::to_posint(iter.next().unwrap_unchecked()) };
    let m = unsafe { u32::to_posint(iter.next().unwrap_unchecked()) };
    let k = unsafe { i32::to_posint(iter.next().unwrap_unchecked()) };

    let mut applicants: Vec<i32> = (0..n)
        .map(|_| unsafe { i32::to_posint(iter.next().unwrap_unchecked()) })
        .collect();
    applicants.sort_unstable();
    let mut apartments: Vec<i32> = (0..m)
        .map(|_| unsafe { i32::to_posint(iter.next().unwrap_unchecked()) })
        .collect();
    apartments.sort_unstable();

    let mut counter = 0;
    let mut apt_idx = apartments.len();

    'applicant: for applicant in applicants.into_iter().rev() {
        let u_bound = applicant + k;
        let l_bound = applicant - k;

        while apt_idx > 0 {
            let apartment = unsafe { *apartments.get_unchecked(apt_idx - 1) };
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

    writeln!(out, "{counter}").unwrap();
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
