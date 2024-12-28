use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug)]
pub struct Graph<T>
where
    T: Eq + Clone + Hash,
{
    pub edges: HashMap<T, HashSet<T>>,
}

impl<T> Graph<T>
where
    T: Eq + Clone + Hash,
{
    pub fn new() -> Self {
        Graph {
            edges: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: T) {
        self.edges.entry(vertex).or_insert_with(HashSet::new);
    }

    pub fn add_edge(&mut self, from: T, to: T) {
        self.edges
            .entry(from.clone())
            .or_insert_with(HashSet::new)
            .insert(to.clone());
        self.edges.entry(to).or_insert_with(HashSet::new);
    }

    pub fn neighbors(&self, vertex: &T) -> Option<&HashSet<T>> {
        self.edges.get(vertex)
    }
}
