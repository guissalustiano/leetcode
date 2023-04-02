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

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut current = head;
    let mut previous = None;
    while let Some(mut node) = current {
        current = node.next.take();
        node.next = previous;
        previous = Some(node);
    };
    previous
}

// also has a recursive solution

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
