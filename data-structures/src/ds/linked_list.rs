#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    pub fn new(value:T) -> Self {
        Self { value, next: None }
    }
}

#[derive(Debug)]
pub struct List<T> {
    pub head: Option<Box<Node<T>>>,
    pub size: usize
}

impl<T> List<T> where T: PartialEq + Clone {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0
        }
    }

    fn create_node(value: T) -> Option<Box<Node<T>>> {
        Some(Box::new(Node::new(value)))
    }

    fn remove_node(node: &mut Box<Node<T>>) -> Option<Box<Node<T>>> {
        let next = node.next.take();

        next
    }

    pub fn remove_node_by_value(&mut self, value: T) -> bool {
        if self.size == 0 {
            return false;
        }

        let mut iter = &mut self.head;

        if let Some(node) = iter {
            if node.value == value {
                self.head = List::remove_node(node);
                self.size -= 1;
                return true;
            }
        }

        loop {
            if let Some(node) = iter {
                match &mut node.next {
                    Some(next) => {
                        if next.value == value {
                            node.next = List::remove_node(next);
                            self.size -= 1;
                            return true;
                        }
                    },
                    None => {
                        return false
                    }
                }
                iter = &mut node.next;
            }
        }
    }

    pub fn remove_node_by_index(&mut self, idx: usize) -> Result<T, &str> {

        if self.size == 0 {
            return Err("List does not have any alement to remove");
        }

        if idx >= self.size {
            return Err("Invalid index");
        }

        
        if idx == 0 {
            let mut removed_node = self.head.take();
            if let Some(node) = &mut removed_node {
                self.head = List::remove_node(node);
                let removed_value = node.value.clone();
                self.size -= 1;
                return Ok(removed_value);
            } else {
                return Err("internal error")
            }
        }

        let mut iter = &mut self.head;
        
        let mut idx_counter: usize = 0;

        loop {
            match iter {
                Some(node) => {

                    if idx_counter == idx - 1 {
                        let mut old_node = std::mem::take(&mut node.next);
                        if let Some(next) = &mut old_node {
                            let removed_value = next.value.clone();
                            node.next = List::remove_node(next);
                            self.size -= 1;
                            return Ok(removed_value);
                        }
                    }

                    idx_counter += 1;
                    iter = &mut node.next;
                },
                None => {
                    return Err("Internal error")
                }
            }
        }
    }

    pub fn add(&mut self, value: T) {
        if let None = self.head {
            self.head = List::create_node(value);
            self.size += 1;
            return;
        }

        let mut iter = &mut self.head;

        loop {
            if let Some(node) = iter {
                match &node.next {
                    Some(_) => {},
                    None => {
                        node.next = List::create_node(value);
                        self.size += 1;
                        break;
                    }
                }
                iter = &mut node.next;
            } else {
                break;
            }
        }

    }
}