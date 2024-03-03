# Linked list in Rust

## Stack

To set the layout of Linked list we could have used enums likewise...

```rust
pub enum List {
    Empty,
    Elem(i32, Box<List>),
}
```

... but this is not good for multiple reasons

consider list with two elements...

```
[] = Stack
() = Heap

[Elem A, ptr] -> (Elem B, ptr) -> (Empty, *junk)
```
there are two issues: 
1) We're allocating the node that says "I'm not actually a Node."
2) One of our node isn't heap-allocated at all..

> `Box<T>`, provides the simplest form of heap allocation in Rust. Boxes provides ownership for this allocation, and drop 
> their contents when they go out of scope.

Consider the following layout for our list:

```
[ptr] -> (Elem A, ptr) -> (Elem B, *null)
```

In this layout we conditionally heap allocates our nodes. Key difference is absence of junk from first. To understand junk
lets see how enums work...

```rust
enum Foo {
    D1(T1),
    D2(T2),
    D3(T3),
    ...,
    D4(T4)
}
```

Foo needs to store some integer to indicate which variant of the enum it represents (D1, D2, ..., Dn). This is tag in enum. 
It will also need enough space to store the largest of T1, T2, .., Tn (+ some extra space to satisfy alignment requirements)
More on enums : https://www.youtube.com/watch?v=Epwlk4B90vk&pp=ygUWaG93IGVudW1zIHdvcmsgaW4gcnVzdA%3D%3D

The main part is that even though Empty is a single bit of information, it necessarily consumes space for pointer or an element,
because it has be ready to become an Elem at any time. So the first layout heap allocates an extra  element that just a junk,
consuming more space as compared to 2nd layout.

While enums let us declare a type that can contain one of several values, structs let us declare a type that contains may values at once.
So breaking our list into 2 parts: A List ana a Node

```rust
struct Node {
    elem: i32,
    next: List,
}

pub enum List {
    Empty,
    More(Box<Node>),
}
```

So our requirements were:
- Tail of list never allocates extra junk: ✔️!
- emum is in null-pointer-optimized form: ✔️!
- All elements are uniformly allocated: ✔️!

But when we build it there is an issue. We made List public because we want people to be able to use it, but not the node.
The problem is that the internal of the node is totally public, and we are not allowed to publicly talk about private items.
`More(Box<Node>)`. We could make all Node totally public, but generally in Rust it is favoured keeping implementation details 
private. So,

```rust
pub struct List {
    head: Link
}

enum Link {
    Empty,
    Mpre(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}
```

As List is a struct with a single field, its size is same as that field. And hence zero-cost abstraction! 

### Push

```rust
pub fn push(&mut self, elem: i32) {
    let new_node = Node {
        elem: elem,
        next: self.head,
    }
}
```

But this will give out error `cannot move out of borrowed content`, it means we are trying to move the self.head to next,
but Rust doesn't want us doing that. So we use `mem::replace`.

```rust
use std::mem;

pub fn push(&mut self, elem: i32) {
    let new_node = Box::new(Node {
        elem: elem,
        next: mem::replace(&mut self.head, Link::Empty),
    });
    
    self.head = Link::More(new_node);
}
```

Here we replace self.head temporally with Link::Empty before replacing it with the new head of the list.

### Pop

Pop function returns `Option<T>` which is an enum, it can be `Some(T)` or `None`. As pop has to deal with corner cases: 
what if the list is empty? The pointy bits on `Option<T>` indicates that Option is generic over T, which means that ypu can make 
an Option of any type. So using `match` we are checking weather `self.head` is `Empty` or has `More`.

```rust
pub fn pop(&mut self) -> Option<i32> {
    let result;
    match &self.head {
        Link::Empty => {
            result = None
        }
        Link::More(node) => {
            result = Some(node.elem);
            self.head = node.next;
        }
    };
    result
    // unimplemented!()        // macro that panics the program when we get to it - (~crashes it in a controlled manner)
}
```

code makes sense right? but there is an issue... it says ```cannot move out of borrowed content``` for ```self.head = node.next```.

We are trying to move out of `node` when all we have is shared reference to it. Before moving forward lets summarize what we want...
- Check if the list is empty
- If it's empty, return None
- If it's not empty
  - remove the head of the list
  - remove its elem
  - replace the list's head with its next
  - return Some(elem)

The key insight is we want to remove things, which means we want to get the head of the list bu value. We cant do that 
via the shared reference we get through &self.head. As we only have the shared reference to self, so the only way we can 
move stuff is to replace it. So we have,

```rust
pub fn pop(&mut self) -> Option<i32> {
    let result;
    match mem::replace(&mut self.head, Link::Empty) {
        Link::Empty => {
            result = None
        }
        Link::More(node) => {
            result = Some(node.elem);
            self.head = node.next;
        }
    };
    result
}
```