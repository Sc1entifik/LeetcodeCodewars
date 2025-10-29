fn main() {
    let head = create_test_values();
    let reversed_ll = reverse_list(head);
    println!("{:?}", reversed_ll);
}


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


fn set_next_node(tail_node: Option<Box<ListNode>>, current_node: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match current_node {
        None => None,
        Some(mut head) => {
            head.next = tail_node;
            return Some(head);
        }
    }
}


fn create_test_values() -> Option<Box<ListNode>> {
    return [5,4,3,2,1]
        .map(|x| Some(Box::new(ListNode::new(x))))
        .into_iter()
        .reduce(|x, y| set_next_node(x, y))
        .unwrap();
}


pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn helper(current: Option<Box<ListNode>>, next: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match current {
            None => None,
            Some(mut current_node) => {
                let new_next = current_node.next;
                current_node.next = next;

                match new_next {
                    None => Some(current_node),
                    Some(new_head_node) => helper(Some(new_head_node), Some(current_node))
                }
            }
        }
    }
    
    helper(head.clone(), None)
}
