use nom::IResult;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt, ops,
};

pub fn star_1(data: String) {
    let graph = parse(&data);
    let count = graph.num_paths(false);
    println!("{}", count);
}

pub fn star_2(data: String) {
    let graph = parse(&data);
    let count = graph.num_paths(true);
    println!("{}", count);
}

fn parse(input: &str) -> Graph {
    super::utils::parse(graph, input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Graph<'a> {
    edges: HashMap<Node<'a>, HashSet<Node<'a>>>,
}

impl<'a> Graph<'a> {
    fn new() -> Self {
        Self::default()
    }

    fn add_edge(&mut self, start: Node<'a>, end: Node<'a>) {
        self.edges
            .entry(start)
            .or_insert_with(HashSet::new)
            .insert(end);
        self.edges
            .entry(end)
            .or_insert_with(HashSet::new)
            .insert(start);
    }

    fn num_paths(&self, can_visit_twice: bool) -> usize {
        let mut visited = HashSet::new();
        self.num_paths_from(Node::Start, &mut visited, can_visit_twice, None)
    }

    fn num_paths_from(
        &self,
        node: Node<'a>,
        visited: &mut HashSet<Node<'a>>,
        can_visit_twice: bool,
        visited_twice: Option<Node<'a>>,
    ) -> usize {
        let mut num_paths = 0;

        for next in &self[node] {
            match next {
                Node::Start => {}
                Node::End => {
                    num_paths += 1;
                }
                Node::Big(_) => {
                    num_paths +=
                        self.num_paths_from(*next, visited, can_visit_twice, visited_twice);
                }
                Node::Small(_) => {
                    let visited_twice = if visited.contains(next) {
                        if !can_visit_twice || visited_twice.is_some() {
                            continue;
                        }
                        Some(*next)
                    } else {
                        visited_twice
                    };

                    visited.insert(*next);
                    num_paths +=
                        self.num_paths_from(*next, visited, can_visit_twice, visited_twice);

                    match visited_twice {
                        Some(n) if n == *next => {}
                        _ => {
                            visited.remove(next);
                        }
                    }
                }
            }
        }

        num_paths
    }
}

impl<'a> Default for Graph<'a> {
    fn default() -> Self {
        Self {
            edges: HashMap::default(),
        }
    }
}

impl<'a> ops::Index<Node<'a>> for Graph<'a> {
    type Output = HashSet<Node<'a>>;

    fn index(&self, node: Node<'a>) -> &Self::Output {
        self.edges.index(&node)
    }
}

fn graph(input: &str) -> IResult<&str, Graph> {
    use nom::{
        character::complete::line_ending, combinator::opt, multi::fold_many0, sequence::terminated,
    };

    fold_many0(
        terminated(edge, opt(line_ending)),
        Graph::new,
        |mut map, (start, end)| {
            map.add_edge(start, end);
            map
        },
    )(input)
}

fn edge(input: &str) -> IResult<&str, (Node, Node)> {
    use nom::{character::complete::char as char_, sequence::separated_pair};

    separated_pair(node, char_('-'), node)(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Node<'a> {
    Start,
    End,
    Big(&'a str),
    Small(&'a str),
}

impl<'a> From<&'a str> for Node<'a> {
    fn from(name: &'a str) -> Self {
        if name == "start" {
            Self::Start
        } else if name == "end" {
            Self::End
        } else if name.chars().all(char::is_uppercase) {
            Self::Big(name)
        } else {
            Self::Small(name)
        }
    }
}

impl<'a> fmt::Display for Node<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Start => write!(f, "start"),
            Self::End => write!(f, "end"),
            Self::Big(name) | Self::Small(name) => write!(f, "{}", name),
        }
    }
}

fn node(input: &str) -> IResult<&str, Node> {
    use nom::{character::complete::alpha1, combinator::map};

    map(alpha1, Node::from)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_node_is_correct() {
        let name = "start";
        assert_eq!(Node::Start, Node::from(name));
    }

    #[test]
    fn end_node_is_correct() {
        let name = "end";
        assert_eq!(Node::End, Node::from(name));
    }

    #[test]
    fn big_node_is_correct() {
        let name = "ABC";
        assert_eq!(Node::Big("ABC"), Node::from(name));
    }

    #[test]
    fn small_node_is_correct() {
        let name = "star";
        assert_eq!(Node::Small("star"), Node::from(name));
    }
}
