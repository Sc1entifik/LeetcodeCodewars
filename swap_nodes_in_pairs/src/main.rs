fn main() {
    let head = create_test_values();
    println!("head: {:?}\n", head);

    println!("swapped nodes: {:?}", swap_pairs(head));
}


#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode{
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode{
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { 
            next: None,
            val
        }
    }
}


fn set_next_node(tail_node: Option<Box<ListNode>>, head_node: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head_node {
        None => None,
        Some(mut head) => {
            head.next = tail_node;
            return Some(head);
        }
    }
}


fn create_test_values() -> Option<Box<ListNode>> {
    return [4,3,2,1]
        .map(|x| Some(Box::new(ListNode::new(x))))
        .into_iter()
        .reduce(|x, y| set_next_node(x, y))
        .unwrap();
}


fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head.clone(){
        None => None,
        Some(mut old_head_node) => {
            let new_head = old_head_node.next;

            match new_head {
                None => head,
                Some(mut new_head_node) => {
                    let old_head_next = swap_pairs(new_head_node.next);
                    old_head_node.next = old_head_next;
                    new_head_node.next = Some(old_head_node);
                    return Some(new_head_node);
                }
            }
            
        }
    }
}
