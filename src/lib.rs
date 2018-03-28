mod h;

pub use h::*;

pub trait Target {
    type Component;
    type Context;
    fn get_root(&mut self) -> &mut Self::Component;
    fn get_context(&mut self) -> &Self::Context;
    fn get_root_and_context(&mut self) -> (&mut Self::Component, &Self::Context);
}

pub trait NativeComponent<T: Target> {
    type Props;
    fn create(&self, target: &T) -> T::Component;
    fn reconcile(&self,
                 context: &T::Context,
                 component: &mut T::Component,
                 props: &Self::Props,
                 children: &Vec<VNode<T>>);
}

pub fn render<T: Target>(target: &mut T, node: VNode<T>) {
    let (root, context) = target.get_root_and_context();
    render_component(context, node, root);
}

pub fn render_component<T: Target>(context: &T::Context, node: VNode<T>, component: &mut T::Component) {
    node.reconcile(context, component);
}
