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

/// There is a line that goes through the points p<sub>1</sub>=(x<sub>1</sub>,y<sub>1</sub>) and p<sub>2</sub>=(x<sub>2</sub>,y<sub>2</sub>). There is also a point p<sub>3</sub>=(x<sub>3</sub>,y<sub>3</sub>).
///
/// Your task is to determine whether p<sub>3</sub> is located on the left or right side of the line or if it touches the line when we are looking from p<sub>1</sub> to p<sub>2</sub>.
///
/// <b>Input</b>
///
/// The first input line has an integer t: the number of tests.
///
/// After this, there are t lines that describe the tests. Each line has six integers: x<sub>1</sub>, y<sub>1</sub>, x<sub>2</sub>, y<sub>2</sub>, x<sub>3</sub> and y<sub>3</sub>.
///
/// <b>Output</b>
///
/// For each test, print "LEFT", "RIGHT" or "TOUCH".
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ t ≤ 10<sup>5</sup></li>
/// <li>-10<sup>9</sup> ≤ x<sub>1</sub>, y<sub>1</sub>, x<sub>2</sub>, y<sub>2</sub>, x<sub>3</sub>, y<sub>3</sub> ≤ 10<sup>9</sup></li>
/// <li>x<sub>1</sub> != x<sub>2</sub>, y<sub>1</sub> != y<sub>2</sub> </li>
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

        writer.maybe_flush(6);
        writer.add_bytes(
            match ((y3 - y1) * (x2 - x1) - (x3 - x1) * (y2 - y1)).cmp(&0) {
                std::cmp::Ordering::Less => b"RIGHT\n",
                std::cmp::Ordering::Equal => b"TOUCH\n",
                std::cmp::Ordering::Greater => b"LEFT\n",
            },
        );
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
3
1 1 5 3 2 3
1 1 5 3 4 1
1 1 5 3 3 2
";
        let target = b"\
LEFT
RIGHT
TOUCH
";

        test(input, target);
    }
}
