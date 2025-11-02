use std::ptr::null_mut;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = *mut Node<T>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

// IntoIter
pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// Iter
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = unsafe { node.next.as_ref() };
            &node.elem
        })
    }
}

// IterMut
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = unsafe { node.next.as_mut() };
            &mut node.elem
        })
    }
}
impl<'a, T> List<T> {
    pub fn new() -> Self {
        List {
            head: std::ptr::null_mut(),
            tail: std::ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        let new_tail = Box::into_raw(Box::new(Node {
            elem,
            next: null_mut(),
        }));
        if self.tail.is_null() {
            self.head = new_tail;
        } else {
            unsafe {
                (*self.tail).next = new_tail;
            }
        }
        self.tail = new_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_null() {
            return None;
        }
        let old_head = unsafe { Box::from_raw(self.head) };
        self.head = old_head.next;
        if self.head.is_null() {
            self.tail = null_mut();
        }
        Some(old_head.elem)
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.elem) }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.elem) }
    }
    pub fn iter(&'a self) -> Iter<'a, T> {
        Iter {
            next: unsafe { self.head.as_ref() },
        }
    }

    pub fn iter_mut(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            next: unsafe { self.head.as_mut() },
        }
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}
#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        list.push(1);
        assert_eq!(list.peek(), Some(&1));
        let head_elem = list.peek_mut().unwrap();
        *head_elem = 5;
        assert_eq!(list.peek(), Some(&5));
    }

    #[test]
    fn iters() {
        //IntoIter
        let mut list = List::new();
        list.push(1);
        list.push(2);
        let mut into_iter = list.into_iter();
        assert_eq!(into_iter.next(), Some(1));
        assert_eq!(into_iter.next(), Some(2));
        assert_eq!(into_iter.next(), None);

        //Iter
        let mut list = List::new();
        list.push(1);
        list.push(2);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);

        //IterMut
        let mut list = List::new();
        list.push(1);
        list.push(2);
        let mut iter_mut = list.iter_mut();
        let first = iter_mut.next();
        assert_eq!(first, Some(&mut 1));
        *first.unwrap() = 100;
        assert_eq!(iter_mut.next(), Some(&mut 2));
        assert_eq!(iter_mut.next(), None);
        assert_eq!(list.peek(), Some(&100));
    }
}
