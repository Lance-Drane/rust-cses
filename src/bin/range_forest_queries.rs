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

const MAX_FOREST_CAPACITY: usize = 1001 * 1001;

/// You are given an n * n grid representing the map of a forest. Each square is either empty or contains a tree. The upper-left square has coordinates (1,1), and the lower-right square has coordinates (n,n).
///
/// Your task is to process q queries of the form: how many trees are inside a given rectangle in the forest?
///
/// <b>Input</b>
///
/// The first input line has two integers n and q: the size of the forest and the number of queries.
///
/// Then, there are n lines describing the forest. Each line has n characters: . is an empty square and * is a tree.
///
/// Finally, there are q lines describing the queries. Each line has four integers y1, x1, y2, x2 corresponding to the corners of a rectangle.
///
/// <b>Output</b>
///
/// Print the number of trees inside each rectangle.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 1000</li>
/// <li>1 ≤ q ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ y<sub>1</sub> ≤ y<sub>2</sub> ≤ n</li>
/// <li>1 ≤ x<sub>1</sub> ≤ x<sub>2</sub> ≤ n</li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let n = scan.token::<usize>() + 1;
    let q: u32 = scan.token();

    let mut forest: Vec<u32> = vec![0; MAX_FOREST_CAPACITY];
    for (row_idx, row) in (1..n).map(|idx| (idx, scan.token::<String>().into_bytes())) {
        for (cell, col_idx) in row.into_iter().zip(1..n) {
            forest[row_idx * n + col_idx] = forest[(row_idx - 1) * n + col_idx]
                + forest[row_idx * n + col_idx - 1]
                - forest[(row_idx - 1) * n + col_idx - 1]
                + u32::from(cell == b'*');
        }
    }

    for _ in 0..q {
        let y1: usize = scan.token();
        let x1: usize = scan.token();
        let y2: usize = scan.token();
        let x2: usize = scan.token();

        writeln!(out, "{}", unsafe {
            forest.get_unchecked(y2 * n + x2) + forest.get_unchecked((y1 - 1) * n + x1 - 1)
                - forest.get_unchecked(y2 * n + x1 - 1)
                - forest.get_unchecked((y1 - 1) * n + x2)
        })
        .unwrap();
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
4 3
.*..
*.**
**..
****
2 2 3 4
3 1 3 1
1 1 2 2
";
        let target = b"\
3
1
2
";

        test(input, target);
    }
}
