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

    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        let current = self.head.clone();
        Cursor {
            list: self,
            current: current,
        }
    }
}

pub struct Cursor<'a, T: Debug> {
    list: &'a mut LinkedList<T>,
    current: Option<NodeRef<T>>,
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T: Debug> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.current.as_deref().and_then(|cell| {
            // This is safe because we have exclusive access to the list
            // through the mutable reference in the cursor.
            unsafe { cell.as_ptr().as_mut() }
        }).map(|node| &mut node.data)
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    pub fn next(&mut self) -> Option<&mut T> {
        self.current = self.current.as_ref().and_then(|node| node.borrow().next.clone());
        self.peek_mut()
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        todo!()
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        todo!()
    }

    pub fn insert_after(&mut self, _element: T) {
        todo!()
    }

    pub fn insert_before(&mut self, _element: T) {
        todo!()
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

    #[test]
    fn test_cursor_peek_mut() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let mut cursor = list.cursor_front();
        assert_eq!(cursor.peek_mut(), Some(&mut 1));

        if let Some(val) = cursor.peek_mut() {
            *val = 10;
        }
        assert_eq!(cursor.peek_mut(), Some(&mut 10));
    }

    #[test]
    fn test_cursor_peek_mut_empty() {
        let mut list: LinkedList<i32> = LinkedList::new();
        let mut cursor = list.cursor_front();
        assert_eq!(cursor.peek_mut(), None);
    }

    #[test]
    fn test_cursor_next() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let mut cursor = list.cursor_front();
        assert_eq!(cursor.peek_mut(), Some(&mut 1));
        assert_eq!(cursor.next(), Some(&mut 2));
        assert_eq!(cursor.next(), Some(&mut 3));
        assert_eq!(cursor.next(), None);
    }

    #[test]
    fn test_cursor_next_empty() {
        let mut list: LinkedList<i32> = LinkedList::new();
        let mut cursor = list.cursor_front();
        assert_eq!(cursor.next(), None);
    }

    #[test]
    fn test_cursor_next_mutate() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let mut cursor = list.cursor_front();
        cursor.next();
        if let Some(val) = cursor.peek_mut() {
            *val = 20;
        }

        drop(cursor);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.pop_front(), Some(3));
    }

    #[test]
    fn test_cursor_peek_mut_and_next_combined() {
        let mut list = LinkedList::new();
        list.push_back(10);
        list.push_back(20);
        list.push_back(30);

        let mut cursor = list.cursor_front();

        if let Some(val) = cursor.peek_mut() {
            *val += 5;
        }
        assert_eq!(cursor.peek_mut(), Some(&mut 15));

        if let Some(val) = cursor.next() {
            *val += 5;
        }
        assert_eq!(cursor.peek_mut(), Some(&mut 25));

        if let Some(val) = cursor.next() {
            *val += 5;
        }
        assert_eq!(cursor.peek_mut(), Some(&mut 35));

        assert_eq!(cursor.next(), None);
    }
}
