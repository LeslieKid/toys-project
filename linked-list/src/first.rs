use std::mem;

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> List {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        };
        self.head = Link::Element(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::Element(node) => {
                let elem = node.elem;
                self.head = node.next;
                Some(elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut curr_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::Element(mut node) = curr_link {
            curr_link = mem::replace(&mut node.next, Link::Empty);
        }
    }
}

enum Link {
    Empty,
    Element(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basic() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));

        assert_eq!(list.pop(), None);
    }

    #[test]
    fn long_list() {
        let mut list = List::new();
        for _ in 0..1000000 {
            list.push(1);
        }
    }
}
