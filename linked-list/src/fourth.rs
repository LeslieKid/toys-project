use std::rc::Rc;
use std::cell::RefCell;

type Link<T> = Option<Rc<RefCell<T>>>;

pub struct Node<T> {
    elem: T,
    prev: Link<T>,
    next: Link<T>
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Node<T> {
        Node { elem, prev: None, next: None } 
    }
}

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None, tail: None}
    }

    pub fn push(&mut self, elem: T) -> List<T> {
        
    }
}
