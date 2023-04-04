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
        let l1 = List::from_vec(vec![1, 2, 3, 4, 5]);
        let expected = List::from_vec(vec![5, 4, 3, 2, 1]);

        assert_eq!(reverse_list(l1.0), expected.0);
    }
}
