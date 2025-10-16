use std::rc::Rc;

struct Node<T> {
    elem: T,
    next: Option<Rc<Node<T>>>,
}

pub struct List<T> {
    head: Option<Rc<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: Option::None }
    }

    pub fn prepend(&self, val: T) -> List<T> {
        List {
            head: Option::Some(Rc::new(Node {
                elem: val,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|n| n.next.clone()),
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut curr = self.head.take();
        while let Some(boxed_node) = curr {
            match Rc::try_unwrap(boxed_node) {
                Ok(mut node) => curr = node.next.take(),
                Err(_) => {
                    break;
                }
            }
        }
    }
}
// Iter
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn iter() {
        let mut list = List::<i32>::new();
        list = list.prepend(1);
        list = list.prepend(2);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn basics() {
        let mut list = List::<i32>::new();
        list = list.prepend(1);
        list = list.prepend(2);
        assert_eq!(list.peek(), Some(&2));
        list = list.tail();
        assert_eq!(list.peek(), Some(&1));
        list = list.tail();
        assert_eq!(list.peek(), None);
        list = list.tail();
        assert_eq!(list.peek(), None);
    }
}
