#[derive(Debug)]
struct MyList<'a> {
    parent: Option<&'a MyList<'a>>,
    value: i32,
}

impl<'a> MyList<'a> {
    fn new(value: i32) -> Self {
        Self {
           parent: None,
           value,
        }
    }
   
    fn add(&mut self, other: &'a MyList) {
        self.parent = Some(other);
    }
}


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

    let mut allNodes: Vec<MyList> = Vec::new();

    println!("Hello, world!");

    allNodes.push(MyList::new(0));
    allNodes.push(MyList::new(1));


    // let len = allNodes.len();
    // let (left, right) = allNodes.split_at_mut(len - 1);
    // let last = &mut right[0];
    // let prev = &left[len - 2];
    // last.parent = Some(prev);

    // conn_to_prev(& mut allNodes);
    let allNodes = conn_to_prev(allNodes);

    println!("last node: {:?}", allNodes);
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
