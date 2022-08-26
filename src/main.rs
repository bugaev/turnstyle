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
    state: [u8; 12],
    op: Operation,
    parent: Option<Rc<MyList>>,
    value: i32,
}
impl MyList {

    const OPERATIONS: [Operation; 4] =
        [Operation::Shift(ShiftDir::Left), Operation::Shift(ShiftDir::Right), Operation::Rotation(RotDir::Cw), Operation::Rotation(RotDir::Ccw)];


    fn new(value: i32, state: [u8; 12]) -> Self {
        Self {
           state,
           op: Operation::Nop,
           parent: None,
           value,
        }
    }

    fn new_child(value: i32, op: Operation, parent: &Rc<MyList>) -> Self {
        Self {
           state: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
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

fn shift(dir: ShiftDir, node: &MyList) -> [u8; 12] {
    let mut res: [u8; 12] = [0; 12];
    match dir  {
        ShiftDir::Left =>
        {
            for ind in 0..res.len() - 1 {
                res[ind] = node.state[ind + 1];
            }
            res[res.len() - 1] = node.state[0];
        },
        ShiftDir::Right => 
        {
            for ind in 1..res.len() {
                res[ind] = node.state[ind - 1];
            }
            res[0] = node.state[res.len() - 1];
        },
    }
    res
}

fn main() {

    let root_node = Rc::new(MyList::new(0, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]));

    let shifted_left = shift(ShiftDir::Left, &root_node);
    println!("shifted left: {:?}", shifted_left);

    let shifted_right = shift(ShiftDir::Right, &root_node);
    println!("shifted right: {:?}", shifted_right);

    let mut all_nodes1: Vec<Rc<MyList>> = Vec::new();
    all_nodes1.push(root_node);


    let mut cnt = 0;
    for generation in 1..4 {
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


    println!("Success!");

}
