use std::convert::TryFrom;
use liella::spv::Spv;
use liella::spirv::Spirv;
use liella::graph::Graph;
use inline_spirv::inline_spirv;
#[cfg(not(release))]
use liella::test_utils::dump_spv;

fn main() {
    let spv: &'static [u32] = inline_spirv!(r#"
        #version 450
        layout(location=0) in int pred;
        layout(location=0) out int ans;
        void main() {
            int y = 12345;
            {
                for (int i = pred; i < 5; ++i) {
                    y += 1;
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
    let spv = Spirv::try_from(spv).unwrap();
    let graph = Graph::try_from(&spv).unwrap();
    println!("{:#?}", graph.nodes());
}
