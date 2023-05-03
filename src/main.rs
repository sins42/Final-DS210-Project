#[cfg(test)]
mod tests;
mod graph;
mod utils;

use utils::{read_file};
use graph::Graph;

fn main() {
    // load edges from the file 
    let edgelist = read_file("p2p-Gnutella04.txt");
    
    // create graph
    let mut graph = Graph::new();
    for (src, des) in edgelist{
        graph.add_edge(src, des);
    }

    // compute degree centrality for each node
    let centrality = graph.deg_centrality();

    // print centrality for each node in descending order 
    let mut sorted_centrality: Vec<_> = centrality.iter().collect();
    let mut count=0;
    sorted_centrality.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());
    println!("\nDegree Centrality of Top 5 nodes: \n");
    for (node, centrality) in &sorted_centrality{
        if count<5{
            println!("{}. {} has degree centrality: {:.3}", count+1, node, centrality);
            // find shortest path between two nodes
            let start_node = node;
            let end_node = sorted_centrality[count+1].0;
            if let Some(path) = graph.shortest_path(**start_node, *end_node) {
                println!("\tShortest path between {} and {}: {:?}", start_node, end_node, path);
            } else {
                println!("\tNo path found between {} and {}", start_node, end_node);
            }
            count+=1;
        }
    }
    
}