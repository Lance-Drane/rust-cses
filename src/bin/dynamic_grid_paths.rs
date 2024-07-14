// I/O boilerplate //

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

const MODULO: u32 = 1_000_000_007;

/// Consider an n * n grid whose squares may have traps. It is not allowed to move to a square with a trap.
///
/// Your task is to calculate the number of paths from the upper-left square to the lower-right square. You can only move right or down.
///
/// <b>Input</b>
///
/// The first input line has an integer n: the size of the grid.
///
/// After this, there are n lines that describe the grid. Each line has n characters: . denotes an empty cell, and * denotes a trap.
///
/// <b>Output</b>
///
/// Print the number of paths modulo 10^9+7.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 1000</li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');

    let dimension = unsafe { usize::to_posint(iter.next().unwrap_unchecked()) };
    let size = dimension * dimension;
    let mut counters = vec![0_u32; size];

    for (i, point) in unsafe { iter.next().unwrap_unchecked().iter().enumerate() } {
        if *point == b'*' {
            break;
        }
        unsafe {
            *counters.get_unchecked_mut(i) = 1;
        }
    }

    if counters[0] == 0 {
        out.write_all(b"0\n").unwrap();
        return;
    }
    let mut first_col_terminate = false;

    for (i, row) in (dimension..size)
        .step_by(dimension)
        .map(|idx| (idx, unsafe { iter.next().unwrap_unchecked() }))
    {
        if !first_col_terminate {
            if unsafe { *row.get_unchecked(0) } == b'*' {
                first_col_terminate = true;
            } else {
                unsafe {
                    *counters.get_unchecked_mut(i) = 1;
                }
            }
        }
        for (j, _) in row.iter().enumerate().skip(1).filter(|(_, c)| **c == b'.') {
            let idx = i + j;
            let mut count = unsafe {
                counters.get_unchecked(idx - 1) + counters.get_unchecked(idx - dimension)
            };
            if count >= MODULO {
                count -= MODULO;
            }
            unsafe {
                *counters.get_unchecked_mut(idx) = count;
            }
        }
    }

    writeln!(out, "{}", counters[size - 1]).unwrap();
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
4
....
.*..
...*
*...
";
        let target = b"\
3
";

        test(input, target);
    }

    #[test]
    fn test_one() {
        let input = b"\
1
.
";
        let target = b"\
1
";

        test(input, target);
    }

    #[test]
    fn test_start_trap() {
        let input = b"\
2
*.
..
";
        let target = b"\
0
";

        test(input, target);
    }
}
