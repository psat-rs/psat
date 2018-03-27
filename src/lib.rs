mod h;

pub use h::*;

pub trait Target {
    type Component;
    fn get_root(&mut self) -> Self::Component;
}

pub trait Node<T: Target> {
    type Props;
    type State;
    fn create(target: &T) -> T::Component;
    fn reconcile(target: &T,
                 component: T::Component,
                 props: &Self::Props,
                 children: &Vec<VNode<T>>,
                 state: &mut Self::State) -> T::Component;
}

pub fn render<T: Target, N: Node<Target>>(target: &mut T, node: N) {
    let root = target.get_root();

}
