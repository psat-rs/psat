mod h;

pub use h::*;

pub trait Target {
    type Component;
    type Context;
    fn get_root(&mut self) -> &mut Self::Component;
    fn get_context(&mut self) -> &Self::Context;
    fn get_root_and_context(&mut self) -> (&mut Self::Component, &Self::Context);
}

pub trait Component<T: Target> {
    type Props;
    type State;
    fn create(&self, target: &T) -> T::Component;
    fn reconcile(&self,
                 context: &T::Context,
                 component: &mut T::Component,
                 props: &Self::Props,
                 children: &Vec<VNode<T>>/*,
                 state: &mut Self::State*/);
}

pub fn render<T: Target>(target: &mut T, node: VNode<T>) {
    let (root, context) = target.get_root_and_context();
    node.reconcile(context, root);
}
