pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }             // We dont write List<T> when we construct an instance of list 
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new( Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        // match self.head.take() {
        //     None => None,
        //     Some(node) => {
        //         self.head = node.next;
        //         Some(node.elem)
        //     }
        // }
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })                                  // using closures:
        // Closures are anonymous functions with extra superpower
        // they can refer to local variables outside the closure
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        while let Some(mut box_node) = cur_link {
            cur_link = box_node.next.take();
        }
    }
}


// here self.head.take() is
// mem::replace(&mut self.head, None)


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