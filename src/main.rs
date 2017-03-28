use std::fmt;

fn main() {
    let mut v = Vec::new();
    v.push(Edge(32));
    v.push(Edge(12));
    let g = new_graph(v);

    for e in g.edges() {
        println!("{}", e);
    }
}

trait Graph {
    type E: fmt::Display;
    type EVec: std::iter::IntoIterator;

    fn edges(&self) -> &Self::EVec;
}

struct Edge(i32);

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct MyGraph {
    edges: Vec<Edge>
}

fn new_graph(e: Vec<Edge>) -> MyGraph {
    MyGraph{edges: e}
}

impl Graph for MyGraph {
    type E = Edge;
    type EVec = Vec<Edge>;

    fn edges<'a>(&'a self) -> &'a Self::EVec {
        &self.edges
    }
}