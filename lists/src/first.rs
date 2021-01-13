// Learning linked lists:
// Linked lists are a "sum type": A type that can have different values that may be different
// types. (enums in rust!)
// We need to define what a node is. We can do that with a struct. Every node contains an element (we
// will just say that element is a 32bit integer) and points to another List object. We will define
// a list as an enum which can take on Empty element, or another node.
//
// struct Node {
//    elem: i32,
//    next: List,
//}

//pub enum List {
//    Empty,
//    More(Box<Node>),
//}
//I was lied to!!! We need to define what a link is. THe above throws Error[E0446]: private type `first::Node` in public interface
//because Node is a private struct, while List is a public enum because we should be able to
//publicly use a linked list. We are not allowed to publicly use private types
//
use std::mem; // This let's us steal a value out of a borrow by replacing it (mem::replace).
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


// The above will not compile because we never use/call any of the elements. We need to implement
// some code to do this. It is like defining methods for objects

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
        // This method instantiates a new List with an empty link
        // We refer to variants of an enum using ::, which is the namespacing operator
        // Self is an alias for the type 'List'
        // In Rust, The last expression of a function is implicitly returned. This makes simple functions a little neater. You can still use return to return early like other C-like languages.
    // We want a new function that will let us push an element to the list
    
    pub fn push(&mut self, elem: i32) {
        //We push nodes to lists so let's instantiate a new node to push:
        let new_node = Box::new(Node {
            elem: elem,
            //next: self.head, ^^^^^^^^^ move occurs because `self.head` has type `first::Link`, which does not implement the `Copy` trait. AKA: We are trying to move self.head to next without borrowing (we are stealing)
            next: mem::replace(&mut self.head, Link::Empty) //We are now saying that self.head is a mutable reference; &mut and we will replace it with an Empty link.
        });
        self.head = Link::More(new_node);
        // The replacement in line 53 was temporary because here we redefine self.head as a new node
    }

    // To test `push`, we need a `pop` function:
    pub fn pop(&mut self) -> Option<i32> {
        //Option takes care of the case when we try to `pop` an Empty node from the list (because
        //we can't pop anything from an empty list).
        let result;
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => {
                // We don't want to return anything if the link is empty:
                result = None;
            }
            // We want to return the element that the node contains:
            Link::More(node) => { //by the by ... 
                result = Some(node.elem);
                self.head = node.next;
            }
        };
        result
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

// Write some tests. (Tests are generally written next to the code they support, but within a new
// namespace).
#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        // We want to create a new list:
        let mut list = List::new();

        // Test empty list behavior:
        assert_eq!(list.pop(), None);

        // We want to use push functionality and test the pop functionality on non None values:
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push a few more things to ensure list is not corrupted:
        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check list exhaustion:
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
