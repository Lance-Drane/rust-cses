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
            std::mem::transmute::<std::str::SplitAsciiWhitespace<'_>, std::str::SplitAsciiWhitespace<'_>>(slice.split_ascii_whitespace())
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
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let n_nodes: usize = scan.token();
    let n_connections: usize = scan.token();
    let mut graph = Graph::new(n_nodes + 1, n_connections << 1);

    for _ in 0..n_connections {
        let a = scan.token();
        let b = scan.token();
        graph.add_undirected_edge(a, b);
    }

    // 1 = team 1, 2 = neutral, 3 = team 2
    let mut teams = vec![DEFAULT_TEAM; n_nodes + 1];
    let mut stack = Vec::with_capacity(n_nodes);

    for idx in 1..=n_nodes {
        if teams[idx] == DEFAULT_TEAM {
            stack.push((1_u8, idx));
            while let Some((team, node)) = stack.pop() {
                let other_team = team ^ 2;
                for (_, vertex) in graph.adj_list(node) {
                    match teams.get(vertex) {
                        Some(t) if *t == team => {
                            out.write_all(b"IMPOSSIBLE\n").unwrap();
                            return;
                        }
                        Some(&DEFAULT_TEAM) => {
                            teams[vertex] = other_team;
                            stack.push((other_team, vertex));
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    for team in teams.into_iter().skip(1) {
        out.write_all(if team == 1 { b"1 " } else { b"2 " })
            .unwrap();
    }
}

// entrypoints //

fn main() {
    let scan = UnsafeScanner::new(std::io::stdin());
    let mut out = std::io::BufWriter::with_capacity(32_768, std::io::stdout().lock());
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
