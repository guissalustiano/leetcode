// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Link
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct List(Link);

impl List {
  #[inline]
  fn new() -> Self {
      List(None)
  }

  pub fn from_vec(vs: Vec<i32>) -> Self {
    vs.into_iter().rev().fold(Self::new(), |head, v| head.push(v))
  }

  fn push(self, val: i32) -> Self {
      List(Some(Box::new(ListNode { val, next: self.0 })))
  }
}

type Link = Option<Box<ListNode>>;

pub fn len(head: &Link) -> usize {
    let mut counter = 0;
    let mut cursor = head;
    while let Some(node) = cursor {
        cursor = &node.next;
        counter += 1;
    }
    counter
}

// also could be resolved with slow/fast pointers
pub fn middle_node(head: Link) -> Link {
    let size = len(&head);

    let mut cursor = head;
    for _ in 0..(size/2) {
        // how we check the size before, we could safe unwrap here
        cursor = cursor.unwrap().next;
    }
    cursor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let head = List::from_vec(vec![1, 2, 3, 4, 5]);
        let expected = List::from_vec(vec![3, 4, 5]);

        assert_eq!(middle_node(head.0), expected.0);
    }

    #[test]
    fn example_2() {
        let head = List::from_vec(vec![1, 2, 3, 4, 5, 6]);
        let expected = List::from_vec(vec![4, 5, 6]);

        assert_eq!(middle_node(head.0), expected.0);
    }
}
