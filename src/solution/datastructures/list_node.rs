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

  pub fn get_last_node(&mut self) -> &mut Self {
    if let Some(ref mut box_node) = self.next {
      box_node.get_last_node()
    } else {
      self
    }
  }

  pub fn append(&mut self, val: i32) {
    let new_node = ListNode::new(val);
    self.get_last_node().next = Some(Box::new(new_node));
  }
}
