#[derive(Debug)]
pub struct Queue<T> {
    vector: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self { vector: Vec::new() }
    }
    pub fn enqueue(&mut self, element: T) {
        self.vector.push(element);
    }

    pub fn dequeue(&mut self) -> Result<T, Error> {
        let first_position = 0;

        if self.len() == 0 {
            return Err(Error::NotPossibleDequeue);
        }

        Ok(self.vector.remove(first_position))
    }

    pub fn is_empty(&self) -> bool {
        self.vector.is_empty()
    }

    pub fn len(&self) -> usize {
        self.vector.len()
    }

    pub fn peek(&self) -> Result<&T, Error> {
        if self.len() == 0 {
            return Err(Error::DontHaveElement);
        }

        let from_element = self.vector.first();

        match from_element {
            Some(x) => Ok(x),
            _ => Err(Error::DontHaveElement),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Error {
    NotPossibleDequeue,
    DontHaveElement,
}

#[cfg(test)]
#[test]
fn len_add_multiple_element_to_queue() {
    let mut queue: Queue<u8> = Queue::new();
    let first_element = 5;
    let second_elent = 3;
    let third_element = 1;
    let four_element = 8;

    queue.enqueue(first_element);
    queue.enqueue(second_elent);
    queue.enqueue(third_element);
    queue.enqueue(four_element);

    assert_eq!(4, queue.len());
}

#[test]
fn remove_multiple_element_to_queue() {
    let mut queue: Queue<u8> = Queue::new();
    let first_element = 5;
    let second_elent = 3;
    let third_element = 1;

    queue.enqueue(first_element);
    queue.enqueue(second_elent);
    queue.enqueue(third_element);

    queue.dequeue();
    queue.dequeue();
    let dequeue = queue.dequeue().unwrap();

    assert_eq!(1, dequeue);
}

#[test]
fn add_multiple_element_to_queue() {
    let mut queue: Queue<u8> = Queue::new();
    let first_element = 5;
    let second_elent = 3;
    let third_element = 1;
    let four_element = 8;

    queue.enqueue(first_element);
    queue.enqueue(second_elent);
    queue.enqueue(third_element);
    queue.enqueue(four_element);

    queue.dequeue();
    queue.dequeue();

    let element = queue.dequeue().unwrap();

    assert_eq!(third_element, element);
}

#[test]
fn get_front_element() {
    let mut queue: Queue<u8> = Queue::new();
    let first_element = 5;
    let second_elent = 3;
    let third_element = 1;
    let four_element = 8;

    queue.enqueue(first_element);
    queue.enqueue(second_elent);
    queue.enqueue(third_element);
    queue.enqueue(four_element);

    queue.dequeue();
    queue.dequeue();

    let element = queue.peek().unwrap();

    assert_eq!(&third_element, element);
}

#[test]
fn empty_queue() {
    let mut queue: Queue<u8> = Queue::new();
    let first_element = 5;
    let second_elent = 3;

    queue.enqueue(first_element);
    queue.enqueue(second_elent);

    queue.dequeue();
    queue.dequeue();

    assert_eq!(true, queue.is_empty());
}

#[test]
fn remove_element_empty_queue() {
    let mut queue: Queue<u8> = Queue::new();
    let first_element = 5;
    let second_elent = 3;

    queue.enqueue(first_element);
    queue.enqueue(second_elent);

    queue.dequeue();
    queue.dequeue();

    let dequeue = queue.dequeue();

    assert_eq!(Err(Error::NotPossibleDequeue), dequeue);
}

#[test]
fn peek_element_empty_queue() {
    let mut queue: Queue<u8> = Queue::new();
    let first_element = 5;
    let second_elent = 3;

    queue.enqueue(first_element);
    queue.enqueue(second_elent);

    queue.dequeue();
    queue.dequeue();

    let dequeue = queue.peek();

    assert_eq!(Err(Error::DontHaveElement), dequeue);
}
