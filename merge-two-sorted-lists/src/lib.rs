// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Link
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }

  // from: https://rust-unofficial.github.io/too-many-lists/second-final.html
  fn push(&mut self, val: i32) {
        let new_node = Box::new(ListNode {
            val,
            next: self.next.take(),
        });

        self.next = Some(new_node);
    }
}

type Link = Option<Box<ListNode>>;

pub fn merge_two_lists(list1: Link, list2: Link) -> Link {
    match (list1, list2) {
        (None, None) => None,
        (Some(l1), None) => Some(l1),
        (None, Some(l2)) => Some(l2),
        (Some(l1), Some(l2)) => {
            let node = if l1.val <= l2.val {
                ListNode { val: l1.val, next: merge_two_lists(l1.next, Some(l2)) }
            } else {
                ListNode { val: l2.val, next: merge_two_lists(Some(l1), l2.next) }
            };

            Some(Box::new(node))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_1() {
        let mut l1 = Box::new(ListNode::new(1));
        l1.push(4);
        l1.push(2);

        let mut l2 = Box::new(ListNode::new(1));
        l2.push(4);
        l2.push(3);

        let mut expected = Box::new(ListNode::new(1)); // we could'd change the head
        expected.push(4);
        expected.push(4);
        expected.push(3);
        expected.push(2);
        expected.push(1);

        assert_eq!(merge_two_lists(Some(l1), Some(l2)), Some(expected));
    }

    #[test]
    fn test_2() {
        let l1 = None;
        let l2 = None;
        let expected = None;

        assert_eq!(merge_two_lists(l1, l2), expected);
    }

    #[test]
    fn test_3() {
        let l1 = None;
        let l2 = Some(Box::new(ListNode::new(0)));
        let expected = l2.clone();

        assert_eq!(merge_two_lists(l1, l2), expected);
    }
}
