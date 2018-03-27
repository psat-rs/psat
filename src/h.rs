use {Target, Node};

pub struct VNode<T: Target> {
    pub node: Box<Node<T>>,
    pub attributes: (),
    pub children: Vec<VNode<T>>
}

pub fn h<T: Target, N: 'static + Node<T>>(node: N, attributes: (), children: Vec<VNode<T>>) -> VNode<T> {
    VNode {
        node: Box::new(node),
        attributes,
        children
    }
}
