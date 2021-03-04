use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node { val: t, next: None }
    }
}

pub struct LinkedList<T> {
    length: usize,
    start: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
        }
    }

    pub fn insert_head(&mut self, data: T) {
        self.insert_at_index(0, data)
    }

    pub fn insert(&mut self, data: T) {
        self.insert_at_index(self.length, data)
    }

    pub fn insert_at_index(&mut self, index: usize, data: T) {
        if index > self.length {
            panic!("list index out of range")
        }
        let mut node = Box::new(Node::new(data));
        if self.start.is_none() {
            self.start = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        } else if index == 0 {
            node.next = self.start;
            self.start = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        } else {
            let mut tmp = self.start;
            for _ in 0..index - 1 {
                tmp = unsafe { &*tmp.as_ref().unwrap().as_ptr() }.next;
            }
            node.next = unsafe { &*tmp.as_ref().unwrap().as_ptr() }.next;
            unsafe { &mut *tmp.as_ref().unwrap().as_ptr() }.next =
                Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        }
        self.length += 1;
    }

    pub fn delete_head(&mut self) -> Option<T> {
        self.delete_at_index(0)
    }
    pub fn delete_tail(&mut self) -> Option<T> {
        if self.length > 0 {
            self.delete_at_index(self.length - 1)
        } else {
            None
        }
    }

    pub fn delete_at_index(&mut self, index: usize) -> Option<T> {
        assert!(index <= self.length, "list index out of range");
        self.start?;
        self.length -= 1;
        if index == 0 {
            let node = unsafe { std::ptr::read(self.start.unwrap().as_ptr()) };
            self.start = node.next;
            Some(node.val)
        } else {
            let mut tmp = self.start;
            for _ in 0..index - 1 {
                tmp = unsafe { tmp.unwrap().as_ref() }.next;
            }
            let delete_node =
                unsafe { std::ptr::read((tmp.unwrap().as_ref()).next.unwrap().as_ptr()) };
            unsafe {
                tmp.unwrap().as_mut().next = tmp.unwrap().as_ref().next.unwrap().as_ref().next
            };
            Some(delete_node.val)
        }
    }
    //pub fn add(&mut self, obj: T) {
    //    let mut node = Box::new(Node::new(obj));
    //    // Since we are adding node at the end, next will always be None
    //    node.next = None;
    //    node.prev = self.end;
    //    // Get a pointer to node
    //    let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
    //    match self.end {
    //        // This is the case of empty list
    //        None => self.start = node_ptr,
    //        Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
    //    }
    //    self.end = node_ptr;
    //    self.length += 1;
    //}

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.insert(1);
        list.insert(2);
        list.insert(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.insert("A".to_string());
        list_str.insert("B".to_string());
        list_str.insert("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn get_by_index_in_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.insert(1);
        list.insert(2);
        println!("Linked List is {}", list);
        let retrived_item = list.get(1);
        assert!(retrived_item.is_some());
        assert_eq!(2 as i32, *retrived_item.unwrap());
    }

    #[test]
    fn get_by_index_in_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.insert("A".to_string());
        list_str.insert("B".to_string());
        println!("Linked List is {}", list_str);
        let retrived_item = list_str.get(1);
        assert!(retrived_item.is_some());
        assert_eq!("B", *retrived_item.unwrap());
    }

    #[test]
    fn delete_various() {
        let mut list = LinkedList::<usize>::new();
        assert_eq!(list.delete_head(), None);
        assert_eq!(list.delete_tail(), None);
        for i in 0..5 {
            list.insert(i);
        }
        assert_eq!(list.delete_tail(), Some(4));
        assert_eq!(list.delete_head(), Some(0));
        assert_eq!(list.delete_at_index(1), Some(2));
        assert_eq!(list.len(), 2);
        assert_eq!(list.delete_head(), Some(1));
        assert_eq!(list.delete_head(), Some(3));
        assert_eq!(list.delete_head(), None);
        assert_eq!(list.delete_tail(), None);
    }
}
