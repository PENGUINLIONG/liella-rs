import json
from typing import Match

SPEC = None
with open("third/SPIRV-Headers/include/spirv/unified1/spirv.core.grammar.json") as f:
    SPEC = json.load(f)

KIND2CATEGORY = {}
for operand_kind in SPEC["operand_kinds"]:
    kind = operand_kind["kind"]
    category = operand_kind["category"]
    KIND2CATEGORY[kind] = category

class EnforceLast:
    """
    A marker object to annotate note the generator that the operand should be
    the last one. Any extra operand should trigger an error.
    """
    pass
    def __repr__():
        return ""

def gen_operand_proc(out_path, match_dict):
    FOUND_INSTR = set()

    BODY = [
        "//! THIS IS A GENERATED SOURCE. MODIFICATION WILL BE OVERWRITTEN.\n",
        "//! @PENGUINLIONG\n",
    ]
    BODY += match_dict["header"]

    for instr in SPEC['instructions']:
        if instr['class'] == "@reserved":
            continue

        opcode = instr["opcode"]
        opname = instr["opname"]
        if opcode in FOUND_INSTR:
            BODY += [f"    // Ignored alias op {opname}.\n"]
            continue
        FOUND_INSTR.add(opcode)

        if 'operands' not in instr:
            instr['operands'] = []

        operands = instr["operands"]

        BODY += [f"    {instr['opcode']} => {{ // {instr['opname'][2:]}\n"]

        uncertain_size = False
        body = []
        for operand in instr['operands']:
            if uncertain_size:
                raise RuntimeError("unexpected params after a context-dependent param")
            body += ["      "]

            if "quantifier" in operand:
                quantifier = operand["quantifier"]
                if quantifier == '*':
                    body += ["while !it.ate() {\n"]
                elif quantifier == '?':
                    body += ["if !it.ate() {\n"]

            kind = operand['kind']
            category = KIND2CATEGORY[kind]

            if kind in match_dict["kind"]:
                seg = match_dict["kind"][kind]
            elif category in match_dict["category"]:
                seg = match_dict["category"][category]
            else:
                raise RuntimeError("unexpected operand category/kind")

            uncertain_size = uncertain_size or any(type(x) == EnforceLast for x in seg)
            seg = [x for x in seg if type(x) is str]

            if "quantifier" in operand:
                body += ["        "]
                body += seg
                body += ["      }\n"]
            else:
                body += seg

        body += ["    },\n"]

        BODY += body

    BODY += match_dict["footer"]

    with open(out_path, "w") as f:
        f.writelines(BODY)

COLLECT_ID_REFS = {
    "header": [
        "use crate::spirv::{Instruction, InstructionBuilder, SpirvDeserializer};\n",
        "use crate::spv::{SpvId, Instr};\n",
        "\n",
        "pub(crate) fn deserialize_instr(\n",
        "  ctxt: &mut SpirvDeserializer,\n"
        "  instr: &Instr,\n",
        ") -> Option<(SpvId, Instruction)> {\n",
        "  let mut it = instr.operands();\n",
        "  let mut ib = InstructionBuilder::new(ctxt, instr.opcode());\n",
        "  let mut result_id: u32 = 0;\n",
        "  match instr.opcode() {\n",
    ],
    "footer": [
        "    _ => return None,\n"
        "  }\n",
        "  while let Some(w) = it.next() { ib.lit(w); }\n"
        "  ib.build().map(|x| (result_id, x))\n",
        "}\n",
    ],
    "kind": {
        "IdResult": ["result_id = it.id(); ib.res();\n"],
        "LiteralString": [
            "while let Some(w) = it.next() { ib.lit(w); if (w >> 24) == 0 { break; } }\n"
        ],
        "PairLiteralIntegerIdRef": [
            "{ ",
            "ib.lit(it.u32()); ",
            "ib.id(it.id()); ",
            "}\n",
        ],
        "PairIdRefLiteralInteger": [
            "{ ",
            "ib.id(it.id()); ",
            "ib.lit(it.u32()); ",
            "}\n",
        ],
        "PairIdRefIdRef": [
            "{ ",
            "ib.id(it.id()); ",
            "ib.id(it.id()); ",
            "}\n",
        ],
        "LiteralContextDependentNumber": [
            "while !it.ate() { ib.lit(it.u32()); }\n",
            EnforceLast()
        ],
    },
    "category": {
        "BitEnum": ["ib.lit(it.u32());\n"],
        "ValueEnum": ["ib.lit(it.u32());\n"],
        "Id": ["ib.id(it.id());\n"],
        "Literal": ["ib.lit(it.u32());\n"],
    },
}

gen_operand_proc("src/spirv/gen/deserialize_instr.rs", COLLECT_ID_REFS)
