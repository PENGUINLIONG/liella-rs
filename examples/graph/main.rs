use std::convert::TryFrom;
use liella::spv::Spv;
use liella::spirv::Spirv;
use liella::graph::Graph;
use inline_spirv::inline_spirv;
#[cfg(not(release))]
use liella::test_utils::dump_spv;
use liella::rewrite;

fn print_graphviz_py<'a>(graph: &Graph) {
    println!("from graphviz import Digraph");
    println!("g = Digraph()");
    for block in graph.blocks() {
        let name = format!("{:?}", block.clone().downgrade());
        println!("g.node('{}')", name);
    }
    for edge in graph.edges() {
        let src = format!("{:?}", edge.src);
        let dst = format!("{:?}", edge.dst);
        println!("g.edge('{}', '{}')", src, dst);
    }
    println!("g.view()");
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
    let rewrite_spv = rewrite::rewrite_spirv(&spv);
    println!("------");
    println!("{:#?}", spv);
    println!("------");
    println!("{:#?}", rewrite_spv);
    let graph = Graph::try_from(&spv).unwrap();
    print_graphviz_py(&graph);
    println!("\"\"\"\n{:#?}\n\"\"\"", graph);
}
