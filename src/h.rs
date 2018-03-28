use {Target, NativeComponent};

pub struct ComponentNode<T: Target, C: NativeComponent<T>> {
    pub component: C,
    pub attributes: C::Props,
}

pub trait ComponentNodeTrait<T: Target> {
    fn reconcile(&self, context: &T::Context, component: &mut T::Component, children: &Vec<VNode<T>>);
    fn create(&self, context: &T::Context) -> T::Component;
}

impl<T: Target, C: NativeComponent<T>> ComponentNodeTrait<T> for ComponentNode<T, C> {
    fn reconcile(&self, context: &T::Context, component: &mut T::Component, children: &Vec<VNode<T>>) {
        self.component.reconcile(context, component, &self.attributes, children)
    }
    fn create(&self, context: &T::Context) -> T::Component {
        self.component.create(context)
    }
}

pub struct VNode<T: Target> {
    pub component: Box<ComponentNodeTrait<T>>,
    pub children: Vec<VNode<T>>
}

impl<T: Target> VNode<T> {
    pub fn reconcile(&self, context: &T::Context, component: &mut T::Component) {
        self.component.reconcile(context, component, &self.children)
    }
    pub fn create(&self, context: &T::Context) -> T::Component {
        self.component.create(context)
    }
}

pub fn h<T: 'static + Target, N: 'static + NativeComponent<T>>(node: N, attributes: N::Props, children: Vec<VNode<T>>) -> VNode<T> {
    VNode {
        component: Box::new(ComponentNode {
            component: node,
            attributes
        }),
        children
    }
}
