use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}


impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        // `while let` == "do this until this pattern does not match"
        while let Link::More(mut box_node) = cur_link {
            cur_link = mem::replace(&mut box_node.next, Link::Empty);
            // box_node goes out of scope here and gets dropped,
            // but it's node.next has set to Empty
            // so no unbounded recursion 
        }
    }
}


// testing


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // checking empty list is created
        assert_eq!(list.pop(), None);

        // Add values to list
        list.push(1);
        list.push(2);
        list.push(3);

        // popping elements
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Adding more values
        list.push(4);
        list.push(5);

        // popping
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // end
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}