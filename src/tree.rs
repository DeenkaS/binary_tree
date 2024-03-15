use core::panic;

pub struct Tree {
    value: Option<i32>,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

impl Tree {
    pub fn new() -> Tree {
        // This is a constructor
        Tree {
            value: None,
            left: None,
            right: None,
        }
    }

    pub fn add(&mut self, input: i32) {
        match self.value {
            None => {
                self.value = Some(input);
                println!("Root: {:?}", self.value)
            }
            Some(value) if input > value => {
                self.right = Some(Box::new(Tree::new()));
                self.right.as_mut().unwrap().value = Some(input);
                println!("Right: {:?}", self.right.as_ref().unwrap().value)
            }
            Some(value) if input < value => {
                self.left = Some(Box::new(Tree::new()));
                self.left.as_mut().unwrap().value = Some(input);
                println!("Left: {:?}", self.left.as_ref().unwrap().value)
            }
            _ => {
                panic!("Duplicated value");
            }
        }
    }
}
