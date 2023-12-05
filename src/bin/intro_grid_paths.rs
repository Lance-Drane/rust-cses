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

/// There are 88418 paths in a 7 * 7 grid from the upper-left square to the lower-left square. Each path corresponds to a 48-character description consisting of characters D (down), U (up), L (left) and R (right).
///
/// For example, the path
///
/// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAALYAAAC4CAYAAABO+hZ0AAAABHNCSVQICAgIfAhkiAAAAAlwSFlzAAAOxAAADsQBlSsOGwAABYZJREFUeJzt3LFO41gUh/GTVSKxmooXoMgLUNAkFZSZl9hiC4p9gQlPgN9gn4FtqJDoxlQ4NR1lJKZkZGkL0HjIFmyz2tjjmzk39v37+7VkzpjLp8iSkzPabDYbA8T80vUFADEQNiQRNiQRNiQRNiQRNiSN636Q57nd3t5G+4/X67VVVWXT6ZT5zN/J2dmZLRaLrT+rDbsoClutVrX/8GddX1/by8uLnZycMJ/5wfI8t9FoVNvnqO4BTZZlVpalXV5eRrmwxWJhZVna/f0985kf7Ed9co8NSYQNSYQNSYQNSYQNSYQNSYQNSYQNSYQNSbWP1HtpuWz90t8fH+319TXo34RgfoT5Web2//uFnWVmZWkW6RG8ZZnZamXW8rMrf08m9vL9u9nhYZTLYb7z/Dw3u7hw6yetd+zZzOzTp1Yv/evzZyvL0v5o+fpQzHeev9m8vzE64R4bkggbkggbkggbkggbkggbkggbkggbkggbkmq/pX5+fm43Nzd2dHTUatBvX77Yh6qyP1u+/uHhwd7e3uz4+LgX80Mx33d+6N/36enJ5vO5XV1dbf157SP18XhsBwcHdtjyWf+vX7/awbdvrV8/mUysqqrezA/FfN/5oX/f5+dnG4/rPxHit1ck8ENQwXsnYs8PxHzn+YF/X/aKYJAIG5IIG5IIG5IIG5IIG5IIG5IIG5IIG5J8v6We5633SATvnSgKs/l892trI/beEse9GVulfv2O/MJeLs1Go9YvD9478fFj69ULO4m9t8R5b8b/pH79znzfsQPCi73XYicx95Y4783YKvXrd8Q9NiQRNiQRNiQRNiQRNiQRNiQRNiQRNiQRNiS57RUJlfpeC+Z3O39ve0VCpb7Xgvndzt/fXpFAqe+1YH6389krgkEibEgibEgibEgibEgibEgibEgibEgibEjy/ZZ67L0WAXq5twR74xd27L0WgXq3twR75fuOHXOvRaBe7i3B3nCPDUmEDUmEDUmEDUmEDUmEDUmEDUmEDUmEDUlue0Vi750I1bf5fdvLkfr8ve0Vib13IlTf5vdtL0fq8/e3VyT23olAvZvfs70cqc9nrwgGibAhibAhibAhibAhibAhibAhibAhibAhyfdb6rGxt6Rbed769+36fNIJm70l3VouzUaj1i/v+nzSCduMvSVdC/hduz4f7rEhibAhibAhibAhibAhibAhibAhibAhibAhKZm9IuwtaTa085HZK8LekmZDOx+dvSLsLWk2sPNhrwgGibAhibAhibAhibAhibAhibAhibAhibAhKa1vqcfca2H2/vQupph7UfaxtyT2XhfH808n7Nh7LfLc7OKi9SPpYLH3osTeWxL7+p3PP52wzeLutdhs3j9rEVOP9qLsJOb1O58/99iQRNiQRNiQRNiQRNiQRNiQRNiQRNiQRNiQlMxekVB928vB+fjOl9krEqpvezk4H9/5OntFAvVuLwfn4zqfvSIYJMKGJMKGJMKGJMKGJMKGJMKGJMKGJMKGpLS+pZ76Xo7Ye1EC9PJ8HKUTdup7OWLvRQnUu/Nxlk7YZunv5Yi5FyVQL8/HEffYkETYkETYkETYkETYkETYkETYkETYkETYkJTMXpHU93Iwv9lg94qkvpeD+c2Gu1ck9b0czG/GXhHgxwgbkggbkggbkggbkggbkggbkggbkggbktL6lnrKezmY38x5b4lf2JEO6D/zU97LwfxmzntL0nrHHtBeDub/HO6xIYmwIYmwIYmwIYmwIYmwIYmwIYmwIYmwIanxyWOe57aM9Kj88d/PEjCf+bsoisLmDZ8tqV2/UBSF3d3dRbkoM7P1em1VVdl0OmU+83cym83s9PR0689qwwZSxj02JBE2JBE2JBE2JBE2JBE2JP0Du5gjZ7gUsZMAAAAASUVORK5CYII=" width="100" height="100">
///
/// corresponds to the description DRURRRRRDDDLUULDDDLDRRURDDLLLLLURULURRUULDLLDDDD.
///
/// You are given a description of a path which may also contain characters ? (any direction). Your task is to calculate the number of paths that match the description.
///
/// <b>Input</b>
///
/// The only input line has a 48-character string of characters ?, D, U, L and R.
///
/// <b>Output</b>
///
/// Print one integer: the total number of paths.
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let mut path = [0_u8; 48];
    path.copy_from_slice(&scan.token::<String>().into_bytes());

    // marking the actual grid with a border saves on having to do additional control flow checks
    let mut visited = [
        [true, true, true, true, true, true, true, true, true],
        [true, false, false, false, false, false, false, false, true],
        [true, false, false, false, false, false, false, false, true],
        [true, false, false, false, false, false, false, false, true],
        [true, false, false, false, false, false, false, false, true],
        [true, false, false, false, false, false, false, false, true],
        [true, false, false, false, false, false, false, false, true],
        [true, false, false, false, false, false, false, false, true],
        [true, true, true, true, true, true, true, true, true],
    ];

    let mut count = 0;
    recurse(0, (1, 1), &mut count, &mut visited, &path);

    writeln!(out, "{count}").ok();
}

#[allow(clippy::too_many_lines)]
fn recurse(
    path_iter: usize,
    position: (usize, usize),
    count: &mut u32,
    visited: &mut [[bool; 9]; 9],
    path: &[u8; 48],
) {
    // base case - check if we're on the ending square
    if position == (1, 7) {
        if path_iter == 48 {
            *count += 1;
        }
        return;
    }
    // reached end of path
    if path_iter == 48 {
        return;
    }

    unsafe {
        *visited
            .get_unchecked_mut(position.0)
            .get_unchecked_mut(position.1) = true;
        match path.get_unchecked(path_iter) {
            b'U' => {
                if can_proceed_up(&position, visited) {
                    recurse(
                        path_iter + 1,
                        (position.0, position.1 - 1),
                        count,
                        visited,
                        path,
                    );
                }
            }
            b'R' => {
                if can_proceed_right(&position, visited) {
                    recurse(
                        path_iter + 1,
                        (position.0 + 1, position.1),
                        count,
                        visited,
                        path,
                    );
                }
            }
            b'D' => {
                if can_proceed_down(&position, visited) {
                    recurse(
                        path_iter + 1,
                        (position.0, position.1 + 1),
                        count,
                        visited,
                        path,
                    );
                }
            }
            b'L' => {
                if can_proceed_left(&position, visited) {
                    recurse(
                        path_iter + 1,
                        (position.0 - 1, position.1),
                        count,
                        visited,
                        path,
                    );
                }
            }
            _ => {
                if can_proceed_up(&position, visited) {
                    recurse(
                        path_iter + 1,
                        (position.0, position.1 - 1),
                        count,
                        visited,
                        path,
                    );
                }
                if can_proceed_right(&position, visited) {
                    recurse(
                        path_iter + 1,
                        (position.0 + 1, position.1),
                        count,
                        visited,
                        path,
                    );
                }
                if can_proceed_down(&position, visited) {
                    recurse(
                        path_iter + 1,
                        (position.0, position.1 + 1),
                        count,
                        visited,
                        path,
                    );
                }
                if can_proceed_left(&position, visited) {
                    recurse(
                        path_iter + 1,
                        (position.0 - 1, position.1),
                        count,
                        visited,
                        path,
                    );
                }
            }
        }
        *visited
            .get_unchecked_mut(position.0)
            .get_unchecked_mut(position.1) = false;
    }
}

// do not visit any square if it would disconnect unvisited squares (front blocked, both sides not) or cause a dead end

fn can_proceed_up(position: &(usize, usize), visited: &mut [[bool; 9]; 9]) -> bool {
    unsafe {
        !visited
            .get_unchecked(position.0)
            .get_unchecked(position.1 - 1)
            && (!visited
                .get_unchecked(position.0)
                .get_unchecked(position.1 - 2)
                || *visited
                    .get_unchecked(position.0 - 1)
                    .get_unchecked(position.1 - 1)
                || *visited
                    .get_unchecked(position.0 + 1)
                    .get_unchecked(position.1 - 1))
    }
}

fn can_proceed_right(position: &(usize, usize), visited: &mut [[bool; 9]; 9]) -> bool {
    unsafe {
        !visited
            .get_unchecked(position.0 + 1)
            .get_unchecked(position.1)
            && (!visited
                .get_unchecked(position.0 + 2)
                .get_unchecked(position.1)
                || *visited
                    .get_unchecked(position.0 + 1)
                    .get_unchecked(position.1 - 1)
                || *visited
                    .get_unchecked(position.0 + 1)
                    .get_unchecked(position.1 + 1))
    }
}

fn can_proceed_down(position: &(usize, usize), visited: &mut [[bool; 9]; 9]) -> bool {
    unsafe {
        !visited
            .get_unchecked(position.0)
            .get_unchecked(position.1 + 1)
            && (!visited
                .get_unchecked(position.0)
                .get_unchecked(position.1 + 2)
                || *visited
                    .get_unchecked(position.0 - 1)
                    .get_unchecked(position.1 + 1)
                || *visited
                    .get_unchecked(position.0 + 1)
                    .get_unchecked(position.1 + 1))
    }
}

fn can_proceed_left(position: &(usize, usize), visited: &mut [[bool; 9]; 9]) -> bool {
    unsafe {
        !visited
            .get_unchecked(position.0 - 1)
            .get_unchecked(position.1)
            && (!visited
                .get_unchecked(position.0 - 2)
                .get_unchecked(position.1)
                || *visited
                    .get_unchecked(position.0 - 1)
                    .get_unchecked(position.1 - 1)
                || *visited
                    .get_unchecked(position.0 - 1)
                    .get_unchecked(position.1 + 1))
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
??????R??????U??????????????????????????LD????D?
";
        let target = b"\
201
";

        test(input, target);
    }
}
