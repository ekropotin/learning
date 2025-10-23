struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct List<'a, T> {
    head: Link<T>,
    // tail: Link<T>,
    tail: Option<&'a mut Node<T>>,
}

impl<'a, T> List<'a, T> {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    fn push(&mut self, val: T) {
        let new_tail = Box::new(Node {
            elem: val,
            next: None,
        });
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.next = Some(new_tail);
                self.tail = Some(&mut new_tail);
            }
            None => {
                self.head = Some(new_tail);
                self.tail = Some(&mut new_tail);
            }
        }
    }
}
