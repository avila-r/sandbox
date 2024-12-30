#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &value in values.iter().rev() {
            let mut node = Box::new(ListNode::new(value));
            node.next = current;
            current = Some(node);
        }
        current
    }

    pub fn to(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = vec![];
        let mut current = head;
        while let Some(node) = current {
            result.push(node.val);
            current = node.next;
        }
        result
    }
}

pub fn reverse_linked_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut current, mut new) = (head.as_ref(), None);

    while let Some(n) = current {
        let mut node = ListNode::new(n.val);
        node.next = new;
        new = Some(Box::new(node));
        current = n.next.as_ref();
    }

    new
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
                ListNode::to(ListNode::from(case.expected.clone())),
                case.expected
            };

            let result = reverse_linked_list(ListNode::from(case.input.clone()));

            assert_eq! {
                ListNode::to(result),
                case.expected
            };
        });
    }
}
