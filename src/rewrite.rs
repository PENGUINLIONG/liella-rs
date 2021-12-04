use std::collections::hash_map::{HashMap, Entry};
use std::collections::hash_set::HashSet;
use crate::graph::{Block, BlockRef, Graph, GraphEdge,
    GraphLoop};
use crate::spirv::{OpCode, Instruction, InstructionRef, Operand, Spirv};

const OP_LOAD: u32 = 61;
const OP_STORE: u32 = 62;
const OP_VARIABLE: u32 = 59;

const STORAGE_CLASS_FUNCTION: u32 = 7;

fn is_fn_var(instr: InstructionRef) -> bool {
    let instr = instr.upgrade().unwrap();
    match instr.opcode() {
        OP_LOAD => {
            // To prevent function-scoped `OpVariable` behind `OpLoad` to be
            // treated as function variable.
            false
        },
        OP_VARIABLE => {
            let storage_cls = instr.operands()[2].as_lit().unwrap();
            storage_cls == STORAGE_CLASS_FUNCTION
        },
        _ => {
            instr.operands().iter()
                .filter_map(Operand::as_instr)
                .any(is_fn_var)
        }
    }
}
fn match_simple_load_instr(instr: &Instruction) -> Option<InstructionRef> {
    if instr.opcode() != OP_LOAD { return None; }
    let operands = instr.operands();
    if operands.len() != 3 { return None; }
    let var_expr = operands[2].as_instr().unwrap();
    if !is_fn_var(var_expr.clone()) { return None; }
    Some(var_expr)
}
fn match_simple_store_instr(instr: &Instruction) -> Option<(InstructionRef, InstructionRef)> {
    if instr.opcode() != OP_STORE { return None; }
    let operands = instr.operands();
    if operands.len() != 2 { return None; }
    let var_expr = operands[0].as_instr().unwrap();
    if !is_fn_var(var_expr.clone()) { return None; }
    let value_expr = operands[1].as_instr().unwrap();
    Some((var_expr, value_expr))
}

struct Rewriter {
    // Mapping from variable to value expression.
    var_states: HashMap<Instruction, Instruction>,
    // Mapping from instructions to unfolded value expressions. This is used to
    // correctly transition operands in multiple instructions referring to a
    // same load instruction.
    instr_map: HashMap<Instruction, Instruction>,
    stmts: Vec<InstructionRef>,
}
impl Rewriter {
    fn new() -> Self {
        Rewriter {
            var_states: HashMap::new(),
            instr_map: HashMap::new(),
            stmts: Vec::new(),
        }
    }
    fn rewrite_impl(&mut self, instr: &Instruction) {
        // An operand has been rewritten so the instruction referring to it must
        // be rewritten too.
        let mut any_rewrite = false;
        let out_operands = instr.operands().iter()
            .map(|operand| {
                match operand {
                    Operand::Instruction(instr) => {
                        let instr = instr.clone().upgrade().unwrap();
                        self.rewrite_impl(&instr);
                        let rewrite = self.instr_map.get(&instr).unwrap();
                        if rewrite != &instr {
                            any_rewrite = true;
                        }
                        Operand::Instruction(rewrite.downgrade())
                    },
                    Operand::Literal(lit) => Operand::Literal(*lit),
                    Operand::ResultPlaceholder => Operand::ResultPlaceholder,
                }
            })
            .collect::<Vec<_>>();

        let out_instr = if any_rewrite {
            Instruction::new(instr.opcode(), out_operands)
        } else {
            instr.clone()
        };

        if let Some((var, value)) = match_simple_store_instr(&out_instr) {
            // Track states of function variables instead of making store
            // instructions for them.
            let var = &var.upgrade().unwrap();
            let value = value.upgrade().unwrap();
            println!("store {:?} -> {:?}", var, value);
            self.var_states.insert(var.clone(), value.clone());
        } else if let Some(var) = match_simple_load_instr(&out_instr) {
            let var = &var.upgrade().unwrap();
            let value = self.var_states.get(&var).unwrap();
            println!("load {:?} -> {:?}", var, value);
            self.instr_map.insert(instr.clone(), value.clone());
        } else {
            self.instr_map.insert(instr.clone(), out_instr);
        }
    }
    fn rewrite(&mut self, instr: &Instruction) {
        self.rewrite_impl(instr);
        if let Some(rewrite) = self.instr_map.get(&instr) {
            self.stmts.push(rewrite.downgrade());
        }
    }
}

pub fn rewrite_spirv(spv: &Spirv) -> Spirv {
    let header = spv.header().clone();
    let mut rewriter = Rewriter::new();
    for stmt in spv.stmts().iter() {
        rewriter.rewrite(&stmt.upgrade().unwrap());
    }
    let instr_pool = rewriter.instr_map.into_iter()
        .map(|(_instr, rewrite)| rewrite)
        .collect();
    let stmts = rewriter.stmts;
    Spirv::new(header, instr_pool, stmts)
}

#[cfg(test)]
mod tests {
    use super::*;
    const OP_TYPE_FLOAT: u32 = 22;
    const OP_TYPE_POINTER: u32 = 32;
    const OP_CONSTANT: u32 = 43;
    const OP_FADD: u32 = 129;

    //#[test]
    //fn test_rewrite() {
    //    let float = Instruction::new(OP_TYPE_FLOAT, vec![
    //        Operand::ResultPlaceholder,
    //        Operand::Literal(32),
    //    ]);
    //    let pfloat = Instruction::new(OP_TYPE_POINTER, vec![
    //        Operand::ResultPlaceholder,
    //        Operand::Literal(STORAGE_CLASS_FUNCTION),
    //        Operand::Instruction(float.downgrade()),
    //    ]);
    //    let var0 = Instruction::new(OP_VARIABLE, vec![
    //        Operand::Instruction(pfloat.downgrade()),
    //        Operand::ResultPlaceholder,
    //        Operand::Literal(STORAGE_CLASS_FUNCTION),
    //    ]);
    //    let const0 = Instruction::new(OP_CONSTANT, vec![
    //        Operand::Instruction(float.downgrade()),
    //        Operand::ResultPlaceholder,
    //        Operand::Literal(0),
    //    ]);
    //    let load_var0 = Instruction::new(OP_LOAD, vec![
    //        Operand::Instruction(float.downgrade()),
    //        Operand::ResultPlaceholder,
    //        Operand::Instruction(var0.downgrade()),
    //    ]);
    //    let fadd0_load_var0 = Instruction::new(OP_FADD, vec![
    //        Operand::Instruction(float.downgrade()),
    //        Operand::ResultPlaceholder,
    //        Operand::Instruction(load_var0.downgrade()),
    //        Operand::Instruction(const0.downgrade()),
    //    ]);
    //    let store_fadd0_load_var0 = Instruction::new(OP_STORE, vec![
    //        Operand::Instruction(var0.downgrade()),
    //        Operand::Instruction(fadd0_load_var0.downgrade()),
    //    ]);
    //    let instrs = vec![
    //        float,
    //        var0,
    //        const0,
    //        load_var0.clone(),
    //        fadd0_load_var0.clone(),
    //        store_fadd0_load_var0.clone(),
    //        load_var0,
    //        fadd0_load_var0,
    //        store_fadd0_load_var0,
    //    ];
    //    let rewrites = rewrite_instrs(&instrs);
    //    assert_eq!(rewrites.len(), 6);
    //}
}
