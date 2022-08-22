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

// struct MyList<'a> {
//     parent: Option<&'a MyList<'a>>,
//     value: i32,
// }
// impl<'a> MyList<'a> {
//     fn new(value: i32) -> Self {
//         Self {
//            parent: None,
//            value,
//         }
//     }
//    
//     fn add(&mut self, other: &'a MyList) {
//         self.parent = Some(other);
//     }
// }


// fn conn_to_prev<'a>(v: &'a mut Vec<MyList<'a>>) {
// fn conn_to_prev<'a>(mut v: Vec<MyList<'a>>) -> Vec<MyList<'a>> {
//     let len = v.len();
//     let (left, right) = v.split_at_mut(len - 1);
//     let last = &mut right[0];
//     let prev = &left[len - 2];
//     last.parent = Some(prev);
//     v
// }

fn main() {

    // let mut all_nodes: Vec<Box<MyList>> = Vec::new();
    // all_nodes.push(Box())
    let root_node = Box::new(MyList::new(0));
    let child1 = Box::new(MyList::new_child(1, root_node));
    let child2 = Box::new(MyList::new_child(2,child1));
    let child3 = Box::new(MyList::new_child(3,child2));
    println!("last node: {:?}", child3);
    let mut child4 = Box::new(MyList::new(4));
    child4.become_child_of(child3);
    println!("last node: {:?}", child4);
    // all_nodes.push(root_node);
    // all_nodes.push(other_node);
    println!("Success!");

    // allNodes.push(MyList::new(0));
    // allNodes.push(MyList::new(1));


    // let len = allNodes.len();
    // let (left, right) = allNodes.split_at_mut(len - 1);
    // let last = &mut right[0];
    // let prev = &left[len - 2];
    // last.parent = Some(prev);

    // conn_to_prev(& mut allNodes);
    // let allNodes = conn_to_prev(allNodes);

    // println!("last node: {:?}", allNodes);
    // println!("last node: {:?}", last);
    // let mut val1 = MyList::new(1);
    // let mut val2 = MyList::new(2);
    // let val3 = MyList::new(3);
    // val1.add(&val2);
    // val2.add(&val3);
    // println!("val1 value is {:?}", val1);
    // println!("val2 value is {:?}", val2);
    // println!("val3 value is {:?}", val3);
}
