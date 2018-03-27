extern crate psat;

struct SomeComponent {
    pub content: String;
    pub children: Vec<SomeComponent>;
}

struct SomeTarget {
    root: SomeComponent
}

impl psat::Target for SomeTarget {
    type Component = SomeComponent;
    fn get_root(&mut self) -> Self::Component {
        self.root
    }
}

struct SomeNode {}

impl psat::Node<SomeTarget> for SomeNode {
    type Props = ();
    type State = ();
    fn reconcile(_: &T,
                 component: SomeComponent,
                 props: &Self::Props,
                 children: &Vec<VNode>,
                 state: &mut Self::State) -> SomeComponent {
        component
    }
    fn create(_: &T) -> SomeComponent {
        SomeComponent {
            content: "content".to_owned(),
            children: vec![]
        }
    }
}
const SOME_NODE: SomeNode = SomeNode {};

fn main() {
    let node = psat::h(SOME_NODE, (), vec![]);
}
