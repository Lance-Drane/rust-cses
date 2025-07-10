// I/O boilerplate //

use std::fs::File;
use std::io::Read;

#[cfg(unix)]
fn stdin_raw() -> File {
    use std::os::fd::FromRawFd;

    unsafe { File::from_raw_fd(0) }
}

#[cfg(unix)]
fn stdout_raw() -> File {
    use std::os::fd::FromRawFd;

    unsafe { File::from_raw_fd(1) }
}

#[cfg(windows)]
fn stdin_raw() -> File {
    use std::os::windows::io::{AsRawHandle, FromRawHandle};

    unsafe { File::from_raw_handle(std::io::stdin().as_raw_handle()) }
}

#[cfg(windows)]
fn stdout_raw() -> File {
    use std::os::windows::io::{AsRawHandle, FromRawHandle};

    unsafe { File::from_raw_handle(std::io::stdout().as_raw_handle()) }
}

// problem //

/// Given a string, your task is to reorder its letters in such a way that it becomes a palindrome (i.e., it reads the same forwards and backwards).
///
/// <b>Input</b>
///
/// The only input line has a string of length n consisting of characters A–Z.
///
/// <b>Output</b>
///
/// Print a palindrome consisting of the characters of the original string. You may print any valid solution. If there are no solutions, print "NO SOLUTION".
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 10<sup>6</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &mut [u8], out: &mut W) {
    let (token, _) = scan.split_at_mut(scan.len() - 1);

    let mut counter = [0_u32; 26];
    for c in token.iter() {
        unsafe {
            *counter.get_unchecked_mut((*c - b'A') as usize) += 1;
        }
    }

    let mut odd_letter = b'\0';
    for (count, letter) in counter.iter().zip(b'A'..) {
        if count & 1 == 1 {
            if odd_letter != b'\0' {
                out.write_all(b"NO SOLUTION\n").unwrap();
                return;
            }
            odd_letter = letter;
        }
    }

    let mut iter = token.iter_mut();
    for (count, letter) in counter.iter().zip(b'A'..) {
        for _ in 0..(count >> 1) {
            unsafe {
                *iter.next().unwrap_unchecked() = letter;
                *iter.next_back().unwrap_unchecked() = letter;
            }
        }
    }

    if odd_letter != b'\0' {
        unsafe {
            *iter.next().unwrap_unchecked() = odd_letter;
        }
    }

    out.write_all(scan).unwrap();
}

// entrypoints //

fn main() {
    let mut buf_str = vec![];
    stdin_raw().read_to_end(&mut buf_str).unwrap();
    let mut out = stdout_raw();
    solve(&mut buf_str, &mut out);
}

#[cfg(test)]
mod test {
    use super::*;

    fn test(input: &[u8], target: &[u8]) {
        let mut out = Vec::with_capacity(target.len());
        solve(&mut input.to_owned(), &mut out);

        assert_eq!(out, target);
    }

    // NOTE: while CSES allows for any valid solution, we have a specific implementation.
    // Characters which come first in the alphabet come in at the beginning and end of the string
    // Characters at the end of the alphabet come in at the middle of the string

    #[test]
    fn test_example() {
        let input = b"\
AAAACACBA
";
        let target = b"\
AAACBCAAA
";

        test(input, target);
    }

    #[test]
    fn test_invalid() {
        let input = b"\
NOIX
";
        let target = b"\
NO SOLUTION
";

        test(input, target);
    }

    #[test]
    fn test_no_middle() {
        let input = b"\
REDRED
";
        let target = b"\
DERRED
";

        test(input, target);
    }

    #[test]
    fn test_long_middle() {
        let input = b"\
AADDDCC
";
        let target = b"\
ACDDDCA
";

        test(input, target);
    }
}
