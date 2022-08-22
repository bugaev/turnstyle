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

    let mut all_nodes: Vec<Box<MyList>> = Vec::new();
    let node = Box::new(MyList::new(0));
    let node = Box::new(MyList::new_child(1,node));
    let node = Box::new(MyList::new_child(2, node));
    let mut adopted = Box::new(MyList::new(3));
    adopted.become_child_of(node);
    all_nodes.push(adopted);

    let node = Box::new(MyList::new(0));
    let node = Box::new(MyList::new_child(4,node));
    let node = Box::new(MyList::new_child(5, node));
    let mut adopted = Box::new(MyList::new(6));
    adopted.become_child_of(node);
    all_nodes.push(adopted);

    for child in all_nodes.iter_mut() {
        child.value = child.value + 100;
        println!("child: {:?}", child);
    }
    // println!("last node: {:?}", child3);
    println!("Success!");

}
