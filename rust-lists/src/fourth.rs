use std::{
    cell::{Ref, RefCell},
    ops::Deref,
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

// Intro iter
struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

//Iter
struct Iter<T> {
    next: Link<T>,
}

impl<'a, T: 'a> Iterator for Iter<T> {
    type Item = Ref<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|next_rc| {
            let borrow = next_rc.borrow(); // Ref<Node<T>>
            self.next = borrow.next.clone(); // advance iterator
            Ref::map(borrow, |node| &node.elem)
        })
    }
}
impl<T> List<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
    fn push_back(&mut self, val: T) {
        let new_tail = Rc::new(RefCell::new(Node::new(val)));
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|tail| Ref::map(tail.borrow(), |tail_ref| &tail_ref.elem))
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

    #[test]
    fn test_basic() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());

        list.push_front(1);
        assert_eq!(*list.peek_front().unwrap(), *list.peek_back().unwrap());

        list.push_front(2);
        assert_eq!(*list.peek_front().unwrap(), 2);
        assert_eq!(*list.peek_back().unwrap(), 1);

        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(*list.peek_front().unwrap(), 1);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());

        list.push_back(1);
        list.push_back(2);
        assert_eq!(*list.peek_back().unwrap(), 2);
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(*list.peek_back().unwrap(), 1);
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
        assert!(list.peek_back().is_none());
    }

    #[test]
    fn test_iterator() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(4);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);

        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(4);

        let mut iter = list.into_iter();
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next_back(), Some(2));
        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.next_back(), Some(4));
        assert_eq!(iter.next_back(), None);
    }
}
