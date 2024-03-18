use std::mem;

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

struct List<'a, T> {
    head: Link<T>,
    tail: Link<&'a T>,
}

impl<'a, T> List<'a, T> {
    pub fn new() -> List<'a, T> {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push(&mut self, elem: T) {
        let new_tail = Box::new(&Node {
            elem,
            next: None,
        });

        let old_tail = mem::replace(&mut self.tail, Some(new_tail));
        match old_tail {
            Some(mut old_tail) => {
                old_tail.next = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail);
            }
        }
    }
}