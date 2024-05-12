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

struct Light {
    /// place of light on street
    position: u32,
    /// order we added the light
    number: usize,
    /// previous light on the street
    left: usize,
    /// next light on the street
    right: usize,
}

/// There is a street of length x whose positions are numbered 0,1,...,x. Initially there are no traffic lights, but n sets of traffic lights are added to the street one after another.
///
/// Your task is to calculate the length of the longest passage without traffic lights after each addition.
///
/// <b>Input</b>
///
/// The first input line contains two integers x and n: the length of the street and the number of sets of traffic lights.
///
/// Then, the next line contains n integers p<sub>1</sub>,p<sub>2</sub>,...,p<sub>n</sub>: the position of each set of traffic lights. Each position is distinct.
///
/// <b>Output</b>
///
/// Print the length of the longest passage without traffic lights after each addition.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ x ≤ 10<sup>9</sup></li>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// <li>0 < p<sub>i</sub> < x</li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let total: u32 = scan.token();
    let n: usize = scan.token();

    let mut positions = vec![0; n];

    let mut lights: Vec<Light> = std::iter::once(Light {
        position: 0,
        number: 0,
        left: usize::MAX,
        right: 1,
    })
    .chain((0..n).map(|num| Light {
        position: scan.token(),
        number: num,
        left: 0,
        right: 0,
    }))
    .chain(std::iter::once(Light {
        position: total,
        number: n,
        left: n - 1,
        right: usize::MAX,
    }))
    .collect();
    lights.sort_unstable_by_key(|l| l.position);

    for (idx, light) in lights.iter_mut().enumerate().skip(1).take(n) {
        unsafe {
            *positions.get_unchecked_mut(light.number) = idx;
        }
        light.left = idx - 1;
        light.right = idx + 1;
    }

    let mut max = lights
        .windows(2)
        .map(|w| w[1].position - w[0].position)
        .max()
        .unwrap();
    let mut answers = Vec::with_capacity(n);
    answers.push(max);

    for position in positions.into_iter().rev().take(n - 1) {
        let curr = unsafe { lights.get_unchecked(position) };
        let left = curr.left;
        let right = curr.right;
        max = unsafe {
            max.max(lights.get_unchecked(right).position - lights.get_unchecked(left).position)
        };
        answers.push(max);
        unsafe {
            lights.get_unchecked_mut(left).right = right;
            lights.get_unchecked_mut(right).left = left;
        }
    }

    for answer in answers.into_iter().rev() {
        write!(out, "{answer} ").unwrap();
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
8 3
3 6 2
";
        let target = b"\
5 3 3 ";

        test(input, target);
    }

    #[test]
    fn test_varying_ranges() {
        let input = b"\
7 2
3 5
";
        let target = b"\
4 3 ";

        test(input, target);
    }
}
