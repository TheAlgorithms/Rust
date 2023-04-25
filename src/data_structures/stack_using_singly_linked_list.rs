// The public struct can hide the implementation detail
pub struct Stack<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Stack<T> {
    // Self is an alias for Stack
    // We implement associated function name new for single-linked-list
    pub fn new() -> Self {
        // for new function we need to return a new instance
        Self {
            // we refer to variants of an enum using :: the namespacing operator
            head: None,
        } // we need to return the variant, so there without the ;
    }

    // Here are the primary forms that self can take are: self, &mut self and &self.
    // Since push will modify the linked list, we need a mutable reference `&mut`.
    // The push method which the signature's first parameter is self
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        // don't forget replace the head with new node for stack
        self.head = Some(new_node);
    }

    /// The pop function removes the head and returns its value.
    ///
    /// To do so, we'll need to match the `head` of the list, which is of enum type `Option<T>`.\
    /// It has two variants: `Some(T)` and `None`.
    /// * `None` - the list is empty:
    ///   * return an enum `Result` of variant `Err()`, as there is nothing to pop.
    /// * `Some(node)` - the list is not empty:
    ///   * remove the head of the list,
    ///   * relink the list's head `head` to its following node `next`,
    ///   * return `Ok(elem)`.
    pub fn pop(&mut self) -> Result<T, &str> {
        match self.head.take() {
            None => Err("Stack is empty"),
            Some(node) => {
                self.head = node.next;
                Ok(node.elem)
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        // Returns true if head is of variant `None`.
        self.head.is_none()
    }

    pub fn peek(&self) -> Option<&T> {
        // Converts from &Option<T> to Option<&T>.
        match self.head.as_ref() {
            None => None,
            Some(node) => Some(&node.elem),
        }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        match self.head.as_mut() {
            None => None,
            Some(node) => Some(&mut node.elem),
        }
    }

    pub fn into_iter_for_stack(self) -> IntoIter<T> {
        IntoIter(self)
    }
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
    // '_ is the "explicitly elided lifetime" syntax of Rust
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// The drop method of singly linked list.
///
/// Here's a question: *Do we need to worry about cleaning up our list?*\
/// With the help of the ownership mechanism, the type `List` will be cleaned up automatically (dropped) after it goes out of scope.\
/// The Rust Compiler does so automacally. In other words, the `Drop` trait is implemented automatically.\
///
/// The `Drop` trait is implemented for our type `List` with the following order: `List->Link->Box<Node>->Node`.\
/// The `.drop()` method is tail recursive and will clean the element one by one, this recursion will stop at `Box<Node>`\
/// <https://rust-unofficial.github.io/too-many-lists/first-drop.html>
///
/// We wouldn't be able to drop the contents contained by the box after deallocating, so we need to manually write the iterative drop.
impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to None
            // so no unbound recursion occurs.
        }
    }
}

// Rust has nothing like a yield statement, and there are actually 3 different iterator traits to be implemented

// Collections are iterated in Rust using the Iterator trait, we define a struct implement Iterator
pub struct IntoIter<T>(Stack<T>);

impl<T> Iterator for IntoIter<T> {
    // This is declaring that every implementation of iterator has an associated type called Item
    type Item = T;
    // the reason iterator yield Option<self::Item> is because the interface coalesces the `has_next` and `get_next` concepts
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop().ok()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            // as_deref: Converts from Option<T> (or &Option<T>) to Option<&T::Target>.
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        // we add take() here due to &mut self isn't Copy(& and Option<&> is Copy)
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test_stack {

    use super::*;

    #[test]
    fn basics() {
        let mut list = Stack::new();
        assert_eq!(list.pop(), Err("Stack is empty"));

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Ok(3));
        assert_eq!(list.pop(), Ok(2));

        list.push(4);
        list.push(5);

        assert!(!list.is_empty());

        assert_eq!(list.pop(), Ok(5));
        assert_eq!(list.pop(), Ok(4));

        assert_eq!(list.pop(), Ok(1));
        assert_eq!(list.pop(), Err("Stack is empty"));

        assert!(list.is_empty());
    }

    #[test]
    fn peek() {
        let mut list = Stack::new();
        assert_eq!(list.peek(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        match list.peek_mut() {
            None => (),
            Some(value) => *value = 42,
        };

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Ok(42));
    }

    #[test]
    fn into_iter() {
        let mut list = Stack::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter_for_stack();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = Stack::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = Stack::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
