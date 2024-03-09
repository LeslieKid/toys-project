use std::mem;

pub struct List {
    head: Link 
}

impl List {
    pub fn new() -> List {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem,
            next: Box::new(mem::replace(&mut self.head, Link::Empty))
        };
        self.head = Link::Element(new_node); 
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None, 
            Link::Element(node) => {
                let elem = node.elem; 
                self.head = *node.next; 
                Some(elem)
            }
        }
    }
}

enum Link {
    Empty,
    Element(Node)
}

struct Node {
    elem: i32,
    next: Box<Link>
}
