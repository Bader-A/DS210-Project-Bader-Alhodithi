use plotters::prelude::*;
use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
#[cfg(test)]
mod tests;
//Bader Alhodithi
//DS 210
//Final Project
//Google dataset
type Node = u32; //here we define node type as u32

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("src/web-google.txt")?; //opening the google file
    let mut num_lines = 0; //counter for number of lines read from file

    let mut graph: HashMap<Node, Vec<Node>> = HashMap::new(); //here i initialize a hashmap for the graph, the keys are the nodes along with the neighbors as their value

    for line_result in BufReader::new(file).lines() { //iterating over the lines using bufreader
        let line = line_result?;

        let nodes: Result<Vec<Node>, _> = line  // here I parsed the line into a vector of 2 nodes.
            .split_whitespace()
            .map(|s| s.parse())
            .collect();

        match nodes { //using match to add nodes to the graph as pairs given that parsing is successfull
            Ok(nodes) => {
                let (node1, node2) = (nodes[0], nodes[1]);
                graph.entry(node1).or_default().push(node2);
            }
            Err(e) => {
                eprintln!("Ignoring line: {}, skipping: {:?}", line, e);  //printsd error if prasing failed (likely only for first few lines of dataset as they are comments.)
            }
        }

        num_lines += 1;  //line counter
        if num_lines >= (0.25 * 875713.0) as usize { //IMPORTANT: I am making it such that it only reads around 25% of the dataset as it would take a very long time otherwise.
            break;                                   //I played around with a few percentages, this is optimal for my computer. (25% of this dataset is large enough stil)
        }
    }
//visuals
    let root = SVGBackend::new("graph_visualization.svg", (800, 800)).into_drawing_area();
    root.fill(&WHITE)?; //here i am setting up the backend for making SVG visual of the graph, proportions are 800,800 but can be adjusted, likely wont make a difference tho.

    let mut chart = ChartBuilder::on(&root)  //chart for graph, 30 pixel marhins
        .margin(5)
        .set_all_label_area_size(30)
        .build_cartesian_2d(0u32..1000u32, 0u32..1000u32)?;

    chart.configure_mesh().draw()?; //graph mesh (grids)

    let mut visited = std::collections::HashSet::new(); //here i initialized a hashset to track which nodes were visited

    for node in graph.keys() {
        if !visited.contains(node) {
            dfs(&graph, *node, &mut visited, &|from, to| {
                let (x1, y1) = (from % 32 * 30, from / 32 * 30);//converting node coordinates to pixel coordinates.
                let (x2, y2) = (to % 32 * 30, to / 32 * 30);

                root.draw(&PathElement::new( //line segment between nodes in graph visual.
                    vec![(x1 as i32, y1 as i32), (x2 as i32, y2 as i32)],
                    &BLACK,
                ))?;
                Ok(())
            })?;
        }
    }

 //Analyzing the graph
let num_vertices = graph.len(); //here im analyzing the graph, this shows basics such as number of vertices and edges.
let num_edges: usize = graph.values().map(|neighbors| neighbors.len()).sum();

println!("Number of vertices: {}", num_vertices);
println!("Number of edges: {}", num_edges);

let mut degree_sum = 0; //here im computing the degree, meaning number of neighbors, each node has along with distribution of degrees among nodes.
let mut degree_distribution: HashMap<usize, usize> = HashMap::new();

for node in graph.keys() {
    let degree = graph.get(node).map(|neighbors| neighbors.len()).unwrap_or(0);
    degree_sum += degree;

    *degree_distribution.entry(degree).or_insert(0) += 1;
}

let avg_degree = degree_sum as f64 / num_vertices as f64;  //average degree of graph
println!("Average degree: {:.2}", avg_degree);

println!("Degree distribution:");  //printing degree distributions and counts
for (degree, count) in degree_distribution.iter() {
    println!("Degree {}: {} nodes", degree, count);
}

let degree_centrality = calc_degree(&graph);  //here we are doing a degree centrality analysis, Currently i made it such that it takes the first 50, but it can be increased
                                            // I only made it around 50 since it was a little messy, otherwise how much it iterates and takes can change.

//initally i tried getting centrality degree as a table but it kept giving me access violations for some reason?
println!("Node\tDegree\tCentrality");
for (node, degree) in degree_centrality.iter().take(50) {
    let centrality = if num_vertices > 1 {
        *degree as f64 / (num_vertices - 1) as f64
    } else {
        0.0
    };
    println!("{}\t{}\t{}", node, degree, centrality);
}

Ok(())
}

//here this computes degree, or number of neighbours, and returns a tuple of the node ID and degree. (ID to URL can be traced but i wasnt able to find the set for it)
fn calc_degree(graph: &HashMap<Node, Vec<Node>>) -> Vec<(Node, usize)> {
    let mut degree: Vec<(Node, usize)> = graph
        .keys()
        .map(|&k| (k, graph[&k].len()))
        .collect();
    degree.sort_unstable_by(|(_, a), (_, b)| b.cmp(a));
    degree
}



//this fn performs a depth first search on the graph, beggining with a speicified node and calls closure for visited edge.
fn dfs<F>(
    graph: &HashMap<Node, Vec<Node>>,
    start_node: Node,
    visited: &mut std::collections::HashSet<Node>,
    on_edge: &F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(Node, Node) -> Result<(), Box<dyn std::error::Error>>,
{//dfs traversal stack then pushes node.
    let mut stack = Vec::new();
    stack.push(start_node);

    while let Some(node) = stack.pop() { //calls nodes untill they all are reachable from starting node.
        if !visited.contains(&node) {//if node is not visited, mark as visited then call closure for each non-visted node
            visited.insert(node);

            if let Some(neighbors) = graph.get(&node) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        on_edge(node, *neighbor)?;
                        stack.push(*neighbor);
                    }
                }
            }
        }
    }

    Ok(())
}

//Collaborators: None (thats a given for the project anyways but wanted to be on the safe side.)
