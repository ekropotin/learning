use std::{
    cell::{Ref, RefCell},
    ops::Deref,
    rc::Rc,
};

type RcNode<T> = Rc<RefCell<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Option<RcNode<T>>,
    prev: Option<RcNode<T>>,
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
    head: Option<RcNode<T>>,
    tail: Option<RcNode<T>>,
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
}

#[cfg(test)]
mod test {}
