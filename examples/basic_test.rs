extern crate psat;

#[derive(Debug, PartialEq)]
struct SomeComponent {
    pub content: String,
    pub children: Vec<SomeComponent>
}

#[derive(Debug)]
struct SomeTarget {
    root: Option<SomeComponent>
}

impl psat::Target for SomeTarget {
    type Component = SomeComponent;
    type Context = ();
    fn get_context(&mut self) -> &Self::Context {
        &()
    }
    fn set_root(&mut self, new_root: Self::Component) {
        self.root = Some(new_root);
    }
}

impl<'a> psat::ChildAccess<'a, SomeComponent> for Vec<SomeComponent> {
    fn len(&self) -> usize {
        Vec::len(&self)
    }
    fn get_mut(&mut self, index: usize) -> Option<&mut SomeComponent> {
        <[SomeComponent]>::get_mut(self, index)
    }
    fn insert(&mut self, index: usize, item: SomeComponent) {
        self.insert(index, item);
    }
    fn relocate(&mut self, index: usize, i: usize) {
        if i == index { return; }
        let item = self.remove(i);
        self.insert(index, item);
    }
    fn cleanup(&mut self, index: usize) {
        while self.len() > index {
            self.remove(index);
        }
    }
}

struct SomeNode {}

impl psat::NativeComponent<SomeTarget> for SomeNode {
    type Props = &'static str;
    fn reconcile(&self,
                 context: &<SomeTarget as psat::Target>::Context,
                 component: &mut SomeComponent,
                 props: &Self::Props,
                 children: &Vec<psat::VNode<SomeTarget>>) {
        component.content = (*props).to_owned();
        psat::reconcile_children(context, children, &mut component.children);
    }
    fn create(&self, _: &<SomeTarget as psat::Target>::Context) -> SomeComponent {
        SomeComponent {
            content: "content".to_owned(),
            children: vec![]
        }
    }
}
const SOME_NODE: SomeNode = SomeNode {};

fn main() {
    let node = psat::h(SOME_NODE, "Hello, world!", vec![
                       psat::h(SOME_NODE, "child 1", vec![]),
                       psat::h(SOME_NODE, "child 2", vec![])
    ]);
    let mut target = SomeTarget {
        root: None
    };
    psat::render(&mut target, &node);
    println!("{:?}", target);
}
