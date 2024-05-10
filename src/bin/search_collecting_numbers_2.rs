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

/// You are given an array that contains each number between 1 ... n exactly once. Your task is to collect the numbers from 1 to n in increasing order.
///
/// On each round, you go through the array from left to right and collect as many numbers as possible. What will be the total number of rounds?
///
/// Given m operations that swap two numbers in the array, your task is to report the number of rounds after each operation.
///
/// <b>Input</b>
///
/// The first line has two integers n and m: the array size and the number of operations.
///
/// The next line contains n integers x<sub>1</sub>,x<sub>2</sub>,...,x<sub>n</sub>: the numbers in the array.
///
/// Finally, there are m lines that describe the operations. Each line has two integers a and b: the numbers at positions a and b are swapped.
///
/// <b>Output</b>
///
/// Print one integer: the number of rounds.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n,m ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ a,b ≤ n</li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let n: usize = scan.token();
    let m: u32 = scan.token();
    let mut position_arr = vec![0; n + 1];
    let mut num_arr = vec![0; n + 1];

    for (index, number) in (1..=n).map(|idx| (idx, scan.token::<usize>())) {
        unsafe {
            *position_arr.get_unchecked_mut(number) = index;
            *num_arr.get_unchecked_mut(index) = number;
        }
    }

    let mut rounds = position_arr.windows(2).filter(|w| w[1] < w[0]).count() + 1;
    let mut pairs: Vec<(usize, usize)> = Vec::with_capacity(4);

    for _ in 0..m {
        let l_idx: usize = scan.token();
        let r_idx: usize = scan.token();

        if l_idx == r_idx {
            writeln!(out, "{rounds}").unwrap();
            continue;
        }

        let l_num = unsafe { *num_arr.get_unchecked(l_idx) };
        let r_num = unsafe { *num_arr.get_unchecked(r_idx) };
        num_arr.swap(l_idx, r_idx);

        let (min, max) = if l_num < r_num {
            (l_num, r_num)
        } else {
            (r_num, l_num)
        };
        if min != 1 {
            pairs.push((min - 1, min));
        }
        match max - min {
            1 => pairs.push((min, max)),
            _ => pairs.extend([(min, min + 1), (max - 1, max)]),
        }
        if max != n {
            pairs.push((max, max + 1));
        }

        rounds -= pairs
            .iter()
            .filter(|(l, r)| position_arr[*l] > position_arr[*r])
            .count();
        position_arr.swap(l_num, r_num);
        rounds += pairs
            .iter()
            .filter(|(l, r)| position_arr[*l] > position_arr[*r])
            .count();

        pairs.clear();
        writeln!(out, "{rounds}").unwrap();
    }
}

// 4 2 1 5 3 = 3 2 5 1 4
// 4 1 2 5 3 = 2 3 5 1 4
// 3 1 2 5 4 = 3 5 4 1 2
// 3 2 1 5 4 = 3 5 4 2 1

// 3 1 5 2 4

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
5 3
4 2 1 5 3
2 3
1 5
2 3
";
        let target = b"\
2
3
4
";

        test(input, target);
    }

    #[test]
    fn test_one() {
        let input = b"\
1 1
1
1 1
";
        let target = b"\
1
";

        test(input, target);
    }
}
