use std::convert::TryFrom;
use liella::spv::Spv;
use liella::spirv::Spirv;
use liella::graph::SpirvGraph;
use inline_spirv::inline_spirv;

fn main() {
    let spv: &'static [u32] = inline_spirv!(r#"
        #version 450
        layout(location=0) in int pred;
        layout(location=0) out int ans;
        void main() {
            if (pred > 0) { ans = 0; } else { ans = 1; }
        }
    "#, vert, vulkan1_0);
    let spv = Spv::try_from(spv).unwrap();
    let spv = Spirv::try_from(spv).unwrap();
    let graph = SpirvGraph::try_from(&spv).unwrap();
    println!("{:#?}", graph);
}
