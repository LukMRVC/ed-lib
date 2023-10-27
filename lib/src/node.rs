use crate::label::Label;
use std::cell::RefCell;
use std::rc::Rc;

type NodeLabel<T> = Box<dyn Label<LabelType = T>>;
type ChildrenType<T> = Vec<Rc<RefCell<T>>>;

pub type NodeType<'a, T> = Rc<RefCell<Node<'a, T>>>;

pub struct Node<'a, T> {
    label: NodeLabel<T>,
    children: ChildrenType<Self>,
}

impl<'a, T> Node<'a, T> {
    pub fn new(label: NodeLabel<T>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            label,
            children: Vec::new(),
        }))
    }

    pub fn get_children(&self) -> &ChildrenType<Self> {
        &self.children
    }

    pub fn get_children_mut(&mut self) -> &mut ChildrenType<Self> {
        &mut self.children
    }

    pub fn is_leaf(&self) -> bool {
        return self.children.is_empty();
    }

    pub fn get_label(&self) -> &NodeLabel<T> {
        return &self.label;
    }

    pub fn get_label_mut(&mut self) -> &mut NodeLabel<T> {
        return &mut self.label;
    }
}

impl<'a, T> From<NodeLabel<T>> for Node<'a, T> {
    fn from(value: NodeLabel<T>) -> Self {
        Self {
            label: value,
            children: Vec::new(),
        }
    }
}
