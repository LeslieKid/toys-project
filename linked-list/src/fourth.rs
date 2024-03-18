use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct Node<T> {
    elem: T,
    prev: Link<T>,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            elem,
            prev: None,
            next: None,
        }))
    }
}

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_node) => {
                old_node.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_node);
                self.head = Some(new_head);
            }
            None => {
                self.head = Some(new_head.clone());
                self.tail = Some(new_head);
            }
        }
    }

    pub fn push_back(&mut self, elem: T) {
        let new_node = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.head = None;
                    self.tail = None;
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head = None;
                    self.tail = None;
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basic() {
        let mut list = List::new();

        assert_eq!(list.pop_back(), None);
        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_back(3);

        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_front(), None);
    }
}
