#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
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

pub fn reverse_list(head: Link) -> Link {
    let mut current = head;
    let mut previous = None;
    while let Some(mut node) = current {
        current = std::mem::replace(&mut node.next, previous);
        previous = Some(node);
    };
    previous
}

// also has a recursive solution
// https://leetcode.com/problems/reverse-linked-list/solutions/3162279/idiomatic-rust-with-recursion-analysis/?envType=study-plan&id=level-1
// ```cur_prev(head, None) ```
pub fn cur_prev(cur: Link, prev: Link) -> Link {
    match cur {
        Some(mut node) => cur_prev(std::mem::replace(&mut node.next, prev), Some(node)),
        None => prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use pretty_assertions::assert_eq;

    #[test]
    fn test_1() {
        let mut l1 = Box::new(ListNode::new(1));
        l1.push(5);
        l1.push(4);
        l1.push(3);
        l1.push(2);

        let mut expected = Box::new(ListNode::new(5)); // we could'd change the head
        expected.push(1);
        expected.push(2);
        expected.push(3);
        expected.push(4);

        assert_eq!(reverse_list(Some(l1)), Some(expected));
    }
}
