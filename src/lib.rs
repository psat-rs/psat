mod h;

pub use h::*;

pub trait Target {
    type Component;
    type Context;
    fn get_context(&mut self) -> &Self::Context;
    fn set_root(&mut self, root: Self::Component);
}

pub trait NativeComponent<T: Target> {
    type Props;
    fn create(&self, context: &T::Context) -> T::Component;
    fn reconcile(&self,
                 context: &T::Context,
                 component: &mut T::Component,
                 props: &Self::Props,
                 children: &Vec<VNode<T>>);
}

pub fn render<T: Target>(target: &mut T, node: &VNode<T>) {
    let rendered = render_component(target.get_context(), node, None);
    target.set_root(rendered.unwrap());
}

pub fn render_component<T: Target>(context: &T::Context, node: &VNode<T>, mut component: Option<&mut T::Component>) -> Option<T::Component> {
    match component {
        Some(ref mut x) => {
            node.reconcile(context, &mut *x);
            None
        },
        None => {
            let mut x = node.create(context);
            node.reconcile(context, &mut x);
            Some(x)
        }
    }
}

pub trait ChildAccess<'a, C: 'a> {
    fn len(&self) -> usize;
    fn get_mut(&mut self, index: usize) -> Option<&mut C>;
    fn insert(&mut self, index: usize, item: C);
    fn relocate(&mut self, to: usize, from: usize);
    fn cleanup(&mut self, index: usize);
}

pub fn reconcile_children<'a, T: 'a + Target, A: ChildAccess<'a, T::Component>>(
    context: &T::Context,
    children: &Vec<VNode<T>>,
    access: &mut A) {
    let mut index = 0;
    for child in children {
        let result = render_component(context, child, access.get_mut(index));
        match result {
            Some(child) => access.insert(index, child),
            None => access.relocate(index, index)
        }
        index += 1;
    }
    access.cleanup(index);
}
