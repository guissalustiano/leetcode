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
        let l1 = List::from_vec(vec![1, 2, 4]);
        let l2 = List::from_vec(vec![1, 3, 4]);
        let expected = List::from_vec(vec![1, 1, 2, 3, 4, 4]);

        assert_eq!(merge_two_lists(l1.0, l2.0), expected.0);
    }

    #[test]
    fn test_2() {
        let l1 = List::from_vec(vec![]);
        let l2 = List::from_vec(vec![]);
        let expected = List::from_vec(vec![]);

        assert_eq!(merge_two_lists(l1.0, l2.0), expected.0);
    }

    #[test]
    fn test_3() {
        let l1 = List::from_vec(vec![]);
        let l2 = List::from_vec(vec![0]);
        let expected = List::from_vec(vec![0]);

        assert_eq!(merge_two_lists(l1.0, l2.0), expected.0);
    }
}
