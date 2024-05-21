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

use std::collections::VecDeque;

/// credit: `EbTech`
/// A compact graph representation. Edges are numbered in order of insertion.
/// Each adjacency list consists of all edges pointing out from a given vertex.
pub struct Graph {
    /// Maps a vertex id to the first edge in its adjacency list.
    first: Vec<Option<usize>>,
    /// Maps an edge id to the next edge in the same adjacency list.
    next: Vec<Option<usize>>,
    /// Maps an edge id to the vertex that it points to.
    endp: Vec<usize>,
}

impl Graph {
    /// Initializes a graph with vmax vertices and no edges. To reduce
    /// unnecessary allocations, `emax_hint` should be close to the number of
    /// edges that will be inserted.
    #[must_use]
    pub fn new(vmax: usize, emax_hint: usize) -> Self {
        Self {
            first: vec![None; vmax],
            next: Vec::with_capacity(emax_hint),
            endp: Vec::with_capacity(emax_hint),
        }
    }

    /// Returns the number of vertices.
    #[must_use]
    pub fn num_v(&self) -> usize {
        self.first.len()
    }

    /// Returns the number of edges, double-counting undirected edges.
    #[must_use]
    pub fn num_e(&self) -> usize {
        self.endp.len()
    }

    /// Adds a directed edge from u to v.
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.next.push(self.first[u]);
        self.first[u] = Some(self.num_e());
        self.endp.push(v);
    }

    /// An undirected edge is two directed edges. If edges are added only via
    /// this funcion, the reverse of any edge e can be found at e^1.
    pub fn add_undirected_edge(&mut self, u: usize, v: usize) {
        self.add_edge(u, v);
        self.add_edge(v, u);
    }

    /// Gets vertex u's adjacency list.
    #[must_use]
    pub fn adj_list(&self, u: usize) -> AdjListIterator {
        AdjListIterator {
            graph: self,
            next_e: self.first[u],
        }
    }
}

/// An iterator for convenient adjacency list traversal.
pub struct AdjListIterator<'a> {
    graph: &'a Graph,
    next_e: Option<usize>,
}

impl<'a> Iterator for AdjListIterator<'a> {
    type Item = (usize, usize);

    /// Produces an outgoing edge and vertex.
    fn next(&mut self) -> Option<Self::Item> {
        self.next_e.map(|e| {
            let v = self.graph.endp[e];
            self.next_e = self.graph.next[e];
            (e, v)
        })
    }
}

/// Syrjälä's network has n computers and m connections. Your task is to find out if Uolevi can send a message to Maija, and if it is possible, what is the minimum number of computers on such a route.
///
/// <b>Input</b>
///
/// The first input line has two integers n and m: the number of computers and connections. The computers are numbered 1,2,...,n. Uolevi's computer is 1 and Maija's computer is n.
///
/// Then, there are m lines describing the connections. Each line has two integers a and b: there is a connection between those computers.
///
/// Every connection is between two different computers, and there is at most one connection between any two computers.
///
/// <b>Output</b>
///
/// If it is possible to send a message, first print k: the minimum number of computers on a valid route. After this, print an example of such a route. You can print any valid solution.
///
/// If there are no routes, print "IMPOSSIBLE".
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>2 ≤ n ≤ 10<sup>5</sup></li>
/// <li>1 ≤ m ≤ 2 * 10<sup>5</sup></li>
/// <li>1 ≤ a,b ≤ n </li>
/// </ul>
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let n_nodes: usize = scan.token();
    let n_connections: usize = scan.token();
    let mut graph = Graph::new(n_nodes + 1, n_connections << 1);

    for _ in 0..n_connections {
        let a = scan.token();
        let b = scan.token();
        graph.add_undirected_edge(a, b);
    }

    // if value in "parents" is not 0, we have already found its parent
    let mut parents = vec![0; n_nodes + 1];
    let mut queue = VecDeque::with_capacity(n_nodes);
    queue.push_back((0_u32, 1));

    let mut additional_comps = usize::MAX;

    'qloop: while let Some((depth, node)) = queue.pop_front() {
        for (_, vertex) in graph.adj_list(node) {
            if parents[vertex] == 0 {
                parents[vertex] = node;
                if vertex == n_nodes {
                    additional_comps = depth as usize;
                    break 'qloop;
                }
                queue.push_back((depth + 1, vertex));
            }
        }
    }

    if additional_comps == usize::MAX {
        out.write_all(b"IMPOSSIBLE\n").unwrap();
        return;
    }

    let mut answer = vec![0; additional_comps];
    let mut parent = *parents.last().unwrap();
    for ans in answer.iter_mut().rev() {
        *ans = parent;
        parent = parents[parent];
    }

    writeln!(out, "{}", additional_comps + 2).unwrap();
    out.write_all(b"1 ").unwrap();
    for a in answer {
        write!(out, "{a} ").unwrap();
    }
    writeln!(out, "{n_nodes}").unwrap();
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
5 5
1 2
1 3
1 4
2 3
5 4
";
        let target = b"\
3
1 4 5
";

        test(input, target);
    }

    #[test]
    fn test_empty() {
        let input = b"\
4 2
2 3
3 4
";
        let target = b"\
IMPOSSIBLE
";

        test(input, target);
    }

    #[test]
    fn test_immediate() {
        let input = b"\
2 1
1 2
";
        let target = b"\
2
1 2
";

        test(input, target);
    }
}
