use std::{cell::RefCell, fmt::Display, rc::Rc};

type Link<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    pub value: T,
    pub next: Option<Link<T>>,
    pub prev: Option<Link<T>>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
}

#[derive(Debug)]
pub struct DoubleLinkedList<T> {
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Rc<RefCell<Self>> {
        let cell = RefCell::new(Node {
            value,
            next: None,
            prev: None,
        });
        Rc::new(cell)
    }
}

impl<T: Display + Clone> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn of(values: &[T]) -> Self {
        let mut list = Self::new();
        for value in values {
            list.append(value.clone());
        }
        list
    }

    pub fn append(&mut self, value: T) {
        let new = Node::new(value);

        match self.tail.take() {
            Some(old) => {
                old.borrow_mut().next = Some(new.clone());
                self.tail = Some(new);
            }
            None => {
                self.head = Some(new.clone());
                self.tail = Some(new);
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(old) = self.head.take() {
            self.head = old.borrow_mut().next.take();
            if self.head.is_none() {
                self.tail = None;
            }
            Some(Rc::try_unwrap(old).ok().unwrap().into_inner().value)
        } else {
            None
        }
    }

    pub fn print_all(&self) {
        let mut current = self.head.clone();
        while let Some(node) = current {
            print!("{} -> ", node.borrow().value);
            current = node.borrow().next.clone();
        }
        println!("None");
    }
}

impl<T: Display + Clone> DoubleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn of(values: &[T]) -> Self {
        let mut list = Self::new();
        for value in values {
            list.append(value.clone());
        }
        list
    }

    pub fn append(&mut self, value: T) {
        let new = Node::new(value);

        match self.tail.take() {
            Some(old) => {
                old.borrow_mut().next = Some(new.clone());
                new.borrow_mut().prev = Some(old);
                self.tail = Some(new);
            }
            None => {
                self.head = Some(new.clone());
                self.tail = Some(new);
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.tail.take().map(|old| {
            if let Some(prev) = old.borrow_mut().prev.take() {
                prev.borrow_mut().next = None;
                self.tail = Some(prev);
            } else {
                self.head = None;
            }

            Rc::try_unwrap(old).ok().unwrap().into_inner().value
        })
    }

    pub fn print_all(&self) {
        let mut current = self.head.clone();
        while let Some(node) = current {
            print!("{} -> ", node.borrow().value);
            current = node.borrow().next.clone();
        }
        println!("None");
    }
}

#[cfg(test)]
mod linked_list_tests {
    use super::*;

    #[test]
    fn test_append() {
        let mut list = LinkedList::new();
        list.append(10);
        list.append(20);
        list.append(30);

        let expected = "10 -> 20 -> 30 -> None";
        let mut current = list.head.clone();
        let mut result = String::new();

        while let Some(node) = current {
            result.push_str(&format!("{} -> ", node.borrow().value));
            current = node.borrow().next.clone();
        }
        result.push_str("None");

        assert_eq!(result, expected);
    }

    #[test]
    fn test_pop() {
        let mut list = LinkedList::new();
        list.append(10);
        list.append(20);
        list.append(30);

        let popped = list.pop();
        assert_eq!(popped, Some(10));

        let expected = "20 -> 30 -> None";
        let mut current = list.head.clone();
        let mut result = String::new();

        while let Some(node) = current {
            result.push_str(&format!("{} -> ", node.borrow().value));
            current = node.borrow().next.clone();
        }
        result.push_str("None");

        assert_eq!(result, expected);
    }

    #[test]
    fn test_pop_all() {
        let mut list = LinkedList::new();
        list.append(10);
        list.append(20);
        list.append(30);

        assert_eq!(list.pop(), Some(10));
        assert_eq!(list.pop(), Some(20));
        assert_eq!(list.pop(), Some(30));

        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_append_and_pop_repeatedly() {
        let mut list = LinkedList::new();

        list.append(10);
        assert_eq!(list.pop(), Some(10));

        list.append(20);
        list.append(30);
        assert_eq!(list.pop(), Some(20));
        assert_eq!(list.pop(), Some(30));

        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_print_empty_list() {
        let list: LinkedList<i32> = LinkedList::new();
        let expected = "None";
        let mut current = list.head.clone();
        let mut result = String::new();

        while let Some(node) = current {
            result.push_str(&format!("{} -> ", node.borrow().value));
            current = node.borrow().next.clone();
        }
        result.push_str("None");

        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_element() {
        let mut list = LinkedList::new();

        list.append(10);

        let expected = "10 -> None";
        let mut current = list.head.clone();
        let mut result = String::new();

        while let Some(node) = current {
            result.push_str(&format!("{} -> ", node.borrow().value));
            current = node.borrow().next.clone();
        }
        result.push_str("None");

        assert_eq!(result, expected);

        assert_eq!(list.pop(), Some(10));
        assert_eq!(list.pop(), None);
    }
}

#[cfg(test)]
mod double_linked_list_tests {
    use super::*; // Import the structures and methods to test

    /// Test appending elements to the list
    #[test]
    fn test_append() {
        let mut list = DoubleLinkedList::new();

        // Append some elements
        list.append(10);
        list.append(20);
        list.append(30);

        // Test the list print output manually (we expect: 10 -> 20 -> 30 -> None)
        let expected = "10 -> 20 -> 30 -> None";
        let mut current = list.head.clone();
        let mut result = String::new();

        while let Some(node) = current {
            result.push_str(&format!("{} -> ", node.borrow().value));
            current = node.borrow().next.clone();
        }
        result.push_str("None");

        assert_eq!(result, expected);
    }

    /// Test popping an element from the list
    #[test]
    fn test_pop() {
        let mut list = DoubleLinkedList::new();

        // Append some elements
        list.append(10);
        list.append(20);
        list.append(30);

        // Pop the last element (should be 30)
        let popped = list.pop();
        assert_eq!(popped, Some(30));

        // Check the remaining elements in the list
        let expected = "10 -> 20 -> None";
        let mut current = list.head.clone();
        let mut result = String::new();

        while let Some(node) = current {
            result.push_str(&format!("{} -> ", node.borrow().value));
            current = node.borrow().next.clone();
        }
        result.push_str("None");

        assert_eq!(result, expected);
    }

    /// Test popping all elements from the list
    #[test]
    fn test_pop_all() {
        let mut list = DoubleLinkedList::new();

        // Append some elements
        list.append(10);
        list.append(20);
        list.append(30);

        // Pop all elements and check them
        assert_eq!(list.pop(), Some(30)); // First pop (should be 30)
        assert_eq!(list.pop(), Some(20)); // Second pop (should be 20)
        assert_eq!(list.pop(), Some(10)); // Third pop (should be 10)

        // The list should be empty now
        assert_eq!(list.pop(), None);
    }

    /// Test appending and popping elements repeatedly
    #[test]
    fn test_append_and_pop_repeatedly() {
        let mut list = DoubleLinkedList::new();

        // Append elements and pop them immediately
        list.append(10);
        assert_eq!(list.pop(), Some(10));

        list.append(20);
        list.append(30);
        assert_eq!(list.pop(), Some(30));
        assert_eq!(list.pop(), Some(20));

        // The list should be empty now
        assert_eq!(list.pop(), None);
    }

    /// Test printing the list when it is empty
    #[test]
    fn test_print_empty_list() {
        let list: DoubleLinkedList<i32> = DoubleLinkedList::new();
        let expected = "None";
        let mut current = list.head.clone();
        let mut result = String::new();

        while let Some(node) = current {
            result.push_str(&format!("{} -> ", node.borrow().value));
            current = node.borrow().next.clone();
        }
        result.push_str("None");

        assert_eq!(result, expected);
    }

    /// Test the behavior when there's a single element in the list
    #[test]
    fn test_single_element() {
        let mut list = DoubleLinkedList::new();

        // Append a single element
        list.append(10);

        // Check the list
        let expected = "10 -> None";
        let mut current = list.head.clone();
        let mut result = String::new();

        while let Some(node) = current {
            result.push_str(&format!("{} -> ", node.borrow().value));
            current = node.borrow().next.clone();
        }
        result.push_str("None");

        assert_eq!(result, expected);

        // Pop the single element
        assert_eq!(list.pop(), Some(10));

        // The list should be empty now
        assert_eq!(list.pop(), None);
    }
}
