use add_one;
use std::rc::Weak;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );

    let a = List::cons((1, List::cons((2, List::cons(3)))));
    println!("RC count = {}", Rc::strong_count(&a));
    let b = List::Cons(3, Rc::clone(&a));
    println!("RC count = {}", Rc::strong_count(&a));
    {
        let c = List::Cons(4, Rc::clone(&a));
        println!("RC count = {}", Rc::strong_count(&a));
    }
    println!("RC count = {}", Rc::strong_count(&a));

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}


#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}



enum List {
    Cons(i32, Rc<List>),
    Nil,
}

trait IntoList {
    fn to_list(self) -> Rc<List>;
}

impl IntoList for i32 {
    fn to_list(self) -> Rc<List> {
        Rc::new(List::Cons(self, Rc::new(List::Nil)))
    }
}

impl IntoList for (i32, Rc<List>) {
    fn to_list(self) -> Rc<List> {
        Rc::new(List::Cons(self.0, self.1))
    }
}

impl IntoList for (i32, &Rc<List>) {
    fn to_list(self) -> Rc<List> {
        Rc::new(List::Cons(self.0, Rc::clone(self.1)))
    }
}

impl List {
    fn cons<T>(v: T) -> Rc<List>
    where
        T: IntoList,
    {
        v.to_list()
    }
}
