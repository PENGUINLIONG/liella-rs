//! THIS IS A GENERATED SOURCE. MODIFICATION WILL BE OVERWRITTEN.
//! @PENGUINLIONG
use crate::spirv::{Instruction, InstructionBuilder, SpirvDeserializer};
use crate::spv::{SpvId, Instr};
use crate::error::{LiellaError as Error, LiellaResult as Result};
pub(crate) fn deserialize_instr(
  ctxt: &mut SpirvDeserializer,
  instr: &Instr,
) -> Result<Option<(SpvId, Instruction)>> {
  let mut it = instr.operands();
  let mut ib = InstructionBuilder::new(ctxt, instr.opcode());
  let mut result_id: u32 = 0;
  match instr.opcode() {
    0 => { // Nop
    },
    1 => { // Undef
      ib.id(it.id());
      result_id = it.id(); ib.res();
    },
    2 => { // SourceContinued
      while let Some(w) = it.next() { ib.lit(w); if (w >> 24) == 0 { break; } }
    },
    3 => { // Source
      ib.lit(it.u32());
      ib.lit(it.u32());
      if !it.ate() {
        ib.id(it.id());
      }
      if !it.ate() {
        while let Some(w) = it.next() { ib.lit(w); if (w >> 24) == 0 { break; } }
      }
    },
    4 => { // SourceExtension
      while let Some(w) = it.next() { ib.lit(w); if (w >> 24) == 0 { break; } }
    },
    5 => { // Name
      ib.id(it.id());
      while let Some(w) = it.next() { ib.lit(w); if (w >> 24) == 0 { break; } }
    },
    6 => { // MemberName
      ib.id(it.id());
      ib.lit(it.u32());
      while let Some(w) = it.next() { ib.lit(w); if (w >> 24) == 0 { break; } }
    },
    7 => { // String
      result_id = it.id(); ib.res();
      while let Some(w) = it.next() { ib.lit(w); if (w >> 24) == 0 { break; } }
    },
    8 => { // Line
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    10 => { // Extension
      while let Some(w) = it.next() { ib.lit(w); if (w >> 24) == 0 { break; } }
    },
    11 => { // ExtInstImport
      result_id = it.id(); ib.res();
      while let Some(w) = it.next() { ib.lit(w); if (w >> 24) == 0 { break; } }
    },
    12 => { // ExtInst
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      while !it.ate() {
        ib.id(it.id());
      }
    },
    14 => { // MemoryModel
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    15 => { // EntryPoint
      ib.lit(it.u32());
      ib.id(it.id());
      while let Some(w) = it.next() { ib.lit(w); if (w >> 24) == 0 { break; } }
      while !it.ate() {
        ib.id(it.id());
      }
    },
    16 => { // ExecutionMode
      ib.id(it.id());
      ib.lit(it.u32());
    },
    17 => { // Capability
      ib.lit(it.u32());
    },
    19 => { // TypeVoid
      result_id = it.id(); ib.res();
    },
    20 => { // TypeBool
      result_id = it.id(); ib.res();
    },
    21 => { // TypeInt
      result_id = it.id(); ib.res();
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    22 => { // TypeFloat
      result_id = it.id(); ib.res();
      ib.lit(it.u32());
    },
    23 => { // TypeVector
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
    },
    24 => { // TypeMatrix
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
    },
    25 => { // TypeImage
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    26 => { // TypeSampler
      result_id = it.id(); ib.res();
    },
    27 => { // TypeSampledImage
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    28 => { // TypeArray
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    29 => { // TypeRuntimeArray
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    30 => { // TypeStruct
      result_id = it.id(); ib.res();
      while !it.ate() {
        ib.id(it.id());
      }
    },
    31 => { // TypeOpaque
      result_id = it.id(); ib.res();
      while let Some(w) = it.next() { ib.lit(w); if (w >> 24) == 0 { break; } }
    },
    32 => { // TypePointer
      result_id = it.id(); ib.res();
      ib.lit(it.u32());
      ib.id(it.id());
    },
    33 => { // TypeFunction
      result_id = it.id(); ib.res();
      ib.id(it.id());
      while !it.ate() {
        ib.id(it.id());
      }
    },
    34 => { // TypeEvent
      result_id = it.id(); ib.res();
    },
    35 => { // TypeDeviceEvent
      result_id = it.id(); ib.res();
    },
    36 => { // TypeReserveId
      result_id = it.id(); ib.res();
    },
    37 => { // TypeQueue
      result_id = it.id(); ib.res();
    },
    38 => { // TypePipe
      result_id = it.id(); ib.res();
      ib.lit(it.u32());
    },
    39 => { // TypeForwardPointer
      ib.id(it.id());
      ib.lit(it.u32());
    },
    41 => { // ConstantTrue
      ib.id(it.id());
      result_id = it.id(); ib.res();
    },
    42 => { // ConstantFalse
      ib.id(it.id());
      result_id = it.id(); ib.res();
    },
    43 => { // Constant
      ib.id(it.id());
      result_id = it.id(); ib.res();
      while !it.ate() { ib.lit(it.u32()); }
    },
    44 => { // ConstantComposite
      ib.id(it.id());
      result_id = it.id(); ib.res();
      while !it.ate() {
        ib.id(it.id());
      }
    },
    45 => { // ConstantSampler
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    46 => { // ConstantNull
      ib.id(it.id());
      result_id = it.id(); ib.res();
    },
    48 => { // SpecConstantTrue
      ib.id(it.id());
      result_id = it.id(); ib.res();
    },
    49 => { // SpecConstantFalse
      ib.id(it.id());
      result_id = it.id(); ib.res();
    },
    50 => { // SpecConstant
      ib.id(it.id());
      result_id = it.id(); ib.res();
      while !it.ate() { ib.lit(it.u32()); }
    },
    51 => { // SpecConstantComposite
      ib.id(it.id());
      result_id = it.id(); ib.res();
      while !it.ate() {
        ib.id(it.id());
      }
    },
    52 => { // SpecConstantOp
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.lit(it.u32());
    },
    54 => { // Function
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.lit(it.u32());
      ib.id(it.id());
    },
    55 => { // FunctionParameter
      ib.id(it.id());
      result_id = it.id(); ib.res();
    },
    56 => { // FunctionEnd
    },
    57 => { // FunctionCall
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      while !it.ate() {
        ib.id(it.id());
      }
    },
    59 => { // Variable
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.lit(it.u32());
      if !it.ate() {
        ib.id(it.id());
      }
    },
    60 => { // ImageTexelPointer
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    61 => { // Load
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    62 => { // Store
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    63 => { // CopyMemory
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    64 => { // CopyMemorySized
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    65 => { // AccessChain
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      while !it.ate() {
        ib.id(it.id());
      }
    },
    66 => { // InBoundsAccessChain
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      while !it.ate() {
        ib.id(it.id());
      }
    },
    67 => { // PtrAccessChain
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      while !it.ate() {
        ib.id(it.id());
      }
    },
    68 => { // ArrayLength
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
    },
    69 => { // GenericPtrMemSemantics
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    70 => { // InBoundsPtrAccessChain
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      while !it.ate() {
        ib.id(it.id());
      }
    },
    71 => { // Decorate
      ib.id(it.id());
      ib.lit(it.u32());
    },
    72 => { // MemberDecorate
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    73 => { // DecorationGroup
      result_id = it.id(); ib.res();
    },
    74 => { // GroupDecorate
      ib.id(it.id());
      while !it.ate() {
        ib.id(it.id());
      }
    },
    75 => { // GroupMemberDecorate
      ib.id(it.id());
      while !it.ate() {
        { ib.id(it.id()); ib.lit(it.u32()); }
      }
    },
    77 => { // VectorExtractDynamic
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    78 => { // VectorInsertDynamic
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    79 => { // VectorShuffle
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      while !it.ate() {
        ib.lit(it.u32());
      }
    },
    80 => { // CompositeConstruct
      ib.id(it.id());
      result_id = it.id(); ib.res();
      while !it.ate() {
        ib.id(it.id());
      }
    },
    81 => { // CompositeExtract
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      while !it.ate() {
        ib.lit(it.u32());
      }
    },
    82 => { // CompositeInsert
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      while !it.ate() {
        ib.lit(it.u32());
      }
    },
    83 => { // CopyObject
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    84 => { // Transpose
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    86 => { // SampledImage
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    87 => { // ImageSampleImplicitLod
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    88 => { // ImageSampleExplicitLod
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.lit(it.u32());
    },
    89 => { // ImageSampleDrefImplicitLod
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    90 => { // ImageSampleDrefExplicitLod
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.lit(it.u32());
    },
    91 => { // ImageSampleProjImplicitLod
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    92 => { // ImageSampleProjExplicitLod
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.lit(it.u32());
    },
    93 => { // ImageSampleProjDrefImplicitLod
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    94 => { // ImageSampleProjDrefExplicitLod
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.lit(it.u32());
    },
    95 => { // ImageFetch
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    96 => { // ImageGather
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    97 => { // ImageDrefGather
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    98 => { // ImageRead
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    99 => { // ImageWrite
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    100 => { // Image
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    101 => { // ImageQueryFormat
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    102 => { // ImageQueryOrder
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    103 => { // ImageQuerySizeLod
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    104 => { // ImageQuerySize
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    105 => { // ImageQueryLod
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    106 => { // ImageQueryLevels
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    107 => { // ImageQuerySamples
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    109 => { // ConvertFToU
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    110 => { // ConvertFToS
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    111 => { // ConvertSToF
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    112 => { // ConvertUToF
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    113 => { // UConvert
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    114 => { // SConvert
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    115 => { // FConvert
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    116 => { // QuantizeToF16
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    117 => { // ConvertPtrToU
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    118 => { // SatConvertSToU
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    119 => { // SatConvertUToS
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    120 => { // ConvertUToPtr
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    121 => { // PtrCastToGeneric
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    122 => { // GenericCastToPtr
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    123 => { // GenericCastToPtrExplicit
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
    },
    124 => { // Bitcast
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    126 => { // SNegate
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    127 => { // FNegate
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    128 => { // IAdd
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    129 => { // FAdd
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    130 => { // ISub
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    131 => { // FSub
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    132 => { // IMul
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    133 => { // FMul
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    134 => { // UDiv
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    135 => { // SDiv
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    136 => { // FDiv
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    137 => { // UMod
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    138 => { // SRem
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    139 => { // SMod
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    140 => { // FRem
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    141 => { // FMod
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    142 => { // VectorTimesScalar
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    143 => { // MatrixTimesScalar
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    144 => { // VectorTimesMatrix
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    145 => { // MatrixTimesVector
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    146 => { // MatrixTimesMatrix
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    147 => { // OuterProduct
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    148 => { // Dot
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    149 => { // IAddCarry
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    150 => { // ISubBorrow
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    151 => { // UMulExtended
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    152 => { // SMulExtended
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    154 => { // Any
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    155 => { // All
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    156 => { // IsNan
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    157 => { // IsInf
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    158 => { // IsFinite
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    159 => { // IsNormal
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    160 => { // SignBitSet
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    161 => { // LessOrGreater
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    162 => { // Ordered
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    163 => { // Unordered
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    164 => { // LogicalEqual
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    165 => { // LogicalNotEqual
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    166 => { // LogicalOr
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    167 => { // LogicalAnd
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    168 => { // LogicalNot
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    169 => { // Select
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    170 => { // IEqual
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    171 => { // INotEqual
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    172 => { // UGreaterThan
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    173 => { // SGreaterThan
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    174 => { // UGreaterThanEqual
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    175 => { // SGreaterThanEqual
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    176 => { // ULessThan
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    177 => { // SLessThan
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    178 => { // ULessThanEqual
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    179 => { // SLessThanEqual
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    180 => { // FOrdEqual
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    181 => { // FUnordEqual
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    182 => { // FOrdNotEqual
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    183 => { // FUnordNotEqual
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    184 => { // FOrdLessThan
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    185 => { // FUnordLessThan
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    186 => { // FOrdGreaterThan
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    187 => { // FUnordGreaterThan
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    188 => { // FOrdLessThanEqual
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    189 => { // FUnordLessThanEqual
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    190 => { // FOrdGreaterThanEqual
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    191 => { // FUnordGreaterThanEqual
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    194 => { // ShiftRightLogical
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    195 => { // ShiftRightArithmetic
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    196 => { // ShiftLeftLogical
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    197 => { // BitwiseOr
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    198 => { // BitwiseXor
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    199 => { // BitwiseAnd
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    200 => { // Not
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    201 => { // BitFieldInsert
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    202 => { // BitFieldSExtract
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    203 => { // BitFieldUExtract
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    204 => { // BitReverse
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    205 => { // BitCount
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    207 => { // DPdx
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    208 => { // DPdy
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    209 => { // Fwidth
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    210 => { // DPdxFine
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    211 => { // DPdyFine
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    212 => { // FwidthFine
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    213 => { // DPdxCoarse
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    214 => { // DPdyCoarse
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    215 => { // FwidthCoarse
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    218 => { // EmitVertex
    },
    219 => { // EndPrimitive
    },
    220 => { // EmitStreamVertex
      ib.id(it.id());
    },
    221 => { // EndStreamPrimitive
      ib.id(it.id());
    },
    224 => { // ControlBarrier
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    225 => { // MemoryBarrier
      ib.id(it.id());
      ib.id(it.id());
    },
    227 => { // AtomicLoad
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    228 => { // AtomicStore
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    229 => { // AtomicExchange
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    230 => { // AtomicCompareExchange
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    231 => { // AtomicCompareExchangeWeak
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    232 => { // AtomicIIncrement
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    233 => { // AtomicIDecrement
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    234 => { // AtomicIAdd
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    235 => { // AtomicISub
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    236 => { // AtomicSMin
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    237 => { // AtomicUMin
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    238 => { // AtomicSMax
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    239 => { // AtomicUMax
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    240 => { // AtomicAnd
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    241 => { // AtomicOr
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    242 => { // AtomicXor
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    245 => { // Phi
      ib.id(it.id());
      result_id = it.id(); ib.res();
      while !it.ate() {
        { ib.id(it.id()); ib.id(it.id()); }
      }
    },
    246 => { // LoopMerge
      ib.id(it.id());
      ib.id(it.id());
      ib.lit(it.u32());
    },
    247 => { // SelectionMerge
      ib.id(it.id());
      ib.lit(it.u32());
    },
    248 => { // Label
      result_id = it.id(); ib.res();
    },
    249 => { // Branch
      ib.id(it.id());
    },
    250 => { // BranchConditional
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      while !it.ate() {
        ib.lit(it.u32());
      }
    },
    251 => { // Switch
      ib.id(it.id());
      ib.id(it.id());
      while !it.ate() {
        { ib.lit(it.u32()); ib.id(it.id()); }
      }
    },
    252 => { // Kill
    },
    253 => { // Return
    },
    254 => { // ReturnValue
      ib.id(it.id());
    },
    255 => { // Unreachable
    },
    256 => { // LifetimeStart
      ib.id(it.id());
      ib.lit(it.u32());
    },
    257 => { // LifetimeStop
      ib.id(it.id());
      ib.lit(it.u32());
    },
    259 => { // GroupAsyncCopy
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    260 => { // GroupWaitEvents
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    261 => { // GroupAll
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    262 => { // GroupAny
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    263 => { // GroupBroadcast
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    264 => { // GroupIAdd
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
    },
    265 => { // GroupFAdd
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
    },
    266 => { // GroupFMin
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
    },
    267 => { // GroupUMin
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
    },
    268 => { // GroupSMin
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
    },
    269 => { // GroupFMax
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
    },
    270 => { // GroupUMax
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
    },
    271 => { // GroupSMax
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
    },
    274 => { // ReadPipe
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    275 => { // WritePipe
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    276 => { // ReservedReadPipe
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    277 => { // ReservedWritePipe
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    278 => { // ReserveReadPipePackets
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    279 => { // ReserveWritePipePackets
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    280 => { // CommitReadPipe
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    281 => { // CommitWritePipe
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    282 => { // IsValidReserveId
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    283 => { // GetNumPipePackets
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    284 => { // GetMaxPipePackets
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    285 => { // GroupReserveReadPipePackets
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    286 => { // GroupReserveWritePipePackets
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    287 => { // GroupCommitReadPipe
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    288 => { // GroupCommitWritePipe
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    291 => { // EnqueueMarker
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    292 => { // EnqueueKernel
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      while !it.ate() {
        ib.id(it.id());
      }
    },
    293 => { // GetKernelNDrangeSubGroupCount
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    294 => { // GetKernelNDrangeMaxSubGroupSize
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    295 => { // GetKernelWorkGroupSize
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    296 => { // GetKernelPreferredWorkGroupSizeMultiple
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    297 => { // RetainEvent
      ib.id(it.id());
    },
    298 => { // ReleaseEvent
      ib.id(it.id());
    },
    299 => { // CreateUserEvent
      ib.id(it.id());
      result_id = it.id(); ib.res();
    },
    300 => { // IsValidEvent
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    301 => { // SetUserEventStatus
      ib.id(it.id());
      ib.id(it.id());
    },
    302 => { // CaptureEventProfilingInfo
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    303 => { // GetDefaultQueue
      ib.id(it.id());
      result_id = it.id(); ib.res();
    },
    304 => { // BuildNDRange
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    305 => { // ImageSparseSampleImplicitLod
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    306 => { // ImageSparseSampleExplicitLod
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.lit(it.u32());
    },
    307 => { // ImageSparseSampleDrefImplicitLod
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    308 => { // ImageSparseSampleDrefExplicitLod
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.lit(it.u32());
    },
    309 => { // ImageSparseSampleProjImplicitLod
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    310 => { // ImageSparseSampleProjExplicitLod
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.lit(it.u32());
    },
    311 => { // ImageSparseSampleProjDrefImplicitLod
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    312 => { // ImageSparseSampleProjDrefExplicitLod
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.lit(it.u32());
    },
    313 => { // ImageSparseFetch
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    314 => { // ImageSparseGather
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    315 => { // ImageSparseDrefGather
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    316 => { // ImageSparseTexelsResident
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    317 => { // NoLine
    },
    318 => { // AtomicFlagTestAndSet
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    319 => { // AtomicFlagClear
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    320 => { // ImageSparseRead
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    321 => { // SizeOf
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    322 => { // TypePipeStorage
      result_id = it.id(); ib.res();
    },
    323 => { // ConstantPipeStorage
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    324 => { // CreatePipeFromPipeStorage
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    325 => { // GetKernelLocalSizeForSubgroupCount
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    326 => { // GetKernelMaxNumSubgroups
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    327 => { // TypeNamedBarrier
      result_id = it.id(); ib.res();
    },
    328 => { // NamedBarrierInitialize
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    329 => { // MemoryNamedBarrier
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    330 => { // ModuleProcessed
      while let Some(w) = it.next() { ib.lit(w); if (w >> 24) == 0 { break; } }
    },
    331 => { // ExecutionModeId
      ib.id(it.id());
      ib.lit(it.u32());
    },
    332 => { // DecorateId
      ib.id(it.id());
      ib.lit(it.u32());
    },
    333 => { // GroupNonUniformElect
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    334 => { // GroupNonUniformAll
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    335 => { // GroupNonUniformAny
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    336 => { // GroupNonUniformAllEqual
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    337 => { // GroupNonUniformBroadcast
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    338 => { // GroupNonUniformBroadcastFirst
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    339 => { // GroupNonUniformBallot
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    340 => { // GroupNonUniformInverseBallot
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    341 => { // GroupNonUniformBallotBitExtract
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    342 => { // GroupNonUniformBallotBitCount
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
    },
    343 => { // GroupNonUniformBallotFindLSB
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    344 => { // GroupNonUniformBallotFindMSB
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    345 => { // GroupNonUniformShuffle
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    346 => { // GroupNonUniformShuffleXor
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    347 => { // GroupNonUniformShuffleUp
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    348 => { // GroupNonUniformShuffleDown
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    349 => { // GroupNonUniformIAdd
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      if !it.ate() {
        ib.id(it.id());
      }
    },
    350 => { // GroupNonUniformFAdd
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      if !it.ate() {
        ib.id(it.id());
      }
    },
    351 => { // GroupNonUniformIMul
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      if !it.ate() {
        ib.id(it.id());
      }
    },
    352 => { // GroupNonUniformFMul
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      if !it.ate() {
        ib.id(it.id());
      }
    },
    353 => { // GroupNonUniformSMin
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      if !it.ate() {
        ib.id(it.id());
      }
    },
    354 => { // GroupNonUniformUMin
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      if !it.ate() {
        ib.id(it.id());
      }
    },
    355 => { // GroupNonUniformFMin
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      if !it.ate() {
        ib.id(it.id());
      }
    },
    356 => { // GroupNonUniformSMax
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      if !it.ate() {
        ib.id(it.id());
      }
    },
    357 => { // GroupNonUniformUMax
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      if !it.ate() {
        ib.id(it.id());
      }
    },
    358 => { // GroupNonUniformFMax
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      if !it.ate() {
        ib.id(it.id());
      }
    },
    359 => { // GroupNonUniformBitwiseAnd
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      if !it.ate() {
        ib.id(it.id());
      }
    },
    360 => { // GroupNonUniformBitwiseOr
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      if !it.ate() {
        ib.id(it.id());
      }
    },
    361 => { // GroupNonUniformBitwiseXor
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      if !it.ate() {
        ib.id(it.id());
      }
    },
    362 => { // GroupNonUniformLogicalAnd
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      if !it.ate() {
        ib.id(it.id());
      }
    },
    363 => { // GroupNonUniformLogicalOr
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      if !it.ate() {
        ib.id(it.id());
      }
    },
    364 => { // GroupNonUniformLogicalXor
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      if !it.ate() {
        ib.id(it.id());
      }
    },
    365 => { // GroupNonUniformQuadBroadcast
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    366 => { // GroupNonUniformQuadSwap
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    400 => { // CopyLogical
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    401 => { // PtrEqual
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    402 => { // PtrNotEqual
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    403 => { // PtrDiff
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    4416 => { // TerminateInvocation
    },
    4421 => { // SubgroupBallotKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    4422 => { // SubgroupFirstInvocationKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    4428 => { // SubgroupAllKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    4429 => { // SubgroupAnyKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    4430 => { // SubgroupAllEqualKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    4432 => { // SubgroupReadInvocationKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    4445 => { // TraceRayKHR
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    4446 => { // ExecuteCallableKHR
      ib.id(it.id());
      ib.id(it.id());
    },
    4447 => { // ConvertUToAccelerationStructureKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    4448 => { // IgnoreIntersectionKHR
    },
    4449 => { // TerminateRayKHR
    },
    4450 => { // SDotKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    4451 => { // UDotKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    4452 => { // SUDotKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    4453 => { // SDotAccSatKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    4454 => { // UDotAccSatKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    4455 => { // SUDotAccSatKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    4472 => { // TypeRayQueryKHR
      result_id = it.id(); ib.res();
    },
    4473 => { // RayQueryInitializeKHR
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    4474 => { // RayQueryTerminateKHR
      ib.id(it.id());
    },
    4475 => { // RayQueryGenerateIntersectionKHR
      ib.id(it.id());
      ib.id(it.id());
    },
    4476 => { // RayQueryConfirmIntersectionKHR
      ib.id(it.id());
    },
    4477 => { // RayQueryProceedKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    4479 => { // RayQueryGetIntersectionTypeKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5000 => { // GroupIAddNonUniformAMD
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
    },
    5001 => { // GroupFAddNonUniformAMD
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
    },
    5002 => { // GroupFMinNonUniformAMD
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
    },
    5003 => { // GroupUMinNonUniformAMD
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
    },
    5004 => { // GroupSMinNonUniformAMD
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
    },
    5005 => { // GroupFMaxNonUniformAMD
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
    },
    5006 => { // GroupUMaxNonUniformAMD
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
    },
    5007 => { // GroupSMaxNonUniformAMD
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
    },
    5011 => { // FragmentMaskFetchAMD
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5012 => { // FragmentFetchAMD
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5056 => { // ReadClockKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5283 => { // ImageSampleFootprintNV
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    5296 => { // GroupNonUniformPartitionNV
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5299 => { // WritePackedPrimitiveIndices4x8NV
      ib.id(it.id());
      ib.id(it.id());
    },
    5334 => { // ReportIntersectionNV
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    // Ignored alias op OpReportIntersectionKHR.
    5335 => { // IgnoreIntersectionNV
    },
    5336 => { // TerminateRayNV
    },
    5337 => { // TraceNV
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5338 => { // TraceMotionNV
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5339 => { // TraceRayMotionNV
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5341 => { // TypeAccelerationStructureNV
      result_id = it.id(); ib.res();
    },
    // Ignored alias op OpTypeAccelerationStructureKHR.
    5344 => { // ExecuteCallableNV
      ib.id(it.id());
      ib.id(it.id());
    },
    5358 => { // TypeCooperativeMatrixNV
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5359 => { // CooperativeMatrixLoadNV
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    5360 => { // CooperativeMatrixStoreNV
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      if !it.ate() {
        ib.lit(it.u32());
      }
    },
    5361 => { // CooperativeMatrixMulAddNV
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5362 => { // CooperativeMatrixLengthNV
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5364 => { // BeginInvocationInterlockEXT
    },
    5365 => { // EndInvocationInterlockEXT
    },
    5380 => { // DemoteToHelperInvocationEXT
    },
    5381 => { // IsHelperInvocationEXT
      ib.id(it.id());
      result_id = it.id(); ib.res();
    },
    5391 => { // ConvertUToImageNV
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5392 => { // ConvertUToSamplerNV
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5393 => { // ConvertImageToUNV
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5394 => { // ConvertSamplerToUNV
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5395 => { // ConvertUToSampledImageNV
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5396 => { // ConvertSampledImageToUNV
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5397 => { // SamplerImageAddressingModeNV
      ib.lit(it.u32());
    },
    5571 => { // SubgroupShuffleINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5572 => { // SubgroupShuffleDownINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5573 => { // SubgroupShuffleUpINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5574 => { // SubgroupShuffleXorINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5575 => { // SubgroupBlockReadINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5576 => { // SubgroupBlockWriteINTEL
      ib.id(it.id());
      ib.id(it.id());
    },
    5577 => { // SubgroupImageBlockReadINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5578 => { // SubgroupImageBlockWriteINTEL
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5580 => { // SubgroupImageMediaBlockReadINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5581 => { // SubgroupImageMediaBlockWriteINTEL
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5585 => { // UCountLeadingZerosINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5586 => { // UCountTrailingZerosINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5587 => { // AbsISubINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5588 => { // AbsUSubINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5589 => { // IAddSatINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5590 => { // UAddSatINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5591 => { // IAverageINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5592 => { // UAverageINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5593 => { // IAverageRoundedINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5594 => { // UAverageRoundedINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5595 => { // ISubSatINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5596 => { // USubSatINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5597 => { // IMul32x16INTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5598 => { // UMul32x16INTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5600 => { // ConstantFunctionPointerINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5601 => { // FunctionPointerCallINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      while !it.ate() {
        ib.id(it.id());
      }
    },
    5609 => { // AsmTargetINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      while let Some(w) = it.next() { ib.lit(w); if (w >> 24) == 0 { break; } }
    },
    5610 => { // AsmINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      while let Some(w) = it.next() { ib.lit(w); if (w >> 24) == 0 { break; } }
      while let Some(w) = it.next() { ib.lit(w); if (w >> 24) == 0 { break; } }
    },
    5611 => { // AsmCallINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      while !it.ate() {
        ib.id(it.id());
      }
    },
    5614 => { // AtomicFMinEXT
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5615 => { // AtomicFMaxEXT
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5630 => { // AssumeTrueKHR
      ib.id(it.id());
    },
    5631 => { // ExpectKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5632 => { // DecorateString
      ib.id(it.id());
      ib.lit(it.u32());
    },
    // Ignored alias op OpDecorateStringGOOGLE.
    5633 => { // MemberDecorateString
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    // Ignored alias op OpMemberDecorateStringGOOGLE.
    5699 => { // VmeImageINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5700 => { // TypeVmeImageINTEL
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5701 => { // TypeAvcImePayloadINTEL
      result_id = it.id(); ib.res();
    },
    5702 => { // TypeAvcRefPayloadINTEL
      result_id = it.id(); ib.res();
    },
    5703 => { // TypeAvcSicPayloadINTEL
      result_id = it.id(); ib.res();
    },
    5704 => { // TypeAvcMcePayloadINTEL
      result_id = it.id(); ib.res();
    },
    5705 => { // TypeAvcMceResultINTEL
      result_id = it.id(); ib.res();
    },
    5706 => { // TypeAvcImeResultINTEL
      result_id = it.id(); ib.res();
    },
    5707 => { // TypeAvcImeResultSingleReferenceStreamoutINTEL
      result_id = it.id(); ib.res();
    },
    5708 => { // TypeAvcImeResultDualReferenceStreamoutINTEL
      result_id = it.id(); ib.res();
    },
    5709 => { // TypeAvcImeSingleReferenceStreaminINTEL
      result_id = it.id(); ib.res();
    },
    5710 => { // TypeAvcImeDualReferenceStreaminINTEL
      result_id = it.id(); ib.res();
    },
    5711 => { // TypeAvcRefResultINTEL
      result_id = it.id(); ib.res();
    },
    5712 => { // TypeAvcSicResultINTEL
      result_id = it.id(); ib.res();
    },
    5713 => { // SubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5714 => { // SubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5715 => { // SubgroupAvcMceGetDefaultInterShapePenaltyINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5716 => { // SubgroupAvcMceSetInterShapePenaltyINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5717 => { // SubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5718 => { // SubgroupAvcMceSetInterDirectionPenaltyINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5719 => { // SubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5720 => { // SubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5721 => { // SubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
    },
    5722 => { // SubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
    },
    5723 => { // SubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
    },
    5724 => { // SubgroupAvcMceSetMotionVectorCostFunctionINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5725 => { // SubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5726 => { // SubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
    },
    5727 => { // SubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
    },
    5728 => { // SubgroupAvcMceSetAcOnlyHaarINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5729 => { // SubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5730 => { // SubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5731 => { // SubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5732 => { // SubgroupAvcMceConvertToImePayloadINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5733 => { // SubgroupAvcMceConvertToImeResultINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5734 => { // SubgroupAvcMceConvertToRefPayloadINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5735 => { // SubgroupAvcMceConvertToRefResultINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5736 => { // SubgroupAvcMceConvertToSicPayloadINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5737 => { // SubgroupAvcMceConvertToSicResultINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5738 => { // SubgroupAvcMceGetMotionVectorsINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5739 => { // SubgroupAvcMceGetInterDistortionsINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5740 => { // SubgroupAvcMceGetBestInterDistortionsINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5741 => { // SubgroupAvcMceGetInterMajorShapeINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5742 => { // SubgroupAvcMceGetInterMinorShapeINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5743 => { // SubgroupAvcMceGetInterDirectionsINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5744 => { // SubgroupAvcMceGetInterMotionVectorCountINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5745 => { // SubgroupAvcMceGetInterReferenceIdsINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5746 => { // SubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5747 => { // SubgroupAvcImeInitializeINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5748 => { // SubgroupAvcImeSetSingleReferenceINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5749 => { // SubgroupAvcImeSetDualReferenceINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5750 => { // SubgroupAvcImeRefWindowSizeINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5751 => { // SubgroupAvcImeAdjustRefOffsetINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5752 => { // SubgroupAvcImeConvertToMcePayloadINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5753 => { // SubgroupAvcImeSetMaxMotionVectorCountINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5754 => { // SubgroupAvcImeSetUnidirectionalMixDisableINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5755 => { // SubgroupAvcImeSetEarlySearchTerminationThresholdINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5756 => { // SubgroupAvcImeSetWeightedSadINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5757 => { // SubgroupAvcImeEvaluateWithSingleReferenceINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5758 => { // SubgroupAvcImeEvaluateWithDualReferenceINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5759 => { // SubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5760 => { // SubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5761 => { // SubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5762 => { // SubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5763 => { // SubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5764 => { // SubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5765 => { // SubgroupAvcImeConvertToMceResultINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5766 => { // SubgroupAvcImeGetSingleReferenceStreaminINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5767 => { // SubgroupAvcImeGetDualReferenceStreaminINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5768 => { // SubgroupAvcImeStripSingleReferenceStreamoutINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5769 => { // SubgroupAvcImeStripDualReferenceStreamoutINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5770 => { // SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5771 => { // SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5772 => { // SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5773 => { // SubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5774 => { // SubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5775 => { // SubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5776 => { // SubgroupAvcImeGetBorderReachedINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5777 => { // SubgroupAvcImeGetTruncatedSearchIndicationINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5778 => { // SubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5779 => { // SubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5780 => { // SubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5781 => { // SubgroupAvcFmeInitializeINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5782 => { // SubgroupAvcBmeInitializeINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5783 => { // SubgroupAvcRefConvertToMcePayloadINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5784 => { // SubgroupAvcRefSetBidirectionalMixDisableINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5785 => { // SubgroupAvcRefSetBilinearFilterEnableINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5786 => { // SubgroupAvcRefEvaluateWithSingleReferenceINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5787 => { // SubgroupAvcRefEvaluateWithDualReferenceINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5788 => { // SubgroupAvcRefEvaluateWithMultiReferenceINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5789 => { // SubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5790 => { // SubgroupAvcRefConvertToMceResultINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5791 => { // SubgroupAvcSicInitializeINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5792 => { // SubgroupAvcSicConfigureSkcINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5793 => { // SubgroupAvcSicConfigureIpeLumaINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5794 => { // SubgroupAvcSicConfigureIpeLumaChromaINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5795 => { // SubgroupAvcSicGetMotionVectorMaskINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5796 => { // SubgroupAvcSicConvertToMcePayloadINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5797 => { // SubgroupAvcSicSetIntraLumaShapePenaltyINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5798 => { // SubgroupAvcSicSetIntraLumaModeCostFunctionINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5799 => { // SubgroupAvcSicSetIntraChromaModeCostFunctionINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5800 => { // SubgroupAvcSicSetBilinearFilterEnableINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5801 => { // SubgroupAvcSicSetSkcForwardTransformEnableINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5802 => { // SubgroupAvcSicSetBlockBasedRawSkipSadINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5803 => { // SubgroupAvcSicEvaluateIpeINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5804 => { // SubgroupAvcSicEvaluateWithSingleReferenceINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5805 => { // SubgroupAvcSicEvaluateWithDualReferenceINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5806 => { // SubgroupAvcSicEvaluateWithMultiReferenceINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5807 => { // SubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    5808 => { // SubgroupAvcSicConvertToMceResultINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5809 => { // SubgroupAvcSicGetIpeLumaShapeINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5810 => { // SubgroupAvcSicGetBestIpeLumaDistortionINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5811 => { // SubgroupAvcSicGetBestIpeChromaDistortionINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5812 => { // SubgroupAvcSicGetPackedIpeLumaModesINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5813 => { // SubgroupAvcSicGetIpeChromaModeINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5814 => { // SubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5815 => { // SubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5816 => { // SubgroupAvcSicGetInterRawSadsINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5818 => { // VariableLengthArrayINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5819 => { // SaveMemoryINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
    },
    5820 => { // RestoreMemoryINTEL
      ib.id(it.id());
    },
    5840 => { // ArbitraryFloatSinCosPiINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5841 => { // ArbitraryFloatCastINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5842 => { // ArbitraryFloatCastFromIntINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5843 => { // ArbitraryFloatCastToIntINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5846 => { // ArbitraryFloatAddINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5847 => { // ArbitraryFloatSubINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5848 => { // ArbitraryFloatMulINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5849 => { // ArbitraryFloatDivINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5850 => { // ArbitraryFloatGTINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      ib.lit(it.u32());
    },
    5851 => { // ArbitraryFloatGEINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      ib.lit(it.u32());
    },
    5852 => { // ArbitraryFloatLTINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      ib.lit(it.u32());
    },
    5853 => { // ArbitraryFloatLEINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      ib.lit(it.u32());
    },
    5854 => { // ArbitraryFloatEQINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      ib.lit(it.u32());
    },
    5855 => { // ArbitraryFloatRecipINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5856 => { // ArbitraryFloatRSqrtINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5857 => { // ArbitraryFloatCbrtINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5858 => { // ArbitraryFloatHypotINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5859 => { // ArbitraryFloatSqrtINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5860 => { // ArbitraryFloatLogINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5861 => { // ArbitraryFloatLog2INTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5862 => { // ArbitraryFloatLog10INTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5863 => { // ArbitraryFloatLog1pINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5864 => { // ArbitraryFloatExpINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5865 => { // ArbitraryFloatExp2INTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5866 => { // ArbitraryFloatExp10INTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5867 => { // ArbitraryFloatExpm1INTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5868 => { // ArbitraryFloatSinINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5869 => { // ArbitraryFloatCosINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5870 => { // ArbitraryFloatSinCosINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5871 => { // ArbitraryFloatSinPiINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5872 => { // ArbitraryFloatCosPiINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5873 => { // ArbitraryFloatASinINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5874 => { // ArbitraryFloatASinPiINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5875 => { // ArbitraryFloatACosINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5876 => { // ArbitraryFloatACosPiINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5877 => { // ArbitraryFloatATanINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5878 => { // ArbitraryFloatATanPiINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5879 => { // ArbitraryFloatATan2INTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5880 => { // ArbitraryFloatPowINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5881 => { // ArbitraryFloatPowRINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5882 => { // ArbitraryFloatPowNINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.lit(it.u32());
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5887 => { // LoopControlINTEL
      while !it.ate() {
        ib.lit(it.u32());
      }
    },
    5923 => { // FixedSqrtINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5924 => { // FixedRecipINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5925 => { // FixedRsqrtINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5926 => { // FixedSinINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5927 => { // FixedCosINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5928 => { // FixedSinCosINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5929 => { // FixedSinPiINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5930 => { // FixedCosPiINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5931 => { // FixedSinCosPiINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5932 => { // FixedLogINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5933 => { // FixedExpINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
      ib.lit(it.u32());
    },
    5934 => { // PtrCastToCrossWorkgroupINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5938 => { // CrossWorkgroupCastToPtrINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    5946 => { // ReadPipeBlockingINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5947 => { // WritePipeBlockingINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    5949 => { // FPGARegINTEL
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    6016 => { // RayQueryGetRayTMinKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    6017 => { // RayQueryGetRayFlagsKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    6018 => { // RayQueryGetIntersectionTKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    6019 => { // RayQueryGetIntersectionInstanceCustomIndexKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    6020 => { // RayQueryGetIntersectionInstanceIdKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    6021 => { // RayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    6022 => { // RayQueryGetIntersectionGeometryIndexKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    6023 => { // RayQueryGetIntersectionPrimitiveIndexKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    6024 => { // RayQueryGetIntersectionBarycentricsKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    6025 => { // RayQueryGetIntersectionFrontFaceKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    6026 => { // RayQueryGetIntersectionCandidateAABBOpaqueKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    6027 => { // RayQueryGetIntersectionObjectRayDirectionKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    6028 => { // RayQueryGetIntersectionObjectRayOriginKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    6029 => { // RayQueryGetWorldRayDirectionKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    6030 => { // RayQueryGetWorldRayOriginKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
    },
    6031 => { // RayQueryGetIntersectionObjectToWorldKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    6032 => { // RayQueryGetIntersectionWorldToObjectKHR
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
    },
    6035 => { // AtomicFAddEXT
      ib.id(it.id());
      result_id = it.id(); ib.res();
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
      ib.id(it.id());
    },
    6086 => { // TypeBufferSurfaceINTEL
      result_id = it.id(); ib.res();
      ib.lit(it.u32());
    },
    6090 => { // TypeStructContinuedINTEL
      while !it.ate() {
        ib.id(it.id());
      }
    },
    6091 => { // ConstantCompositeContinuedINTEL
      while !it.ate() {
        ib.id(it.id());
      }
    },
    6092 => { // SpecConstantCompositeContinuedINTEL
      while !it.ate() {
        ib.id(it.id());
      }
    },
    _ => return Err(Error::UNSUPPORTED_OP),
  }
  while let Some(w) = it.next() { ib.lit(w); }
  let out = ib.build().map(|x| (result_id, x));
  Ok(out)
}
