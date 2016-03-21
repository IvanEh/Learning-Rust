use std::mem;

struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    val: i32,
    next: Link,
}

impl List {
    fn new() -> Self {
        List { head: Link::Empty }
    }

    fn push(&mut self, val: i32) {
        let new_link = Link::More(Box::new(Node {
            val: val,
            next: mem::replace(&mut self.head, Link::Empty),
        }));

        self.head = new_link;

    }

    fn print(&self) {
        let mut curr_link = &self.head;
        while let Link::More(ref node) = *curr_link {
            print!("{}, ", node.val);
            curr_link = &node.next;
        }

        println!("");
    }
    
    fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty     => Option::None,
            Link::More(node)   => {
                                let val = node.val;
                                self.head = node.next;
                                Option::Some(val)
                            }
        }
    }
}

fn main() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    println! ("{:?} {:?} {:?} {:?}", list.pop(), list.pop(), list.pop(), list.pop());
}
