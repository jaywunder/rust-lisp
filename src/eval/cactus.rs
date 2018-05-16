#![allow(dead_code)]
use std::ops::Index;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Debug;
use std::mem::swap;

#[derive(Debug)]
pub struct Cactus<T: Debug> (Option<Rc<Node<T>>>);

#[derive(Debug)]
struct Node<T: Debug> {
    pub value: RefCell<T>,
    pub parent: Option<Rc<Node<T>>>,
    pub len: usize,
}

impl<T> Cactus<T> where T: Debug {
    pub fn new() -> Cactus<T> { Cactus(None) }

    pub fn is_empty(&self) -> bool { self.0.is_none() }

    pub fn len(&self) -> usize { self.0.as_ref().map_or(0, |n| n.len) }

    fn parent_ref(&self) -> Option<&Option<Rc<Node<T>>>> {
        self.0.as_ref().map(|n| &n.parent)
    }

    pub fn value(&self) -> Option<&RefCell<T>> {
        self.0.as_ref().map(|n| &n.value)
    }

    fn into_top(self) -> Option<Rc<Node<T>>> { self.0 }

    pub fn peek(&mut self) -> Option<&RefCell<T>> {
        self.0.as_ref().map(|n| &n.value)
    }

    pub fn push(&self, val: T) -> Cactus<T> {
        Cactus(Some(Rc::new(Node {
            value: RefCell::new(val),
            parent: self.0.as_ref().map(|rc| Rc::clone(rc)),
            len: self.len() + 1,
        })))
    }

    pub fn pop(&mut self) -> Option<T> {

        if self.0.is_some() {
            // take ownership of old top
            let mut old_top: Option<Rc<Node<T>>> = None;
            swap(&mut self.0, &mut old_top);

            // take ownership of old top's parent
            let mut new_top: Option<Rc<Node<T>>> = None;
            let mut old_top_rc = old_top.unwrap();
            let old_top_node = Rc::get_mut(&mut old_top_rc).unwrap();
            swap(&mut old_top_node.parent, &mut new_top);

            // make the new top the top
            swap(&mut self.0, &mut new_top);

            // return the value of old top
            Some(Rc::try_unwrap(old_top_rc).unwrap().value.into_inner())

        } else { None }
    }
}

impl<T> Clone for Cactus<T> where T: Debug {
    fn clone(&self) -> Self { Cactus(self.0.as_ref().map(|rc| Rc::clone(&rc))) }
}

impl<T> Index<usize> for Cactus<T> where T: Debug {
    type Output = RefCell<T>;

    fn index(&self, index: usize) -> &Self::Output {
        let mut current = self.0.as_ref().expect("index out of bounds when indexing cactus stack");

        if index > 0 {
            for _ in 0..index {
                current = &current.parent.as_ref().expect("index out of bounds when indexing cactus stack");
            }
        }

        &current.value
    }
}

// struct CactusIterator<T> {
//     next:
// }

// impl<T: Debug> Iterator for Cactus<T> where T: Debug {
//     type Item = Cactus<T>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let parent = self.parent();
//
//         if !parent.is_empty() {
//
//             Some(parent)
//
//         } else { None }
//     }
// }
