// I/O boilerplate //

use std::io::Read;

// problem //

/// You are given a DNA sequence: a string consisting of characters A, C, G, and T. Your task is to find the longest repetition in the sequence. This is a maximum-length substring containing only one type of character.
///
/// <b>Input</b>
///
/// The only input line contains a string of n characters.
///
/// <b>Output</b>
///
/// Print one integer: the length of the longest repetition.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 10<sup>6</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let token = &scan[..scan.len() - 1];

    writeln!(
        out,
        "{}",
        token
            .windows(2)
            .fold((1, 1), |(longest, curr_len), window| {
                if window[0] == window[1] {
                    let next_len = curr_len + 1;
                    (longest.max(next_len), next_len)
                } else {
                    (longest, 1)
                }
            })
            .0
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
    fn test_1() {
        let input = b"\
ATTCGGGA
";
        let target = b"\
3
";

        test(input, target);
    }

    #[test]
    fn test_one_length_string() {
        let input = b"\
A
";
        let target = b"\
1
";

        test(input, target);
    }

    #[test]
    fn test_one_char_repeating() {
        let input = b"\
AAAAAAAAAA
";
        let target = b"\
10
";

        test(input, target);
    }

    #[test]
    fn test_largest_at_end() {
        let input = b"\
ACCGGGTTTT
";
        let target = b"\
4
";

        test(input, target);
    }

    #[test]
    fn test_largest_at_beginning() {
        let input = b"\
AAAACCCGGT
";
        let target = b"\
4
";

        test(input, target);
    }
}
