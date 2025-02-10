use std::cell::RefCell;

#[derive(Debug)]
struct RecursiveNode<'a> {
    data: i32,
    next: Option<&'a RecursiveNode<'a>>,
}

fn new_chain() {
    let mut first_node = RecursiveNode {
        data: 1,
        next: None,
    };

    let second_node = RecursiveNode {
        data: 2,
        next: Some(&first_node),
    };
    
    first_node.next = Some(&second_node);
}