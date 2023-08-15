mod test;

use std::collections::HashMap;

use anyhow::Context;
use keyed_vec::{IndexLike, KeyedVec};

#[derive(Copy, Debug, Clone)]
struct InternalVectorIndex {
    i: usize,
}

impl IndexLike for InternalVectorIndex {
    #[inline]
    fn to_index(&self) -> usize {
        self.i
    }

    #[inline]
    fn from_index(i: usize) -> Self {
        InternalVectorIndex { i }
    }
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImageVectorIndex {
    i: usize,
}

impl IndexLike for ImageVectorIndex {
    #[inline]
    fn to_index(&self) -> usize {
        self.i
    }

    #[inline]
    fn from_index(i: usize) -> Self {
        ImageVectorIndex { i }
    }
}

#[derive(Debug, Clone)]
pub struct Graph {
    base_vector: KeyedVec<InternalVectorIndex, Vertex>,
    value_to_index: HashMap<ImageVectorIndex, InternalVectorIndex>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            base_vector: KeyedVec::new(),
            value_to_index: HashMap::new(),
        }
    }

    fn add_node(&mut self, parent_value: Option<ImageVectorIndex>, value: ImageVectorIndex) -> InternalVectorIndex {
        let internal_index_of_new_value = self.value_to_index.get(&value);

        match internal_index_of_new_value {
            Some(&i) => {
                if let Some(parent_value) = parent_value {
                    let parents = &mut self.base_vector.get_mut(i).unwrap().parents;
                    if !parents.contains(&parent_value) {
                        parents.push(parent_value);
                    }
                }
                i
            },
            None => {
                // if the value doesn't already exists, we add it
                let new_node = Vertex::new(parent_value, value);
                let new_node_index = self.base_vector.push(new_node);
                self.value_to_index.insert(value, new_node_index);
                new_node_index
            }
        }
    }

    fn add_internal(&mut self, parent_value: Option<ImageVectorIndex>, value: ImageVectorIndex) -> Result<(), anyhow::Error> {
        if let Some(parent_value) = parent_value {
            self.add_node(None, parent_value);
        }
        self.add_node(parent_value, value);

        // If there is a parent, we try to find it to update its children
        if let Some(parent_value) = parent_value {
            let internal_index_of_parent_value = *self.value_to_index.get(&parent_value).context("could not find parent_value in value_to_index")?;
            let children = &mut self.base_vector.get_mut(internal_index_of_parent_value).unwrap().children;
            if !children.contains(&value) {
                children.push(value);
            }
        }
        
        Ok(())
    }

    pub fn add(&mut self, parent_value: Option<usize>, value: usize) -> Result<(), anyhow::Error> {
        self.add_internal(parent_value.map(|i| ImageVectorIndex { i }), ImageVectorIndex { i: value })
    }

    pub fn get(&self, value: ImageVectorIndex) -> Result<&Vertex, anyhow::Error> {
        let &index = self.value_to_index.get(&value).context("Could not find value in internal map")?;
        Ok(self.base_vector.get(index).context("Could not find index in base_vector")?)
    }
}

#[derive(Debug, Clone)]
pub struct Vertex {
    parents: Vec<ImageVectorIndex>,
    value: ImageVectorIndex,
    children: Vec<ImageVectorIndex>,
}

impl Vertex {
    fn new(parent: Option<ImageVectorIndex>, value: ImageVectorIndex) -> Vertex {
        Vertex {
            parents: parent.into_iter().collect(),
            value,
            children: vec!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ItemView<'a, T> {
    graph_view: &'a GraphView<T>,
    pub vertex_value: ImageVectorIndex,
}

impl <'a, T> ItemView<'a, T> {
    fn new(graph_view: &'a GraphView<T>, value: ImageVectorIndex) -> ItemView<T> {
        ItemView {
            graph_view,
            vertex_value: value,
        }
    }

    pub fn translate(&self) -> &'a T {
        self.graph_view.backing_vec.get(self.vertex_value).unwrap()
    }
}

pub struct VertexView<'a, T> {
    graph_view: &'a GraphView<T>,
    pub value: ItemView<'a, T>,
}

impl <'a, T> VertexView<'a, T> {
    fn new(graph_view: &'a GraphView<T>, value: ImageVectorIndex) -> VertexView<'a, T> {
        VertexView {
            graph_view,
            value: ItemView::new(&graph_view, value),
        }
    }

    pub fn parents(&'a self) -> Box<dyn Iterator<Item = VertexView<'a, T>> + 'a>{
        let item = self.graph_view.graph.get(self.value.vertex_value).unwrap();
        Box::new(item.parents.iter().map(|&p| VertexView::new(&self.graph_view, p)))
    }

    pub fn children(&'a self) -> Box<dyn Iterator<Item = VertexView<'a, T>> + 'a>{
        let item = self.graph_view.graph.get(self.value.vertex_value).unwrap();
        Box::new(item.children.iter().map(|&p| VertexView::new(&self.graph_view, p)))
    }
}

#[derive(Debug, Clone)]
pub struct GraphView<T> {
    graph: Graph,
    backing_vec: KeyedVec<ImageVectorIndex, T>,
}

impl <T: std::fmt::Debug + Clone> GraphView<T> {
    pub fn new(graph: Graph, content: &Vec<T>) -> GraphView<T> {
        let mut kv = KeyedVec::<ImageVectorIndex, T>::new();
        content.iter().for_each(|e| { kv.push(e.clone()); });
        GraphView {
            graph,
            backing_vec: kv,
        }
    }

    pub fn print(&self) {
        for (node_index, node) in self.graph.base_vector.enumerate() {
            println!("{:?}:{:?}:{:?}", node_index, node.value, self.backing_vec.get(node.value).unwrap());
            for child_index in &node.children {
                let b = self.graph.value_to_index.get(&child_index).unwrap();
                let i = self.graph.base_vector.get(*b).unwrap_or_else(|| panic!("Failed to access child node {:?}: {:?} ({:?})", child_index, self.backing_vec, node.value.i));
                println!("    {}:{:?}", i.value.i, self.backing_vec.get(i.value).unwrap());
            }
        }
    }

    fn get_internal(&self, value: ImageVectorIndex) -> Result<VertexView<T>, anyhow::Error> {
        Ok(VertexView::new(&self, value))
    }

    pub fn get(&self, value: usize) -> Result<VertexView<T>, anyhow::Error> {
        self.get_internal(ImageVectorIndex { i: value })
    }

    pub fn get_by_internal_key(&self, ImageVectorIndex { i }: ImageVectorIndex) -> Result<VertexView<T>, anyhow::Error> {
        self.get(i)
    }

    // TODO actually make it an iterator
    pub fn iter(&self) -> impl Iterator<Item = (ImageVectorIndex, &T)> {
        self.backing_vec.enumerate()
    }
}
