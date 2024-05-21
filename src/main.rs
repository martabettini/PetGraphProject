use petgraph::graph::Graph;
fn main() {
    //println!("Hello, world!");
    let mut graph = Graph::<&str, &str>::new();

    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");

    graph.add_edge(a, b, "A to B");
    graph.add_edge(b, c, "B to C");
    graph.add_edge(c, a, "C to A");

    println!("{:?}", graph);
}
