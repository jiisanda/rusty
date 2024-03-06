pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }             // We don't write List<T> when we construct an instance of list
    }

    pub fn into_iter(self) -> IntoIter<T> {
        // access fields of a tuple struct iteratively
        IntoIter(self)
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

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn mut_peek(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
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

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of tuple struct numerically
        self.0.pop()
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

#[test]
fn peek() {
    let mut list  = List::new();
    assert_eq!(list.peek(), None);
    assert_eq!(list.mut_peek(), None);

    list.push(1);
    list.push(2);
    list.push(3);

    assert_eq!(list.peek(), Some(&3));
    assert_eq!(list.mut_peek(), Some(&mut 3));

    list.mut_peek().map(|value| {
        *value = 42
    });

    assert_eq!(list.peek(), Some(&42));
    assert_eq!(list.pop(), Some(42));
}

#[test]
fn into_iter() {
    let mut list = List::new();

    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
}