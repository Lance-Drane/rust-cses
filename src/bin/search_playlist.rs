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

use std::collections::HashMap;

/// You are given a playlist of a radio station since its establishment. The playlist has a total of n songs.
///
/// What is the longest sequence of successive songs where each song is unique?
///
/// <b>Input</b>
///
/// The first input line contains an integer n: the number of songs.
///
/// The next line has n integers k<sub>1</sub>,k<sub>2</sub<,...,k<sub<n</sub>: the id number of each song.
///
/// <b>Output</b>
///
/// Print the length of the longest sequence of unique songs.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ k<sub>i</sub> ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');

    let number = unsafe { u32::to_posint(iter.next().unwrap_unchecked()) };

    let mut visited: HashMap<u32, u32> = HashMap::with_capacity(number as usize);
    let mut l_pointer = 0;
    let mut best = 0;

    for (song, idx) in
        (1..(number + 1)).map(|n| (unsafe { u32::to_posint(iter.next().unwrap_unchecked()) }, n))
    {
        if let Some(prev) = visited.insert(song, idx) {
            l_pointer = l_pointer.max(prev);
        }
        best = best.max(idx - l_pointer);
    }

    writeln!(out, "{best}").unwrap();
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
8
1 2 1 3 2 7 4 2
";
        let target = b"\
5
";

        test(input, target);
    }

    #[test]
    fn test_repetition() {
        let input = b"\
4
1000000000 1000000000 1000000000 1000000000
";
        let target = b"\
1
";

        test(input, target);
    }

    #[test]
    fn test_sample() {
        let input = b"\
20
4 1 1 4 8 9 7 6 5 9 4 9 7 3 10 3 8 3 9 6
";
        let target = b"\
7
";

        test(input, target);
    }
}
