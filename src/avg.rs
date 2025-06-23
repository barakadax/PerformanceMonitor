use std::rc::Rc;
use std::cell::RefCell;

pub struct Node {
    pub counter: u32,
    pub sum: u128,
    pub next: Option<Rc<RefCell<Node>>>,
}

pub struct LinkedList {
    pub head: Option<Rc<RefCell<Node>>>,
    pub cursor: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(counter: u32, sum: u128) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            counter,
            sum,
            next: None,
        }))
    }

    fn can_add_data(&self, value: u128) -> bool {
        self.sum.checked_add(value).is_some() && self.counter.checked_add(1).is_some()
    }

    pub(crate) fn add_data_in_place(&mut self, value: u128) {
        self.sum = self.sum.checked_add(value).unwrap();
        self.counter = self.counter.checked_add(1).unwrap();
    }
}

impl LinkedList {
    pub fn new(counter: u32, sum: u128) -> Self {
        let initial_node: Rc<RefCell<Node>> = Node::new(counter, sum);
        LinkedList {
            head: Some(Rc::clone(&initial_node)),
            cursor: Some(initial_node),
        }
    }

    pub fn add_data(&mut self, value: u128) {
        let cursor_rc: Rc<RefCell<Node>> = Rc::clone(self.cursor.as_ref().unwrap());
        {
            let cursor_node_borrow: std::cell::Ref<'_, Node> = cursor_rc.borrow();
            if cursor_node_borrow.can_add_data(value) {
                drop(cursor_node_borrow);
                let mut cursor_node_mut_borrow: std::cell::RefMut<'_, Node> = cursor_rc.borrow_mut();
                cursor_node_mut_borrow.add_data_in_place(value);
                return;
            }
        }

        self.push(1, value);
    }

    fn push(&mut self, counter: u32, sum: u128) {
        let new_node: Rc<RefCell<Node>> = Node::new(counter, sum);
        if let Some(old_cursor) = self.cursor.take() {
            old_cursor.borrow_mut().next = Some(Rc::clone(&new_node));
            self.cursor = Some(new_node);
        } else {
            self.head = Some(Rc::clone(&new_node));
            self.cursor = Some(new_node);
        }
    }

    pub fn average(&self) -> f64 {
        let mut pointer: Option<Rc<RefCell<Node>>> = self.head.as_ref().map(Rc::clone);
        let mut avg: f64 = 0.0;
        let mut count: u32 = 0;

        while let Some(node_rc) = pointer {
            let node: std::cell::Ref<'_, Node> = node_rc.borrow();
            if node.counter != 0 {
                avg += node.sum as f64 / node.counter as f64;
                count += 1;
            }
            pointer = node.next.as_ref().map(Rc::clone);
        }

        if count > 0 {
            avg / count as f64
        } else {
            0.0
        }
    }
}
