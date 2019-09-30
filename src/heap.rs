use std::cmp;

pub struct Heap<T: PartialOrd, F> {
    nodes: Vec<T>,
    compare: F
}

impl <T: PartialOrd, F> Heap<T, F>
    where F: FnMut(&T, &T) -> cmp::Ordering {

    pub fn new(compare: F) -> Heap<T, F> {
        Heap { nodes: Vec::new(), compare }
    }

    fn sink(&mut self, index: usize) {
        if index == self.nodes.len() { return }

        let leftChildIndex = (index * 2) + 1;
        let rightChildIndex = (index * 2) + 2;
            
        let mut c_index = index;
    }

    pub fn push(&mut self, value: T) {
        self.nodes.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.nodes.pop()
    }
}
