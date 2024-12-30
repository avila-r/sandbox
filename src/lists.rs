type Node = Option<Box<ListNode>>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Node,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from(values: Vec<i32>) -> Node {
        let mut current = None;
        for &value in values.iter().rev() {
            let mut node = Box::new(ListNode::new(value));
            node.next = current;
            current = Some(node);
        }
        current
    }

    pub fn to_vec(head: Node) -> Vec<i32> {
        let mut result = vec![];
        let mut current = head;
        while let Some(node) = current {
            result.push(node.val);
            current = node.next;
        }
        result
    }
}

pub fn reverse_linked_list(head: Node) -> Node {
    let (mut current, mut new) = (head.as_ref(), None);

    while let Some(n) = current {
        let mut node = ListNode::new(n.val);
        node.next = new;
        new = Some(Box::new(node));
        current = n.next.as_ref();
    }

    new
}

pub fn middle_node(head: Node) -> Node {
    let (mut slow, mut fast) = (head.clone(), head.and_then(|n| n.next));

    while let Some(n) = fast {
        slow = slow.and_then(|s| s.next);
        fast = n.next.and_then(|f| f.next);
    }

    slow
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_linked_list() {
        struct TestCase {
            input: Vec<i32>,
            expected: Vec<i32>,
        }

        let cases = vec![
            TestCase {
                input: vec![1, 2, 3, 4, 5],
                expected: vec![5, 4, 3, 2, 1],
            },
            TestCase {
                input: vec![1, 2],
                expected: vec![2, 1],
            },
            TestCase {
                input: vec![],
                expected: vec![],
            },
            TestCase {
                input: vec![10],
                expected: vec![10],
            },
            TestCase {
                input: vec![1, 3, 2],
                expected: vec![2, 3, 1],
            },
        ];

        cases.iter().for_each(|case| {
            assert_eq! {
                ListNode::to_vec(ListNode::from(case.expected.clone())),
                case.expected
            };

            let result = reverse_linked_list(ListNode::from(case.input.clone()));

            assert_eq! {
                ListNode::to_vec(result),
                case.expected
            };
        });
    }

    #[test]
    fn test_middle_node() {
        struct TestCase {
            input: Vec<i32>,
            expected: Vec<i32>,
        }

        let cases = vec![
            TestCase {
                input: vec![1, 2, 3, 4, 5],
                expected: vec![3, 4, 5],
            },
            TestCase {
                input: vec![1, 2, 3, 4, 5, 6],
                expected: vec![4, 5, 6],
            },
            TestCase {
                input: vec![1],
                expected: vec![1],
            },
            TestCase {
                input: vec![1, 2],
                expected: vec![2],
            },
            TestCase {
                input: vec![],
                expected: vec![],
            },
        ];

        cases.iter().for_each(|case| {
            let result = middle_node(ListNode::from(case.input.clone()));

            assert_eq! {
                ListNode::to_vec(result),
                case.expected
            };
        });
    }
}
