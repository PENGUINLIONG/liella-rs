use std::convert::TryFrom;
use liella::spv::Spv;
use liella::spirv::Spirv;
use liella::graph::SpirvGraph;
use inline_spirv::inline_spirv;

fn print_graphviz_py<'a>(graph: &SpirvGraph<'a>) {
    println!("from graphviz import Digraph");
    println!("g = Digraph()");
    for block in graph.subgraphs()[0].blocks() {
        let name = format!("{:?}", block.clone().downgrade());
        println!("g.node('{}')", name);
    }
    for edge in graph.subgraphs()[0].edges() {
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
            ans = 0;
            {
                for (int i = pred; i < 5; ++i) {
                    ans += pred;
                }
            }

            {
                int i = pred;
                while (i-- != 0) {
                    ans += pred;
                }
            }


            {
                int i = pred;
                while (true) {
                    ans += pred;
                }
            }

            {
                int i = pred;
                do {
                    ans += pred;
                } while (i-- != 0);
            }

            {
                int i = pred;
                for (;;) {
                    ans += pred;
                }
            }
        }
    "#, vert, vulkan1_0);
    let spv = Spv::try_from(spv).unwrap();
    let spv = Spirv::try_from(spv).unwrap();
    let graph = SpirvGraph::try_from(&spv).unwrap();
    print_graphviz_py(&graph);
}
