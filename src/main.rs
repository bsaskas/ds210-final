mod graph;
use crate::graph::Graph;
use std::fs::File;
use std::io::prelude::*;
use crate::graph::Outedge;

//reads data file and produces a list of tuples representing Facebook accounts and who they are following; data is directed and unweighted

fn read_file(path: &str) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        let x = v[0].parse::<usize>().unwrap();
        let y = v[1].parse::<usize>().unwrap();
        result.push((x, y));
    }
    return result;
}

fn main() {
    //importing facebook dataset: unweighted, directed dataset where a vertex represents a user and an edge represents a follow.
    let edges = read_file("facebook_combined.txt");
    let n:usize = 4039;
    //creating directed Graph from data
    let graph = graph::Graph::create_directed(n, &edges);
    //printing out average number of neighbors, average distance, and distribution of degrees.
    println!("Average number of neighbors: {:?}", Graph::averagenumneighbors(&graph));
    println!("Average distance between Vertices: {:?}",graph.averagedistance());
    println!("Distribution of Degrees: {:?}", graph.degreedistribution());
}

#[test]
fn test_createdirected() {
    let testn = 3;
    let testedges = vec![(0,1),(0, 2), (1, 2), (2, 3)];
    let testgraph = graph::Graph::create_directed(testn,&testedges);
    assert_eq!(testgraph.n,testn);
    assert_eq!(testgraph.outedges[0],vec![Outedge { vertex: 1 }, Outedge { vertex: 2 }]);
}

#[test]
fn test_degreedistribution() {
    let testedges = vec![(0, 1), (0, 2), (1,0),(1,2),(2,1)];
    let testgraph = Graph::create_directed(3, &testedges);
    let distribution = testgraph.degreedistribution();
    assert_eq!(distribution.len(), 2);
    assert_eq!(distribution[0], (2, 2));
    assert_eq!(distribution[1], (1, 1)); 
}

#[test]
fn test_averagedistance() {
    let edges = vec![(0, 1), (0, 2), (1, 2), (2, 3), (3, 4)];
    let graph = Graph::create_directed(5, &edges);
    let average_distance = graph.averagedistance();
    let correct:f64 = (17 as f64/10 as f64);
    assert_eq!(average_distance, correct);
}