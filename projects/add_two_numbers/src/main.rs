// Definition for singly-linked list.
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
}


pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => None,
        (Some(n), None) => Some(n),
        (None, Some(n)) => Some(n),
        (Some(n1), Some(n2)) => {
            let mut sum = n1.val + n2.val;
            if sum < 10 {
              Some(Box::new(ListNode{val: sum, next: add_two_numbers(n1.next, n2.next)}))
            }
            else {
                let carry = Some(Box::new(ListNode::new(sum/10)));
                sum = sum%10;
                Some(Box::new(ListNode {val: sum, next: add_two_numbers(add_two_numbers(n1.next, carry), n2.next)}))
            }
        }
    }
}


fn main() {
    println!("Hello, world!");
}
