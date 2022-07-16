#![allow(dead_code)]
use itertools::Itertools;
use std::{collections::HashSet, rc::Rc, str::FromStr};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum NodeType {
    Start,
    End,
    Large,
    Small,
}

impl FromStr for NodeType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.chars().all(|c| c.is_alphabetic()) {
            return Err(());
        }

        if s == "start" {
            return Ok(Self::Start);
        }

        if s == "end" {
            return Ok(Self::End);
        }

        Ok(match s.chars().any(|c| c.is_uppercase()) {
            true => Self::Large,
            false => Self::Small,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    id: String,
    type_: NodeType,
}

impl Node {
    fn new(id: String, type_: NodeType) -> Self {
        Self { id, type_ }
    }

    fn to_dot(&self) -> String {
        self.id.clone()
    }
}

impl FromStr for Node {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Node {
            id: s.to_string(),
            type_: s.parse()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Edge {
    from: Rc<Node>,
    to: Rc<Node>,
}

impl Edge {
    fn new(from: Rc<Node>, to: Rc<Node>) -> Self {
        Self { from, to }
    }

    fn to_dot(&self) -> String {
        format!("{} -- {}", self.from.to_dot(), self.to.to_dot())
    }
}

impl FromStr for Edge {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("-");

        let fst = parts.next().ok_or(())?;
        let snd = parts.next().ok_or(())?;

        let fst_node: Node = fst.parse()?;
        let snd_node: Node = snd.parse()?;

        Ok(Edge {
            from: Rc::new(fst_node),
            to: Rc::new(snd_node),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Graph {
    edges: HashSet<Rc<Edge>>,
}

impl Graph {
    fn new(edges: HashSet<Rc<Edge>>) -> Self {
        Self { edges }
    }

    fn nodes(&self) -> HashSet<Rc<Node>> {
        let mut nodes = HashSet::new();
        for edge in self.edges.iter() {
            nodes.insert(edge.from.clone());
            nodes.insert(edge.to.clone());
        }
        nodes
    }

    fn to_dot(&self) -> String {
        let header = "graph {\n\t";
        let body = self.edges.iter().map(|e| e.to_dot()).join("\n\t");
        let footer = "\n}";
        format!("{}{}{}", header, body, footer)
    }

    fn neighbors(&self, node: &dyn AsRef<Node>) -> HashSet<Rc<Node>> {
        let mut neighbors = HashSet::new();
        for edge in self.edges.iter() {
            if *edge.to == *node.as_ref() {
                neighbors.insert(edge.from.clone());
            } else if *edge.from == *node.as_ref() {
                neighbors.insert(edge.to.clone());
            }
        }
        neighbors
    }

    fn get_start(&self) -> Option<Rc<Node>> {
        self.nodes()
            .iter()
            .filter(|&node| node.type_ == NodeType::Start)
            .next()
            .and_then(|s| Some(s.clone()))
    }

    fn _count_paths_part_1(&self, node: &Rc<Node>, mut visited: HashSet<Rc<Node>>) -> usize {
        if node.type_ == NodeType::End {
            return 1;
        }

        if node.type_ != NodeType::Large {
            visited.insert(node.clone());
        }

        let mut num_paths = 0;

        for neighbor in self.neighbors(node).iter() {
            if !visited.contains(neighbor) {
                num_paths += self._count_paths_part_1(neighbor, visited.clone())
            }
        }
        return num_paths;
    }

    fn count_paths_part_1(&self) -> Option<usize> {
        let start = self.get_start()?;
        let visited: HashSet<Rc<Node>> = HashSet::new();
        Some(self._count_paths_part_1(&start, visited))
    }

    fn _count_paths_part_2(
        &self,
        node: &Rc<Node>,
        mut visited: HashSet<Rc<Node>>,
        mut visited_small_node_twice: bool,
    ) -> usize {
        // Return if we reach end.
        if node.type_ == NodeType::End {
            return 1;
        }

        // Path failed because we revisited a small node. Return 0.
        if visited_small_node_twice && visited.contains(node) {
            return 0;
        }

        // Path failed because we arrived back at start. Return 0.
        if node.type_ == NodeType::Start && visited.contains(node) {
            return 0;
        }

        // If the node is small and we already visited it, latch visited_small_node_twice to true
        if node.type_ == NodeType::Small && visited.contains(node) {
            visited_small_node_twice = true;
        };

        // If node is not large, add it to the heap of visited nodes.
        if !(node.type_ == NodeType::Large) {
            visited.insert(node.clone());
        }

        // Counter for paths
        let mut num_paths = 0;
        // Recurse into neighbors
        for neighbor in self.neighbors(node).iter() {
            num_paths +=
                self._count_paths_part_2(neighbor, visited.clone(), visited_small_node_twice);
        }
        num_paths
    }
}

impl FromStr for Graph {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut edges = HashSet::new();
        for line in s.trim().lines() {
            let edge: Edge = line.parse()?;
            edges.insert(Rc::new(edge));
        }
        Ok(Graph { edges })
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    const TEST_INPUT: &str = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end";
    const TEST_INPUT_PATH: &str = "input/day12_test.txt";
    const INPUT_PATH: &str = "input/day12.txt";

    #[test]
    fn test_graph_from_str() {
        let graph: Graph = TEST_INPUT.parse().unwrap();
        assert_eq!(graph.edges.len(), 7);
        assert_eq!(graph.nodes().len(), 6);
    }

    #[test]
    fn test_graph_to_dot() {
        let graph: Graph = TEST_INPUT.parse().unwrap();
        fs::write("output/day_12_test_graph_display.dot", graph.to_dot()).unwrap();
    }

    #[test]
    fn test_graph_neigbors() {
        let graph: Graph = TEST_INPUT.parse().unwrap();
        let start = graph.get_start().unwrap();

        let neighbors = graph.neighbors(&start);

        assert_eq!(neighbors.len(), 2);

        for neighbor in neighbors.iter() {
            assert!(neighbor.id == "A" || neighbor.id == "b")
        }
    }

    #[test]
    fn test_graph_count_paths() {
        let graph: Graph = TEST_INPUT.parse().unwrap();
        let start = graph.get_start().unwrap();

        let visited: HashSet<Rc<Node>> = HashSet::new();
        let num_paths = graph._count_paths_part_1(&start, visited);
        assert_eq!(num_paths, 10);
    }

    #[test]
    fn test_graph_count_medium() {
        let data = fs::read_to_string(TEST_INPUT_PATH).unwrap();
        let graph: Graph = data.parse().unwrap();
        let start = graph.get_start().unwrap();

        let visited: HashSet<Rc<Node>> = HashSet::new();
        let num_paths = graph._count_paths_part_1(&start, visited);
        assert_eq!(num_paths, 226);
    }

    #[test]
    fn part_1() {
        let data = fs::read_to_string(INPUT_PATH).unwrap();
        let graph: Graph = data.parse().unwrap();
        let start = graph.get_start().unwrap();

        let visited: HashSet<Rc<Node>> = HashSet::new();
        let num_paths = graph._count_paths_part_1(&start, visited);
        println!("{}", num_paths);
    }

    #[test]
    fn test_graph_count_paths_part_2() {
        let graph: Graph = TEST_INPUT.parse().unwrap();
        let start = graph.get_start().unwrap();
        let visited: HashSet<Rc<Node>> = HashSet::new();
        let num_paths = graph._count_paths_part_2(&start, visited, false);
        assert_eq!(num_paths, 36);
    }

    #[test]
    fn part_2() {
        let data = fs::read_to_string(INPUT_PATH).unwrap();
        let graph: Graph = data.parse().unwrap();

        fs::write("output/day12.dot", graph.to_dot()).unwrap();

        let start = graph.get_start().unwrap();
        let visited: HashSet<Rc<Node>> = HashSet::new();
        let num_paths = graph._count_paths_part_2(&start, visited, false);
        println!("{}", num_paths);
    }
}
