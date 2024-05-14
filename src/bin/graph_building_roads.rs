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

const BOUND_NUM: usize = 0xFFF_FFF_FFF_FFF_FFE;

/// this implementation only works for values less than `usize::MAX` / 2,
/// idea is that values = `BOUND_NUM` are not joined, values < `BOUND_NUM` are an index,
struct DisjointSet {
    parents: Vec<usize>,
}

impl DisjointSet {
    pub fn from_offset(n: usize) -> Self {
        let offset = n + 1;
        Self {
            parents: vec![BOUND_NUM; offset],
        }
    }

    pub fn find(&mut self, mut idx: usize) -> usize {
        let mut prev = idx;
        // get parent
        while self.parents[idx] < BOUND_NUM {
            idx = self.parents[idx];
        }

        // path compression
        while prev != idx {
            let tmp = self.parents[prev];
            self.parents[prev] = idx;
            prev = tmp;
        }

        idx
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let mut a_idx = self.find(a);
        let mut b_idx = self.find(b);

        if a_idx == b_idx {
            // both are already added in the same set
            return;
        }

        let a_size = self.parents[a_idx];
        let b_size = self.parents[b_idx];

        if a_size > b_size {
            std::mem::swap(&mut a_idx, &mut b_idx);
        }

        self.parents[b_idx] = a_size.wrapping_add(b_size).wrapping_sub(BOUND_NUM);
        self.parents[a_idx] = b_idx;
    }
}

/// Byteland has n cities, and m roads between them. The goal is to construct new roads so that there is a route between any two cities.
///
/// Your task is to find out the minimum number of roads required, and also determine which roads should be built.
///
/// <b>Input</b>
///
/// The first input line has two integers n and m: the number of cities and roads. The cities are numbered 1,2,...,n.
///
/// After that, there are m lines describing the roads. Each line has two integers a and b: there is a road between those cities.
///
/// A road always connects two different cities, and there is at most one road between any two cities.
///
/// <b>Output</b>
///
/// First print an integer k: the number of required roads.
///
/// Then, print k lines that describe the new roads. You can print any valid solution.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 10<sup>5</sup></li>
/// <li>1 ≤ m ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ a,b ≤ n </li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let cities: usize = scan.token();
    let num_roads: u32 = scan.token();

    let mut dj_set = DisjointSet::from_offset(cities);

    for _ in 0..num_roads {
        let a = scan.token();
        let b = scan.token();
        dj_set.union(a, b);
    }

    let find_pos = dj_set.parents.iter().skip(1).position(|&x| x >= BOUND_NUM);
    if let Some(pos) = find_pos {
        let first_pos = pos + 1;
        let connections: Vec<_> = dj_set
            .parents
            .iter()
            .enumerate()
            .skip(first_pos + 1)
            .filter(|(_, val)| **val >= BOUND_NUM)
            .map(|(idx, _)| idx)
            .collect();
        writeln!(out, "{}", connections.len()).unwrap();
        for location in connections {
            writeln!(out, "{first_pos} {location}").unwrap();
        }
    } else {
        out.write_all(b"0\n").unwrap();
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
4 2
1 2
3 4
";
        let target = b"\
1
2 4
";

        test(input, target);
    }

    #[test]
    fn test_no_roads_needed() {
        let input = b"\
3 2
1 2
3 2
";
        let target = b"\
0
";

        test(input, target);
    }
}
