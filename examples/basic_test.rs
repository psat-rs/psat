extern crate psat;

#[derive(Debug)]
struct SomeComponent {
    pub content: String,
    pub children: Vec<SomeComponent>
}

#[derive(Debug)]
struct SomeTarget {
    root: SomeComponent
}

impl psat::Target for SomeTarget {
    type Component = SomeComponent;
    type Context = ();
    fn get_root(&mut self) -> &mut Self::Component {
        &mut self.root
    }
    fn get_context(&mut self) -> &Self::Context {
        &()
    }
    fn get_root_and_context(&mut self) -> (&mut Self::Component, &Self::Context) {
        (&mut self.root, &())
    }
}

struct SomeNode {}

impl psat::Component<SomeTarget> for SomeNode {
    type Props = &'static str;
    type State = ();
    fn reconcile(&self,
                 _: &<SomeTarget as psat::Target>::Context,
                 component: &mut SomeComponent,
                 props: &Self::Props,
                 children: &Vec<psat::VNode<SomeTarget>>/*,
                 state: &mut Self::State*/) {
        component.content = (*props).to_owned();
    }
    fn create(&self, _: &SomeTarget) -> SomeComponent {
        SomeComponent {
            content: "content".to_owned(),
            children: vec![]
        }
    }
}
const SOME_NODE: SomeNode = SomeNode {};

fn main() {
    let node = psat::h(SOME_NODE, "Hello, world!", vec![]);
    let mut target = SomeTarget {
        root: SomeComponent {
            content: "".to_owned(),
            children: vec![]
        }
    };
    psat::render(&mut target, node);
    println!("{:?}", target);
}
