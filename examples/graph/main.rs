use std::convert::TryFrom;
use liella::spv::Spv;
use liella::spirv::{Context, Node, Visitor, spv2graph, elevate_blocks, visit};
use inline_spirv::inline_spirv;
#[cfg(not(release))]
use liella::test_utils::dump_spv;

#[derive(Default)]
struct DebugVisitor();
impl Visitor for DebugVisitor {
    fn visit(&mut self, node: &mut Node) {
        println!("{:#?}", node);
    }
}

fn main() {
    let spv: &'static [u32] = inline_spirv!(r#"
        #version 450
        layout(location=0) in int pred;
        layout(location=0) out int ans;
        void main() {
            int y = 12345;
            {
                for (int i = pred; i < 5; ++i) {
                    for (int j = pred; j < 7; ++j) {
                        y += 1;
                    }
                    y += 2;
                }
            }
            ans = y;
        }
    "#, vert, vulkan1_0, no_debug);
    if cfg!(not(release)) {
        dump_spv("graph.spv", spv);
    }

    let spv = Spv::try_from(spv).unwrap();
    let mut ctxt = Context::new();
    let root = spv2graph(&mut ctxt, spv);
    elevate_blocks(&mut ctxt, root.clone());
    visit(root, &mut DebugVisitor::default());
}
