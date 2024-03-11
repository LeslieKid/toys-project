type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T> 
}

pub struct List<T> {
    head: Link<T> 
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Node {
            elem,
            next: self.head.take() 
        };
        self.head = Some(Box::new(new_node)); 
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem 
        })
    }

    pub fn peek(&self) -> Option<&T> {

        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut curr_link = self.head.take();
        while let Some(mut node) = curr_link {
            curr_link = node.next.take();
        }
    }
}

/// Create a new struct intead of impl trait for List in order to contain
/// IntoIter(), IterMut() and Iter() method.
pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basic() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);
        
        list.push(1.1);
        list.push(2.1);
        list.push(3.1);
        assert_eq!(list.pop(), Some(3.1));
        assert_eq!(list.pop(), Some(2.1));

        list.push(4.1);
        assert_eq!(list.pop(), Some(4.1));
        assert_eq!(list.pop(), Some(1.1));

        assert_eq!(list.pop(), None);
    }

    #[test]
    fn long_list_test() {
        let mut list = List::new();
        for _ in 0..1000000 {
            list.push(1);
        }
    }

    #[test]
    fn into_iter_test() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
    }

    #[test]
    fn iter_test() {
    }
}