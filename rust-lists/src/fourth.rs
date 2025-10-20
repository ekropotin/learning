use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Self {
            elem: val,
            next: None,
            prev: None,
        }
    }
}

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<T> List<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
    fn push_front(&mut self, val: T) {
        let new_head = Rc::new(RefCell::new(Node::new(val)));
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.head = Some(new_head.clone());
                self.tail = Some(new_head);
            }
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|head| Ref::map(head.borrow(), |head| &head.elem))
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

#[cfg(test)]
mod test {
    use super::List;

    fn test_front() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        assert_eq!(*list.peek_front().unwrap(), 2);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(*list.peek_front().unwrap(), 1);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }
}
