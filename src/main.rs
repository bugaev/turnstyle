const MAX_DEPTH: u16 = 20;
const MAX_SHIFTS: u8 = 11;

use std::{rc::Rc, process::exit,}; // process::exit};

type State = [u8; 12];

#[derive(Debug)]
enum ShiftDir {
    Left,
    Right
}

#[derive(Debug)]
enum Operation {
    Shift(ShiftDir),
    Rotation,
    Nop,
}

#[derive(Debug)]
struct MyList {
    state: [u8; 12],
    op: Operation,
    parent: Option<Rc<MyList>>,
}
impl MyList {

    const OPERATIONS: [Operation; 3] =
        [Operation::Shift(ShiftDir::Left), Operation::Shift(ShiftDir::Right), Operation::Rotation];


    fn new(state: [u8; 12]) -> Self {
        Self {
           state,
           op: Operation::Nop,
           parent: None,
        }
    }

    fn new_child(op: Operation, parent: &Rc<MyList>) -> Self {
        Self {
            state: transform(&op, parent),
            op,
            parent: Some(Rc::clone(parent)),
        }
    }
   
}

fn path_to_top(node_arg: &Rc<MyList>) {

    let mut node: &Rc<MyList> = node_arg;
    let mut op;

    loop {
        op = &node.op;
        println!("{:?}", op);
        match &node.parent  {
            None => return (),
            Some(parent) => node = &parent,
        }
    }
}

fn shift(dir: &ShiftDir, node: &MyList) -> [u8; 12] {
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

fn state_shift(dir: &ShiftDir, state: &State) -> State {
    let mut res: State = [0; 12];
    match dir  {
        ShiftDir::Left =>
        {
            for ind in 0..res.len() - 1 {
                res[ind] = state[ind + 1];
            }
            res[res.len() - 1] = state[0];
        },
        ShiftDir::Right => 
        {
            for ind in 1..res.len() {
                res[ind] = state[ind - 1];
            }
            res[0] = state[res.len() - 1];
        },
    }
    res
}

fn rotation(node: &MyList) -> [u8; 12] {
    let mut res: [u8; 12] = [0; 12];
    for ind in 4..res.len() {
        res[ind] = node.state[ind];
    }
    res[0] = node.state[1];
    res[1] = node.state[0];
    res[2] = node.state[3];
    res[3] = node.state[2];
    res
}

fn state_rotation(state: &State) -> State {
    let mut res: State = [0; 12];
    for ind in 4..res.len() {
        res[ind] = state[ind];
    }
    res[0] = state[1];
    res[1] = state[0];
    res[2] = state[3];
    res[3] = state[2];
    res
}

fn transform(op: &Operation, node: &MyList) -> [u8; 12] {
    match op  {
        Operation::Rotation => rotation(node),
        Operation::Shift(dir) => shift(dir, node),
        Operation::Nop => [0; 12],
    }

}

// HERE ->
fn state_transform(op: &Operation, state: &State) -> State {
    match op  {
        Operation::Rotation => state_rotation(state),
        Operation::Shift(dir) => state_shift(dir, state),
        Operation::Nop => [0; 12],
    }
}

// state is passed by value: https://stackoverflow.com/a/68217529
fn test_solution(depth: u16, mut path: Vec<Operation>, state: &State, solved: &State, mut rotations: u8, mut shifts: u8) -> Vec<Operation> {
    if depth > MAX_DEPTH { return path; };
    // if depth == 4 { println!("{:?}", path) }
    // println!("{:?}", state);
    let last_state_opt = path.last();
    match last_state_opt {
        None => {
            println!("ERROR: Did you forget to init the path with Nop?");
            exit(1);
        }, 
        Some(op) => {
            let mut mutate_state = true;
            let new_state;
            // Seems OK to mix op: &Operation and Operation::Nop?
            if let Operation::Nop = op {
                mutate_state = false
            }
            
            if mutate_state {
                new_state = state_transform(op, state);
                
                if new_state == *solved {
                    println!("Solution: {:?}", path);
                };
            } else {
                new_state = *state;
            };

            for nextop in vec![Operation::Rotation, Operation::Shift(ShiftDir::Right)] {
                let mut new_rotations: u8 = 0;
                let mut new_shifts: u8 = 0;
                match nextop  {
                    Operation::Rotation =>
                        if rotations == 0 {
                            new_rotations = rotations + 1;
                            new_shifts = 0;
                        } else {
                            continue
                        },
                    Operation::Shift(_) =>
                        if shifts < MAX_SHIFTS - 1 {
                            new_rotations = 0;
                            new_shifts = shifts + 1;
                        } else {
                            continue
                        },
                    Operation::Nop => (),
                }
                path.push(nextop);
                path = test_solution(depth + 1, path, &new_state, solved, new_rotations, new_shifts);
                path.pop();
            }
    }
    }
    path
}

fn main() {

    println!("Start of the program.");

    let solved: [u8; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    // let solved: [u8; 12] = [2, 1, 6, 5, 7, 8, 9, 10, 11, 3, 4, 12];
    
    let root_node = Rc::new(MyList::new([1, 2, 3, 4, 5, 6, 7, 8, 11, 12, 9, 10]));

    let path: Vec<Operation> = vec![Operation::Nop];
    // test_solution(1, path, &[1, 2, 3, 4, 5, 6, 7, 8, 11, 12, 9, 10], &solved);
    test_solution(1, path, &[2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 1], &solved, 0, 0);

    println!("End of the program.");

    exit(0);

    let mut all_nodes1: Vec<Rc<MyList>> = Vec::new();
    all_nodes1.push(root_node);


    let mut cnt = 0;
    'generations: for generation in 1..19 {
        let mut all_nodes2: Vec<Rc<MyList>> = Vec::new();
        println!("---------- Generation {} -----------", generation);
        for child in all_nodes1.iter() {
            for op in MyList::OPERATIONS {
                cnt = cnt + 1;
                all_nodes2.push(Rc::new(MyList::new_child(op, child)));
                let last_node = &all_nodes2[all_nodes2.len() - 1];
                if last_node.state == solved {
                   path_to_top(last_node); 
                    println!("!!! Success !!!");
                   break 'generations;
                   // exit(0);
                }
                // println!("cnt: {}", cnt);
            }
        }
        all_nodes1 = all_nodes2;
    }

    // path_to_top(&all_nodes1[all_nodes1.len() - 1]);


    println!("End of the program.");

}
