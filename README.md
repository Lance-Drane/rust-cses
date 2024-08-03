# Rust - CSES Solutions

My solutions for the competitive programming problem set from https://cses.fi/, written in Rust.

## environment

The CSES environment is defined [here](https://cses.fi/howto/). At time of writing, it uses rustc 1.66.1, the 2018 edition of Rust and uses the `rand` crate. Note that I will potentially use later configurations than all of these; however, the files are verified to be accepted.

`cargo install` should get you started.

## running a problem

Since only single-file submissions are accepted on CSES, everything gets stored in the `src/bin` directory per [Cargo conventions](https://doc.rust-lang.org/cargo/reference/cargo-targets.html?highlight=src%2Fbin#binaries). You can then run your file with `cargo run --bin <bin-name> < <STDIN_FILE>`. (Add the `--release` flag if testing for performance.)

Because I don't want to have to manually add `bin` paths inside of Cargo.toml, the file names all follow a particular pattern of `<CATEGORY>_<PROBLEM_NAME>` - `<CATEGORY>` is always condensed to be a single-word summation, while `<PROBLEM_NAME>` is a snake-case representation of the entire problem name.

## boilerplate

To save yourself from having to write the same I/O code for every single problem set, you can start from the `src/bin/0_cses_template.rs` file. You can modify the `solve` function from there, it's just expecting two integers from stdin like `"4 -7"`.

(In a real project, all of the I/O boilerplate would be written in `src/lib.rs` and imported directly in the scripts. This is not done here - CSES submissions require a single file, I don't want to alter files just for submitting.)

## design decisions

- Competitive programming in general de-emphasizes error handling, particularly regarding stdin and stdout. It's always assumed that stdin will match the constraints, focus on speed and clarity over safety here. In a real world application, obviously don't neglect safety.
  - All entries on CSES use ASCII, so you can optimize slightly by skipping Rust's UTF-8 checks. DON'T do this in a real-world problem.
  - Since CSES does not seem to have interactive problems (from what I've seen), and since input size doesn't seem to go over 2MB, you can read all stdin at once to minimize I/O calls. Additionally, you can get away with just parsing generic whitespace instead of handling newlines as a special case; you can predict what the next line will be either from the problem statement or from the first "tokens" parsed, so there's never really a reason to read line-by-line. In a real-world application, you'll probably be processing potentially arbitrary files, so you'd probably want to make use of one of the `BufRead` trait's functions instead.
  - Similarly, you definitely don't want to be calling `unwrapped_unchecked()` at any point in a real-world application during I/O. Handle the `Err` from the Result properly.
  - In certain cases, you can allocate huge arrays on the stack, though be warned that you have a limit of 2MB!
- Strong typing and powerful compilers are great at catching errors as you're writing the code, instead of after you've run the code.
- Be concise in how much code is written. Minimal expressions and statements have an appealing aesthetic on their own.
- General goal is to be among the fastest applications, but not _the_ fastest. **NOTE**: in proper competitive programming environments, you really should try to be the fastest (in both code completion speed and execution speed).
- Speed of _writing_ the application is not emphasized. This is important in a real competitive programming environment, but as the CSES problem set is not timed, I'm emphasizing performance, robustness, and clarity.
- Immutability is preferred, but not obligatory. Sometimes, mutability is the best way to handle something - these scripts don't need to worry about multithreading. Iterators are great for helping out with writing immutable code while not dealing with the cost of allocating another vector.
- I try to use functional-style programming (using Rust iterator patterns) as much as possible, but sometimes imperative programming (or, rarely, OO programming) is just the best way to solve a problem.
- Don't cheat with lookup tables (or small lookup tables which could potentially be invalidated by a new submission under the listed constraints). Obviously, there are many problems where that's the fastest case, and in a real-world application you would of course use the pre-computed values. But where's the fun in that?

## linting

`cargo clippy -- -Wclippy::pedantic` should catch all of the important lints.

## testing

Run `cargo test`, all files should have basic unit tests by default. Note that successful test execution does not guarantee an accepted code submission on CSES, as there's a maximum time limit (usually 1 second) and memory usage.

When editing a test, you should generally only need to change the input and the expected output variables.

Note that these tests are NOT representative of the actual test cases on CSES - in general, I'm testing for general correctness and edge-cases.

## Credits

- [EbTech](https://github.com/EbTech/rust-algorithms/commit/6198cf16f667859ca60babb4b2264b9b9d039ade) : scanner boilerplate, well-designed algorithm implementations
- [The Rust Performance Book](https://nnethercote.github.io/perf-book/introduction.html) - some great tips on how to improve performance
- [itoap](https://github.com/Kogia-sima/itoap) and authors - single-file version of this crate used in several files which require outputting integers
- [indexset](https://github.com/brurucy/indexset) and authors - good implementation of an ordered set with indexes, serves as a Rust replacement for the [GCC policy-based tree](https://gcc.gnu.org/onlinedocs/libstdc++/manual/policy_data_structures_design.html#pbds.design.container.tree). Single-file version of this crate (and its [ftree dependency](https://github.com/brurucy/ftree)) used in some problems.
