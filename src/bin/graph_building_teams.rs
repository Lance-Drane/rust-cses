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

// problem //

pub struct EdgeData {
    // label of next node
    to: usize,
    // index of the previously added edge, or usize::MAX if this was the first edge added. Edges are analyzed in reverse order of how they were added.
    next_edge_idx: usize,
}

pub struct Graph {
    // the head points to the index of the last added edge in "self.edges", or usize::MAX if not connected. the index references the node label
    heads: Vec<usize>,
    // the index references the order in which the edge was added
    edges: Vec<EdgeData>,
}

impl Graph {
    /// Initializes a graph with `num_nodes` vertices and no edges. To reduce
    /// unnecessary allocations, `num_edges` should be close to the number of
    /// edges that will be inserted. Note that `usize::MAX` is meant to be a placeholder value signifying that there are no additional edges.
    #[must_use]
    pub fn new(num_nodes: usize, num_edges: usize) -> Self {
        Self {
            heads: vec![usize::MAX; num_nodes],
            edges: Vec::with_capacity(num_edges),
        }
    }

    /// Adds a directed edge with a cost factor.
    pub fn add_edge(&mut self, from: usize, to: usize) {
        let head_ptr = unsafe { self.heads.get_unchecked_mut(from) };
        let next_edge = EdgeData {
            to,
            next_edge_idx: *head_ptr,
        };
        let edges_len = self.edges.len();
        *head_ptr = edges_len;

        unsafe {
            self.edges.as_mut_ptr().add(edges_len).write(next_edge);
            self.edges.set_len(edges_len + 1);
        }
    }

    /// An undirected edge is two directed edges. If edges are added only via
    /// this function, the reverse of any edge e can be found at e^1.
    pub fn add_undirected_edge(&mut self, u: usize, v: usize) {
        self.add_edge(u, v);
        self.add_edge(v, u);
    }

    /// Gets vertex `node_idx`'s adjacency list.
    #[must_use]
    pub fn adj_list(&'_ self, node_idx: usize) -> AdjListIterator<'_> {
        AdjListIterator {
            edges: &self.edges,
            next_edge_idx: self.heads[node_idx],
        }
    }
}

/// An iterator for convenient adjacency list traversal.
pub struct AdjListIterator<'a> {
    edges: &'a [EdgeData],
    next_edge_idx: usize,
}

impl Iterator for AdjListIterator<'_> {
    type Item = (usize, usize);

    /// Produces an outgoing edge, the next vertex, and the last cost.
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_edge_idx {
            usize::MAX => None,
            idx => {
                let next_edge = &self.edges[idx];
                self.next_edge_idx = next_edge.next_edge_idx;
                Some((idx, next_edge.to))
            }
        }
    }
}

const DEFAULT_TEAM: u8 = 2;

/// There are n pupils in Uolevi's class, and m friendships between them. Your task is to divide the pupils into two teams in such a way that no two pupils in a team are friends. You can freely choose the sizes of the teams.
///
/// <b>Input</b>
///
/// The first input line has two integers n and m: the number of pupils and friendships. The pupils are numbered 1,2,...,n.
///
/// Then, there are m lines describing the friendships. Each line has two integers a and b: pupils a and b are friends.
///
/// Every friendship is between two different pupils. You can assume that there is at most one friendship between any two pupils.
///
/// <b>Output</b>
///
/// Print an example of how to build the teams. For each pupil, print "1" or "2" depending on to which team the pupil will be assigned. You can print any valid team.
///
/// If there are no solutions, print "IMPOSSIBLE".
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 10<sup>5</sup></li>
/// <li>1 ≤ m ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ a,b ≤ n </li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut iter = scan.split(|n| *n <= b' ');
    let mut writer = CustomBufWriter::new(out);

    let n_nodes = unsafe { usize::to_posint(iter.next().unwrap_unchecked()) };
    let n_connections = unsafe { usize::to_posint(iter.next().unwrap_unchecked()) };
    let mut graph = Graph::new(n_nodes + 1, n_connections << 1);

    for _ in 0..n_connections {
        let a = unsafe { usize::to_posint(iter.next().unwrap_unchecked()) };
        let b = unsafe { usize::to_posint(iter.next().unwrap_unchecked()) };
        graph.add_undirected_edge(a, b);
    }

    // 1 = team 1, 2 = neutral, 3 = team 2
    let mut teams = vec![DEFAULT_TEAM; n_nodes + 1];
    let mut stack = Vec::with_capacity(n_nodes);

    for idx in 1..(n_nodes + 1) {
        if teams[idx] == DEFAULT_TEAM {
            teams[idx] = 1;
            stack.push((1_u8, idx));
            while let Some((team, node)) = stack.pop() {
                let other_team = team ^ 2;
                for (_, vertex) in graph.adj_list(node) {
                    match teams.get(vertex) {
                        Some(&DEFAULT_TEAM) => {
                            teams[vertex] = other_team;
                            stack.push((other_team, vertex));
                        }
                        Some(t) if *t == team => {
                            writer.add_bytes(b"IMPOSSIBLE\n");
                            return;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    for team in teams.into_iter().skip(1) {
        writer.maybe_flush(2);
        writer.add_bytes(if team == 1 { b"1 " } else { b"2 " });
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
5 3
1 2
1 3
4 5
";
        let target = b"\
1 2 2 1 2 ";

        test(input, target);
    }

    #[test]
    fn test_impossible() {
        let input = b"\
3 3
1 2
2 3
3 1
";
        let target = b"\
IMPOSSIBLE
";

        test(input, target);
    }
}
