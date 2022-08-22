#[derive(Debug)]
struct MyList {
    parent: Option<Box<MyList>>,
    value: i32,
}
impl MyList {
    fn new(value: i32) -> Self {
        Self {
           parent: None,
           value,
        }
    }

    fn new_child(value: i32, parent: Box<MyList>) -> Self {
        Self {
           parent: Some(parent),
           value,
        }
    }
   
    fn become_child_of(&mut self, parent: Box<MyList>) {
        self.parent = Some(parent);
    }
}

fn main() {

    // let mut all_nodes: Vec<Box<MyList>> = Vec::new();
    let root_node = Box::new(MyList::new(0));
    let child1 = Box::new(MyList::new_child(1, root_node));
    let child2 = Box::new(MyList::new_child(2,child1));
    let mut child3 = Box::new(MyList::new(3));
    child3.become_child_of(child2);
    println!("last node: {:?}", child3);
    println!("Success!");

}
