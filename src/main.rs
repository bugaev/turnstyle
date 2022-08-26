use std::rc::Rc;

#[derive(Debug)]
enum RotDir {
    Cw,
    Ccw,
}

#[derive(Debug)]
enum ShiftDir {
    Left,
    Right
}

#[derive(Debug)]
enum Operation {
    Shift(ShiftDir),
    Rotation(RotDir),
    Nop,
}

#[derive(Debug)]
struct MyList {
    op: Operation,
    parent: Option<Rc<MyList>>,
    value: i32,
}
impl MyList {

    const OPERATIONS: [Operation; 4] =
        [Operation::Shift(ShiftDir::Left), Operation::Shift(ShiftDir::Right), Operation::Rotation(RotDir::Cw), Operation::Rotation(RotDir::Ccw)];


    fn new(value: i32) -> Self {
        Self {
           op: Operation::Nop,
           parent: None,
           value,
        }
    }

    fn new_child(value: i32, op: Operation, parent: &Rc<MyList>) -> Self {
        Self {
           op,
           parent: Some(Rc::clone(parent)),
           value,
        }
    }
   
}

fn path_to_top(node_arg: &Rc<MyList>) {

    let mut node: &Rc<MyList> = node_arg;
    let mut value: i32;

    loop {
        value = node.value;
        println!("{}", value);
        match &node.parent  {
            None => return (),
            Some(parent) => node = &parent,
        }
    }
}



fn main() {

    let root_node = Rc::new(MyList::new(0));
    // let node1 = Rc::new(MyList::new_child(1, Operation::Shift(ShiftDir::Left), &node));

    let mut all_nodes1: Vec<Rc<MyList>> = Vec::new();
    all_nodes1.push(root_node);


    let mut cnt = 0;
    for generation in (1..4) {
        let mut all_nodes2: Vec<Rc<MyList>> = Vec::new();
        println!("---------- Generation {} -----------", generation);
        for child in all_nodes1.iter() {
            for op in MyList::OPERATIONS {
                cnt = cnt + 1;
                all_nodes2.push(Rc::new(MyList::new_child(cnt, op, child)));
                // println!("cnt: {}", cnt);
            }
        }
        all_nodes1 = all_nodes2;
    }

    path_to_top(&all_nodes1[all_nodes1.len() - 1]);

    // let node2 = Rc::new(MyList::new_child(2, &node));
    // println!("node: {:?}", node1);
    // println!("node: {:?}", node2);

    // let node = Box::new(MyList::new(0));
    // let node = Box::new(MyList::new_child(4,node));
    // let node = Box::new(MyList::new_child(5, node));
    // let mut adopted = Box::new(MyList::new(6));
    // adopted.become_child_of(node);
    // all_nodes.push(adopted);

    // for child in all_nodes.iter_mut() {
    //     child.value = child.value + 100;
    //     println!("child: {:?}", child);
    // }
    // println!("last node: {:?}", child3);
    println!("Success!");

}
