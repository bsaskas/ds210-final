
use std::collections::HashMap;
use std::collections::VecDeque;

//breadth-first search function to help compute the average distance between vectors
pub fn bfs(graph:&Graph,source:usize) -> Vec<usize> {
    //initializing distances vector,setting all to the maximum to check if a vector been visited.
    let mut distances = vec![usize::MAX; graph.n];
    let mut queue = VecDeque::new();
    distances[source] = 0;
    queue.push_back(source);

    // while loop to ensure all vectors in queue are visited
    while let Some(current) = queue.pop_front() {
        for &neighbor in &graph.outedges[current] {
            if distances[neighbor.vertex] == usize::MAX {
                //setting distance to the shortest path and adding vertex to queue
                distances[neighbor.vertex] = distances[current] + 1;
                queue.push_back(neighbor.vertex);
            }
        }
    }
    distances
}

//setting up types for the graph struct
pub type Vertex = usize;
pub type Edge = (Vertex, Vertex);
pub type AdjacencyList = Vec<Outedge>;

#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Outedge {
    pub vertex: Vertex
}

#[derive(Debug)]
pub struct Graph {
    pub n: usize,
    pub outedges: Vec<AdjacencyList>,
}
//implementing different functions for the Graph struct
impl Graph {
    //creating the directed graph, which is represented by number of vertices followed by an array of outedges.
    //each array represents a vector and its outgoing edges
    pub fn create_directed(n:usize,edges:&Vec<Edge>) -> Graph {
        let mut outedges = vec![vec![];n];
        for (u, v) in edges {
            outedges[*u].push(Outedge{vertex: *v});
        }
        Graph{n,outedges}
    }

    //using the length of an outedge list to represent num. of connections
    pub fn numneighbors(graph:&Graph,vertex:Vertex) -> usize {
        graph.outedges[vertex].len()
    }

    //iterating through all vectors to find avg. num of neighbors
    pub fn averagenumneighbors(graph:&Graph) -> usize {
        let mut count:usize = 0;
        for x in 0..graph.n {
            count += Graph::numneighbors(&graph,x);
        }
        count/graph.n
    }
    //finding the distribution of degrees among vectors using a Hashmap and returning a list of tuples
    //representing a number of connections and the frequency
    pub fn degreedistribution(&self) -> Vec<(usize, usize)> {
        let mut counts: HashMap<usize, usize> = HashMap::new();
        for x in 0..self.n {
            let deg = Graph::numneighbors(&self, x);
            *counts.entry(deg).or_insert(0) += 1;
        }

        //converting the counts into a list of tuples
        let mut result: Vec<(usize, usize)> = counts.into_iter().collect();

        //sorting the result by count in descending order
        result.sort_by(|a, b| b.1.cmp(&a.1));
        result
    }
    //using bfs search to compute the average distance between vectors
    pub fn averagedistance(&self) -> f64 {
        let mut totaldistance = 0;
        let mut pairs = 0;
        let mut milgram = 0;
        for node in 0..self.n {
            let distance = bfs(self, node);
            for new in 0..self.n {
                if node == new {
                    continue;
                }
                //checking for overflow and if there's a valid distance
                if distance[new] != usize::MAX {
                    totaldistance += distance[new] as u64;
                    pairs += 1;
                }
                if distance[new] > 6 && distance[new] <100 {
                    milgram += 1
                }
                
            }
        }
        println!("There are {:?} pairs that are farther than 6 degrees of separation.",milgram);
        totaldistance as f64 / pairs as f64
    }
}


