use std::fmt::Debug;
use std::ops::{DerefMut, Deref};
use std::borrow::BorrowMut;

enum LinkedList<T> {
    Nil,
    Cons(T, Box<LinkedList<T>>),
}

// struct LinkedList<T> {
//     data: T,
//     next: Box<LinkedList<T>>,
// }

impl<T: Debug> LinkedList<T> {
    fn push_front(mut self, data: T) -> Self {
        match self {
            LinkedList::Nil => LinkedList::Cons(data, Box::new(LinkedList::Nil)),
            LinkedList::Cons(_, _) => LinkedList::Cons(data, Box::new(self))
        }
    }

    fn print(&self) {
        let mut head = self;
        loop {
            match head {
                LinkedList::Nil => {
                    println!("NIL");
                    break;
                }
                LinkedList::Cons(data, next) => {
                    println!("{:?}", data);
                    head = next;
                }
            }
        }
    }
}

#[derive(Debug)]
struct LL<T> {
    head: Option<(T, Box<LL<T>>)>,
}

impl<T: Debug> LL<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn push_front(&mut self, data: T) {
        self.head = Some((data, Box::new(LL { head: self.head.take() })));
    }

    fn push_back(&mut self, data: T) {
        let mut node = self;
        while let Some((_, ref mut tail)) = node.head {
            node = tail;
        }
        node.push_front(data);
    }

    fn print(&self) {
        let mut h = &self.head;
        while let Some((value, tail)) = h {
            println!("{:?}", value);
            h = &tail.head;
        }
    }
}

#[derive(Debug)]
struct M {
    a: i32,
    b: N,
}

#[derive(Debug)]
struct N {
    i: i32,
    j: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ll_push() {

        // let mut x = N { i:10, j: 20};
        // let mut y = M { a:30, b: x};
        // //
        // // // let (a, b) = &x;
        // let M { ref a, b } = y;
        //
        // // *a = 10;
        // println!("{:?}", y);
        //
        // let (c, d) = &mut x;
        // *c = 20;
        // println!("{:?}", x);
        //
        // let y = &x;
        // let e = y.0;
        // let (e, f) = y;


        //
        //
        // let c = 'Q';
        // let mut ref_c1 = c;
        // let ref ref_c1 = c;
        // let ref_c2 = &c;
        // let ref_c3 = &mut c;
        //
        let mut l = LL::new();
        l.push_back(1);
        l.push_back(2);
        l.push_back(3);
        l.print();
    }

    #[test]
    fn test_take() {
        let mut x = 10;

        let mut pair: Option<(i32, i32)> = None;
        let t = pair.take();
        pair = Some((2, 3));

        println!("{:?}, {:?}", t, pair);

        let t = pair.take();
        pair = Some((3, 4));

        println!("{:?}, {:?}", t, pair);
    }

    #[test]
    fn test_linked_list() {
        let mut lst = LinkedList::Nil;
        lst = lst.push_front(10);
        lst = lst.push_front(20);
        lst = lst.push_front(30);
        lst.print();
    }
}