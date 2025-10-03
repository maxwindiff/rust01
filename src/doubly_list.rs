use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::{Rc, Weak};

type NodeRef<T> = Rc<RefCell<Node<T>>>;
type WeakNodeRef<T> = Weak<RefCell<Node<T>>>;

struct Node<T> {
    data: T,
    next: Option<NodeRef<T>>,
    prev: Option<WeakNodeRef<T>>,
}

pub struct LinkedList<T: Debug> {
    head: Option<NodeRef<T>>,
    tail: Option<WeakNodeRef<T>>,
}

impl<T: Debug> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None, tail: None }
    }

    pub fn push_front(&mut self, val: T) {
        let Some(old_head) = self.head.take() else {
            let node = Rc::new(RefCell::new(Node { data: val, next: None, prev: None }));
            self.tail = Some(Rc::downgrade(&node));
            self.head = Some(node);
            return;
        };
        let new_head = Rc::new(RefCell::new(Node {
            data: val,
            next: Some(old_head.clone()),
            prev: None,
        }));
        old_head.borrow_mut().prev = Some(Rc::downgrade(&new_head));
        self.head = Some(new_head);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let old_head = self.head.take()?;
        self.head = old_head.borrow_mut().next.take();
        if let Some(h) = &self.head {
            h.borrow_mut().prev = None;
        } else {
            self.tail = None;
        }
        Some(Rc::into_inner(old_head)?.into_inner().data)
    }

    pub fn push_back(&mut self, val: T) {
        let Some(old_tail) = self.tail.take().and_then(|w| w.upgrade()) else {
            let node = Rc::new(RefCell::new(Node { data: val, next: None, prev: None }));
            self.tail = Some(Rc::downgrade(&node));
            self.head = Some(node);
            return;
        };
        let new_tail = Rc::new(RefCell::new(Node {
            data: val,
            next: None,
            prev: Some(Rc::downgrade(&old_tail)),
        }));
        old_tail.borrow_mut().next = Some(new_tail.clone());
        self.tail = Some(Rc::downgrade(&new_tail));
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let old_tail = self.tail.take().and_then(|w| w.upgrade())?;
        self.tail = old_tail.borrow_mut().prev.take();
        if let Some(weak) = &self.tail {
            if let Some(t) = weak.upgrade() {
                t.borrow_mut().next = None;
            }
        } else {
            self.head = None;
        }
        Some(Rc::into_inner(old_tail)?.into_inner().data)
    }
}

impl<T: Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("data", &self.data)
            .field("prev", if self.prev.is_some() { &"Some" } else { &"X" })
            .field("next", if self.next.is_some() { &"Some" } else { &"X" })
            .finish()
    }
}
impl<T: Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_list = f.debug_list();
        let mut current = self.head.as_ref().map(|node| node.clone());
        while let Some(node) = current {
            debug_list.entry(&node.borrow());
            current = node.borrow().next.as_ref().map(|next| next.clone());
        }
        debug_list.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn test_push_back() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_mixed_push() {
        let mut list = LinkedList::new();
        list.push_front(2);
        list.push_back(3);
        list.push_front(1);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
    }

    #[test]
    fn test_pop_back() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_mixed_pop() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_empty_then_refill() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_front(), None);

        list.push_front(3);
        list.push_back(4);
        list.push_front(5);

        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_back(), None);
    }
}
