// I/O boilerplate //

use std::io::Read;

// problem //

/// Number of characters in output (excluding counter line) is (n+1)!. Count the lines, then multiply (characters + 1) (include newline).
const MAX_CHARS: usize = 9 * 8 * 7 * 6 * 5 * 4 * 3 * 2;

/// Given a string, your task is to generate all different strings that can be created using its characters.
///
/// <b>Input</b>
///
/// The only input line has a string of length n. Each character is between a–z.
///
/// <b>Output</b>
///
/// First print an integer k: the number of strings. Then print k lines: the strings in alphabetical order.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 8</li>
/// </ul>
fn solve<W: std::io::Write>(scan: &mut [u8], out: &mut W) {
    let (string, _) = scan.split_at_mut(scan.len() - 1);
    string.sort_unstable();

    let mut tmp_buf = Vec::with_capacity(MAX_CHARS);

    tmp_buf.extend_from_slice(string);
    tmp_buf.push(b'\n');
    while next_permutation(string) {
        tmp_buf.extend_from_slice(string);
        tmp_buf.push(b'\n');
    }

    writeln!(out, "{}", tmp_buf.len() / (string.len() + 1)).unwrap();
    out.write_all(&tmp_buf).unwrap();
}

fn next_permutation<T: std::cmp::Ord>(slice: &mut [T]) -> bool {
    if let Some((i, i2)) = slice
        .windows(2)
        .enumerate()
        .rfind(|(_, w)| w[0] < w[1])
        .map(|(idx, w)| {
            (
                idx,
                slice[idx + 1..]
                    .iter()
                    .rposition(|x| w[0] < *x)
                    .unwrap_or(0),
            )
        })
    {
        slice.swap(i, i2 + i + 1);
        slice[i + 1..].reverse();
        return true;
    }
    false
}

// entrypoints //

fn main() {
    let mut buf_str = vec![];
    std::io::stdin().lock().read_to_end(&mut buf_str).unwrap();
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    solve(&mut buf_str, &mut out);
}

#[cfg(test)]
mod test {
    use super::*;

    fn test(input: &[u8], target: &[u8]) {
        let mut out = Vec::with_capacity(target.len());
        solve(&mut input.to_owned(), &mut out);

        assert_eq!(out, target);
    }

    #[test]
    fn test_example() {
        let input = b"\
aabac
";
        let target = b"\
20
aaabc
aaacb
aabac
aabca
aacab
aacba
abaac
abaca
abcaa
acaab
acaba
acbaa
baaac
baaca
bacaa
bcaaa
caaab
caaba
cabaa
cbaaa
";

        test(input, target);
    }

    #[test]
    fn test_size_1() {
        let input = b"\
a
";
        let target = b"\
1
a
";

        test(input, target);
    }

    #[test]
    fn test_repeated() {
        let input = b"\
aaaaa
";
        let target = b"\
1
aaaaa
";

        test(input, target);
    }

    #[test]
    fn test_size_2() {
        let input = b"\
ab
";
        let target = b"\
2
ab
ba
";

        test(input, target);
    }
}
