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

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// There is a large hotel, and n customers will arrive soon. Each customer wants to have a single room.
///
/// You know each customer's arrival and departure day. Two customers can stay in the same room if the departure day of the first customer is earlier than the arrival day of the second customer.
///
/// What is the minimum number of rooms that are needed to accommodate all customers? And how can the rooms be allocated?
///
/// <b>Input</b>
///
/// The first input line contains an integer n: the number of customers.
///
/// Then there are n lines, each of which describes one customer. Each line has two integers a and b: the arrival and departure day.
///
/// <b>Output</b>
///
/// Print first an integer k: the minimum number of rooms required.
///
/// After that, print a line that contains the room number of each customer in the same order as in the input. The rooms are numbered 1,2,...,k. You can print any valid solution.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ a ≤ b ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let n: usize = scan.token();
    let mut ans = vec![0; n];
    // (arrival_day, departure_day, original index)
    let mut waitlist: Vec<(u32, u32, usize)> =
        (0..n).map(|i| (scan.token(), scan.token(), i)).collect();
    waitlist.sort_unstable_by_key(|w| w.0);
    // (departure_day, room)
    let mut staylist = BinaryHeap::with_capacity(n);

    // process first separately from rest, as we will now always have at least one item in "staylist"
    {
        let first_in = unsafe { waitlist.get_unchecked(0) };
        staylist.push(Reverse((first_in.1, 1)));
        unsafe {
            *ans.get_unchecked_mut(first_in.2) = 1;
        }
    }

    let mut max_rooms = 1_u32;

    for (arrive, depart, idx) in waitlist.into_iter().skip(1) {
        let next_out = staylist.peek().unwrap().0;

        let room = if arrive > next_out.0 {
            staylist.pop().unwrap().0 .1
        } else {
            max_rooms += 1;
            max_rooms
        };
        staylist.push(Reverse((depart, room)));
        unsafe {
            *ans.get_unchecked_mut(idx) = room;
        }
    }

    writeln!(out, "{max_rooms}").unwrap();
    for a in ans {
        write!(out, "{a} ").unwrap();
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
1 2
2 4
4 4
";
        let target = b"\
2
1 2 1 ";
        test(input, target);
    }

    #[test]
    fn test_example_2() {
        let input = b"\
10
8 8
5 8
8 9
1 4
1 3
5 7
4 8
2 2
4 5
6 8
";
        let target = b"\
5
4 1 5 1 2 4 3 3 2 2 ";
        test(input, target);
    }
}
