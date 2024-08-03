// I/O boilerplate

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

/// In a movie festival n movies will be shown. You know the starting and ending time of each movie. What is the maximum number of movies you can watch entirely?
///
/// <b>Input</b>
///
/// The first input line has an integer n: the number of movies.
///
/// After this, there are n lines that describe the movies. Each line has two integers a and b: the starting and ending times of a movie.
///
/// <b>Output</b>
///
/// Print one integer: the maximum number of movies.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ a ≤ b ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');

    let num = unsafe { u32::to_posint(iter.next().unwrap_unchecked()) };
    let mut movies: Vec<(u32, u32)> = (0..num)
        .map(|_| {
            let start = unsafe { u32::to_posint(iter.next().unwrap_unchecked()) };
            let end = unsafe { u32::to_posint(iter.next().unwrap_unchecked()) };
            (end, start)
        })
        .collect();

    movies.sort_unstable_by_key(|m| m.0);

    writeln!(
        out,
        "{}",
        movies
            .into_iter()
            .fold((0, 0_u32), |(curr_end, count), (end, start)| {
                if start < curr_end {
                    (curr_end, count)
                } else {
                    (end, count + 1)
                }
            })
            .1
    )
    .unwrap();
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
3
3 5
4 9
5 8
";
        let target = b"\
2
";

        test(input, target);
    }

    #[test]
    fn test_one() {
        let input = b"\
3
1 3
2 4
2 4
";
        let target = b"\
1
";

        test(input, target);
    }

    #[test]
    fn skip_the_first() {
        let input = b"\
3
1 1000
2 3
3 4
";
        let target = b"\
2
";

        test(input, target);
    }
}
