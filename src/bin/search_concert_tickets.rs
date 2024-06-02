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

use std::collections::BTreeMap;

/// There are n concert tickets available, each with a certain price. Then, m customers arrive, one after another.
///
/// Each customer announces the maximum price they are willing to pay for a ticket, and after this, they will get a ticket with the nearest possible price such that it does not exceed the maximum price.
///
/// <b>Input</b>
///
/// The first input line contains integers n and m: the number of tickets and the number of customers.
///
/// The next line contains n integers h<sub>1</sub>,h<sub>2</sub>,...,h<sub>n</sub>: the price of each ticket.
///
/// The last line contains m integers t<sub>1</sub>,t<sub>2</sub>,...,t<sub>m</sub>: the maximum price for each customer in the order they arrive.
///
/// <b>Output</b>
///
/// Print, for each customer, the price that they will pay for their ticket. After this, the ticket cannot be purchased again.
///
/// If a customer cannot get any ticket, print -1.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n,m ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ h<sub>i</sub>,t<sub>i</sub> ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let n: u32 = scan.token();
    let m: u32 = scan.token();

    let mut ticket_counter: BTreeMap<u32, u32> = BTreeMap::new();
    for _ in 0..n {
        ticket_counter
            .entry(scan.token())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    for customer in (0..m).map(|_| scan.token::<u32>()) {
        match ticket_counter.range_mut(..=customer).next_back() {
            Some((&ticket, count)) => {
                writeln!(out, "{ticket}").unwrap();
                if *count == 1 {
                    ticket_counter.remove(&ticket);
                } else {
                    *count -= 1;
                }
            }
            None => {
                out.write_all(b"-1\n").unwrap();
            }
        };
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
5 3
5 3 7 8 5
4 8 3
";
        let target = b"\
3
8
-1
";

        test(input, target);
    }

    #[test]
    fn test_identicals() {
        let input = b"\
10 10
1 1 1 1 1 1 1 1 1 1
1 1 1 1 1 1 1 1 1 1
";
        let target = b"\
1
1
1
1
1
1
1
1
1
1
";

        test(input, target);
    }

    #[test]
    fn test_more_customers_than_tickets() {
        let input = b"\
3 4
2 2 2
4 4 4 4
";
        let target = b"\
2
2
2
-1
";

        test(input, target);
    }
}
