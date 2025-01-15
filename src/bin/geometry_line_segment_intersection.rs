// I/O boilerplate //

use std::io::Read;

const BUF_SIZE: usize = 32_768;

pub struct CustomBufWriter<'a, W: std::io::Write> {
    writer: &'a mut W,
    buffer: [u8; BUF_SIZE],
    buffer_pointer: usize,
}

impl<'a, W: std::io::Write> CustomBufWriter<'a, W> {
    pub fn new(writer: &'a mut W) -> Self {
        Self {
            writer,
            buffer: [0; BUF_SIZE],
            buffer_pointer: 0,
        }
    }

    pub fn flush(&mut self) {
        unsafe {
            self.writer
                .write_all(self.buffer.get_unchecked(..self.buffer_pointer))
                .unwrap_unchecked();
            self.buffer_pointer = 0;
        }
    }

    pub fn maybe_flush(&mut self, block_size: usize) {
        if self.buffer_pointer + block_size > BUF_SIZE {
            self.flush();
        }
    }

    pub fn add_byte(&mut self, byte: u8) {
        unsafe {
            self.buffer
                .as_mut_ptr()
                .add(self.buffer_pointer)
                .write(byte);
            self.buffer_pointer += 1;
        }
    }

    pub fn add_bytes(&mut self, buf: &[u8]) {
        unsafe {
            let len = buf.len();
            let ptr = self
                .buffer
                .get_unchecked_mut(self.buffer_pointer..)
                .as_mut_ptr();
            ptr.copy_from_nonoverlapping(buf.as_ptr(), len);
            self.buffer_pointer += len;
        }
    }
}

impl<W: std::io::Write> Drop for CustomBufWriter<'_, W> {
    fn drop(&mut self) {
        self.flush();
    }
}

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

pub trait AnyInt {
    fn to_anyint(buf: &[u8]) -> Self;
}
macro_rules! impl_anyint {
    (for $($t:ty),+) => {
        $(impl AnyInt for $t {
            #[allow(clippy::cast_lossless, clippy::cast_possible_wrap)]
            fn to_anyint(buf: &[u8]) -> Self {
                let (neg, digits) = match buf {
                    [b'-', digits @ ..] => (true, digits),
                    digits => (false, digits),
                };

                let result = unsafe {
                    digits.iter()
                        .map(|byte| (byte & 15) as $t)
                        .reduce(|acc, digit| acc * 10 + digit)
                        .unwrap_unchecked()
                };

                if neg {
                    -result
                } else {
                    result
                }
            }
        })*
    }
}
impl_anyint!(for i8, i16, i32, i64, i128, isize);

// problem //

/// There are two line segments: the first goes through the points (x<sub>1</sub>,y<sub>1</sub>) and (x<sub>2</sub>,y<sub>2</sub>), and the second goes through the points (x<sub>3</sub>,y<sub>3</sub>) and (x<sub>4</sub>,y<sub>4</sub>).
///
/// Your task is to determine if the line segments intersect, i.e., they have at least one common point.
///
/// <b>Input</b>
///
/// The first input line has an integer t: the number of tests.
///
/// After this, there are t lines that describe the tests. Each line has eight integers: x<sub>1</sub>, y<sub>1</sub>, x<sub>2</sub>, y<sub>2</sub>, x<sub>3</sub>, y<sub>3</sub>, x<sub>4</sub>, y<sub>4</sub>.
///
/// <b>Output</b>
///
/// For each test, print "YES" if the line segments intersect and "NO" otherwise.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ t ≤ 10<sup>5</sup></li>
/// <li>-10<sup>9</sup> ≤ x<sub>1</sub>, y<sub>1</sub>, x<sub>2</sub>, y<sub>2</sub>, x<sub>3</sub>, y<sub>3</sub>, x<sub>4</sub>, y<sub>4</sub> ≤ 10<sup>9</sup></li>
/// <li>(x<sub>1</sub>, y<sub>1</sub> != (x<sub>2</sub>, y<sub>2</sub>) </li>
/// <li>(x<sub>3</sub>, y<sub>3</sub> != (x<sub>4</sub>, y<sub>4</sub>) </li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');
    let mut writer = CustomBufWriter::new(out);

    let t = unsafe { u32::to_posint(iter.next().unwrap_unchecked()) };

    for _ in 0..t {
        let x1 = unsafe { i64::to_anyint(iter.next().unwrap_unchecked()) };
        let y1 = unsafe { i64::to_anyint(iter.next().unwrap_unchecked()) };
        let x2 = unsafe { i64::to_anyint(iter.next().unwrap_unchecked()) };
        let y2 = unsafe { i64::to_anyint(iter.next().unwrap_unchecked()) };
        let x3 = unsafe { i64::to_anyint(iter.next().unwrap_unchecked()) };
        let y3 = unsafe { i64::to_anyint(iter.next().unwrap_unchecked()) };
        let x4 = unsafe { i64::to_anyint(iter.next().unwrap_unchecked()) };
        let y4 = unsafe { i64::to_anyint(iter.next().unwrap_unchecked()) };

        writer.maybe_flush(4);
        writer.add_bytes(if intersects(x1, y1, x2, y2, x3, y3, x4, y4) {
            b"YES\n"
        } else {
            b"NO\n"
        });
    }
}

#[allow(clippy::too_many_arguments)]
fn intersects(x1: i64, y1: i64, x2: i64, y2: i64, x3: i64, y3: i64, x4: i64, y4: i64) -> bool {
    let o1 = point_loc(x1, y1, x2, y2, x3, y3);
    if o1 == 0 && intersects_collinear(x1, y1, x2, y2, x3, y3) {
        return true;
    }
    let o2 = point_loc(x1, y1, x2, y2, x4, y4);
    if o2 == 0 && intersects_collinear(x1, y1, x2, y2, x4, y4) {
        return true;
    }

    let o3 = point_loc(x1, y1, x3, y3, x4, y4);
    if o3 == 0 && intersects_collinear(x3, y3, x4, y4, x1, y1) {
        return true;
    }
    let o4 = point_loc(x2, y2, x3, y3, x4, y4);
    if o4 == 0 && intersects_collinear(x3, y3, x4, y4, x2, y2) {
        return true;
    }

    o1 != o2 && o3 != o4
}

// checks that "x3" will be between "x1" and "x2", or that "y3" is between "y1" and "y2" (argument order matters)
fn intersects_collinear(x1: i64, y1: i64, x2: i64, y2: i64, x3: i64, y3: i64) -> bool {
    if x1 < x2 {
        return x1 <= x3 && x3 <= x2;
    }
    if x2 < x1 {
        return x2 <= x3 && x3 <= x1;
    }
    if y1 < y2 {
        return y1 <= y3 && y3 <= y2;
    }
    // additional check not needed given constraint
    y2 <= y3 && y3 <= y1
}

// get location of the 3rd point relative to an infinite line containing points 1 and 2 (argument order does not matter)
fn point_loc(x1: i64, y1: i64, x2: i64, y2: i64, x3: i64, y3: i64) -> i8 {
    match ((y3 - y1) * (x2 - x1) - (x3 - x1) * (y2 - y1)).cmp(&0) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
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
5
1 1 5 3 1 2 4 3
1 1 5 3 1 1 4 3
1 1 5 3 2 3 4 1
1 1 5 3 2 4 4 1
1 1 5 3 3 2 7 4
";
        let target = b"\
NO
YES
YES
YES
YES
";

        test(input, target);
    }

    #[test]
    fn test_parallel() {
        let input = b"\
2
1 1 10 1 1 2 10 2
1 1 1 10 2 1 2 10
";
        let target = b"\
NO
NO
";

        test(input, target);
    }

    #[test]
    fn test_collinear() {
        let input = b"\
6
-5 -5 0 0 1 1 10 10
0 0 1 1 1 1 10 10
0 0 1 1 10 10 -1 -1
0 0 1 1 0 0 1 1
10 10 -1 -1 0 0 1 1
10 10 1 1 0 0 -1 -1
";
        let target = b"\
NO
YES
YES
YES
YES
NO
";

        test(input, target);
    }
}
