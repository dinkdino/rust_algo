use std::cmp;

pub struct Heap<T: PartialOrd, F> {
    nodes: Vec<T>,
    compare: F
}

impl <T: PartialOrd, F> Heap<T, F>
    where F: Fn(&T, &T) -> cmp::Ordering {

    pub fn new(compare: F) -> Heap<T, F> {
        Heap { nodes: Vec::new(), compare }
    }

    fn sink(&mut self, index: usize) {
        let len = self.nodes.len();
        if index == len { return }

        let left_child_index = (index * 2) + 1;
        let right_child_index = (index * 2) + 2;
            
        let mut c_index = index;

        if c_index < len && (self.compare)(&self.nodes[c_index], &self.nodes[left_child_index]) == cmp::Ordering::Greater {
            c_index = left_child_index;
        }

        if c_index < len && (self.compare)(&self.nodes[c_index], &self.nodes[right_child_index]) == cmp::Ordering::Greater {
            c_index = right_child_index;
        }

        if c_index == index { return }

        self.nodes.swap(c_index, index);

        self.sink(c_index);
    }

    pub fn push(&mut self, value: T) {
        self.nodes.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.nodes.pop()
    }
}
