use std::{
    cell::{Ref, RefCell},
    cmp::min,
    rc::Rc,
};

pub struct Node {
    pub counter: u32,
    pub sum: u32,
    pub next: Option<Rc<RefCell<Node>>>,
}

pub struct LinkedList {
    pub head: Option<Rc<RefCell<Node>>>,
    pub cursor: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(counter: u32, sum: u32) -> impl Future<Output = Rc<RefCell<Node>>> + Send {
        async move {
            Rc::new(RefCell::new(Node {
                counter,
                sum,
                next: None,
            }))
        }
    }

    fn can_add_data(&self, value: u32) -> bool {
        self.sum.checked_add(value).is_some() && self.counter.checked_add(1).is_some()
    }

    pub(crate) fn add_data_in_place(&mut self, value: u32) {
        self.sum = self.sum.checked_add(value).unwrap();
        self.counter = self.counter.checked_add(1).unwrap();
    }
}

impl LinkedList {
    pub fn new() -> impl Future<Output = Self> + Send {
        async move {
            let initial_node: Rc<RefCell<Node>> = Node::new(0, 0).await;
            LinkedList {
                head: Some(Rc::clone(&initial_node)),
                cursor: Some(initial_node),
            }
        }
    }

    pub async fn add(&mut self, mut value: u128) {
        while value > 0 {
            let chunk: u32 = min(value, u32::MAX as u128) as u32;
            let cursor_rc: Rc<RefCell<Node>> = Rc::clone(self.cursor.as_ref().unwrap());
            {
                let cursor_node_borrow: Ref<'_, Node> = cursor_rc.borrow();
                if cursor_node_borrow.can_add_data(chunk) {
                    drop(cursor_node_borrow);
                    let mut cursor_node_mut_borrow: std::cell::RefMut<'_, Node> =
                        cursor_rc.borrow_mut();
                    cursor_node_mut_borrow.add_data_in_place(chunk);
                    value -= chunk as u128;
                    continue;
                }
            }
            self.push(1, chunk).await;
            value -= chunk as u128;
        }
    }

    async fn push(&mut self, counter: u32, sum: u32) {
        let new_node: Rc<RefCell<Node>> = Node::new(counter, sum).await;
        if let Some(old_cursor) = self.cursor.take() {
            old_cursor.borrow_mut().next = Some(Rc::clone(&new_node));
            self.cursor = Some(new_node);
        } else {
            self.head = Some(Rc::clone(&new_node));
            self.cursor = Some(new_node);
        }
    }

    pub fn average(&self) -> impl Future<Output = f64> + Send {
        let mut pointer: Option<Rc<RefCell<Node>>> = self.head.as_ref().map(Rc::clone);
        let mut avg: f64 = 0.0;
        let mut count: f64 = 0.0;

        while let Some(node_rc) = pointer {
            let node: std::cell::Ref<'_, Node> = node_rc.borrow();
            if node.counter != 0 {
                let node_avg: f64 = node.sum as f64 / node.counter as f64;
                avg = avg * (count / (count + 1.0)) + node_avg / (count + 1.0);
                count += 1.0;
            }
            pointer = node.next.as_ref().map(Rc::clone);
        }

        async move { if count > 0.0 { avg / count } else { 0.0 } }
    }
}
