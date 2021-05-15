use std::rc::Rc;

use std::fmt::Debug;
use std::sync::RwLock;

type Elem<T> = Option<Rc<RwLock<Node<T>>>>;

struct Node<T>
where
    T: Debug,
{
    val: T,
    next: Elem<T>,
}

impl<T: Debug> Node<T> {
    fn new(v: T) -> Self {
        Node { val: v, next: None }
    }
}

pub struct LList<T>
where
    T: Debug,
{
    head: Elem<T>,
    tail: Elem<T>,
    count: usize,
}

impl<T: Debug> Default for LList<T> {
    fn default() -> LList<T> {
        LList::new()
    }
}

impl<T: Debug> LList<T> {
    pub fn new() -> Self {
        LList {
            head: None,
            tail: None,
            count: 0,
        }
    }

    pub fn push(&mut self, val: T) {
        if self.count == 0 {
            self.head = Some(Rc::new(RwLock::new(Node::new(val))));
            self.head.as_ref().unwrap().write().unwrap().next = self.head.clone();
            self.tail = self.head.clone(); // this is suboptimal
        } else {
            let nl = Some(Rc::new(RwLock::new(Node::new(val))));
            self.tail.as_ref().unwrap().write().unwrap().next = nl.clone();
            self.tail = nl;
        }
        self.count += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.count {
            0 => None,
            1 => {
                self.count = 0;
                self.head = None;
                // make sure tail doesn't point to itself. this happens when
                // there's just one element in the list and head==tail
                // and head->next==tail
                self.tail.as_ref().unwrap().write().unwrap().next = None;
                Rc::try_unwrap(self.tail.take().unwrap())
                    .ok()
                    .map(|v| v.into_inner().unwrap().val)
            }
            _ => {
                let mut it = self.head.clone();
                for _ in 2..self.count {
                    it = it.unwrap().read().unwrap().next.clone();
                }
                it.as_ref().unwrap().write().unwrap().next = None;
                let rez = Rc::try_unwrap(self.tail.take().unwrap())
                    .ok()
                    .map(|v| v.into_inner().unwrap().val);
                self.tail = it;
                self.count -= 1;
                rez
            }
        }
    }

    fn rev(prev: Elem<T>, curr: Elem<T>) {
        if curr.is_some() {
            let n = curr.as_ref().unwrap().read().unwrap().next.clone();
            curr.as_ref().unwrap().write().unwrap().next = prev;
            LList::rev(curr, n)
        }
    }

    pub fn reverse(&mut self) {
        LList::rev(None, self.head.clone());

        let ot = self.tail.take();
        self.tail = self.head.take();
        self.head = ot;
    }
}
