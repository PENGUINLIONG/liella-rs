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

        BODY += [f'    {instr["opcode"]} => "{instr["opname"][2:]}",\n']

    BODY += match_dict["footer"]

    with open(out_path, "w") as f:
        f.writelines(BODY)

COLLECT_ID_REFS = {
    "header": [
        "use crate::spv::OpCode;\n",
        "\n",
        "pub(crate) fn opcode2name(opcode: OpCode) -> &'static str {\n",
        "  match opcode {\n",
    ],
    "footer": [
        '    _ => "Unknown",\n'
        "  }\n",
        "}\n",
    ],
}

gen_operand_proc("src/spirv/gen/opcode2name.rs", COLLECT_ID_REFS)
