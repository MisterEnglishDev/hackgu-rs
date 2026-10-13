//! x86_64 function disassembler, control flow graph (CFG) builder,
//! and struct field layout recovery powered by `iced-x86`.

use std::collections::BTreeSet;
use iced_x86::{Decoder, DecoderOptions, FlowControl, Formatter, Instruction, NasmFormatter, OpKind, Register};

#[derive(Debug, Clone)]
pub struct FunctionEntry {
    pub rva: u32,
    pub size: u32,
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DisassembledBasicBlock {
    pub start_ip: u64,
    pub end_ip: u64,
    pub instructions: Vec<Instruction>,
    pub successors: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StructFieldAccess {
    pub offset: u32,
    pub size: usize,
    pub is_write: bool,
    pub is_float: bool,
}

#[derive(Debug, Clone)]
pub struct DecompiledFunction {
    pub rva: u32,
    pub size: u32,
    pub name: Option<String>,
    pub instructions: Vec<Instruction>,
    pub basic_blocks: Vec<DisassembledBasicBlock>,
    pub calls: Vec<u64>,
    pub field_accesses: BTreeSet<StructFieldAccess>,
    pub stack_size: u32,
}

pub struct CodeAnalyzer<'a> {
    pub code: &'a [u8],
    pub base_rva: u32,
    pub image_base: u64,
}

impl<'a> CodeAnalyzer<'a> {
    pub fn new(code: &'a [u8], base_rva: u32, image_base: u64) -> Self {
        Self { code, base_rva, image_base }
    }

    /// Disassemble a specific function at RVA with iced-x86
    pub fn disassemble_function(&self, func_rva: u32, max_len: usize) -> Option<DecompiledFunction> {
        let offset = func_rva.checked_sub(self.base_rva)? as usize;
        if offset >= self.code.len() {
            return None;
        }

        let slice_len = max_len.min(self.code.len() - offset);
        let bytes = &self.code[offset..offset + slice_len];
        let start_ip = self.image_base + func_rva as u64;

        let mut decoder = Decoder::with_ip(64, bytes, start_ip, DecoderOptions::NONE);
        let mut instructions = Vec::new();
        let mut calls = Vec::new();
        let mut field_accesses = BTreeSet::new();
        let mut stack_size = 0u32;

        let mut current_insn = Instruction::default();
        while decoder.can_decode() {
            decoder.decode_out(&mut current_insn);
            instructions.push(current_insn);

            // 1. Analyze stack allocation in prologue: `sub rsp, XX`
            if current_insn.op0_register() == Register::RSP && current_insn.mnemonic() == iced_x86::Mnemonic::Sub {
                if let OpKind::Immediate8 | OpKind::Immediate32 = current_insn.op1_kind() {
                    stack_size = current_insn.immediate32();
                }
            }

            // 2. Analyze C++ `this` pointer accesses (`RCX` in Windows x64 ABI)
            for op_idx in 0..current_insn.op_count() {
                if current_insn.op_kind(op_idx) == OpKind::Memory {
                    let base_reg = current_insn.memory_base();
                    if base_reg == Register::RCX {
                        let disp = current_insn.memory_displacement64() as u32;
                        let size = current_insn.memory_size().size();
                        let is_write = op_idx == 0;
                        let m_name = format!("{:?}", current_insn.mnemonic());
                        let is_float = m_name.starts_with("Movs") || m_name.starts_with("Movd");
                        if disp < 0x10000 {
                            field_accesses.insert(StructFieldAccess {
                                offset: disp,
                                size,
                                is_write,
                                is_float,
                            });
                        }
                    }
                }
            }

            // 3. Analyze direct call targets
            if current_insn.flow_control() == FlowControl::Call {
                if current_insn.op0_kind() == OpKind::NearBranch64 || current_insn.op0_kind() == OpKind::NearBranch32 {
                    calls.push(current_insn.near_branch_target());
                }
            }

            // Stop at unconditional return or trailing padding if after function end
            if current_insn.flow_control() == FlowControl::Return {
                break;
            }
        }

        let func_size = if let Some(last) = instructions.last() {
            (last.next_ip() - start_ip) as u32
        } else {
            0
        };

        // Construct basic blocks
        let mut leaders = BTreeSet::new();
        leaders.insert(start_ip);
        for insn in &instructions {
            match insn.flow_control() {
                FlowControl::ConditionalBranch | FlowControl::UnconditionalBranch => {
                    leaders.insert(insn.near_branch_target());
                    leaders.insert(insn.next_ip());
                }
                FlowControl::Return => {
                    leaders.insert(insn.next_ip());
                }
                _ => {}
            }
        }

        let mut basic_blocks = Vec::new();
        let mut cur_block_insns: Vec<Instruction> = Vec::new();
        let mut cur_block_start = start_ip;

        for insn in &instructions {
            if leaders.contains(&insn.ip()) && !cur_block_insns.is_empty() {
                let end_ip = cur_block_insns.last().unwrap().next_ip();
                let mut succs = Vec::new();
                let last = cur_block_insns.last().unwrap();
                match last.flow_control() {
                    FlowControl::ConditionalBranch => {
                        succs.push(last.near_branch_target());
                        succs.push(last.next_ip());
                    }
                    FlowControl::UnconditionalBranch => {
                        succs.push(last.near_branch_target());
                    }
                    FlowControl::Next | FlowControl::Call => {
                        succs.push(last.next_ip());
                    }
                    _ => {}
                }
                basic_blocks.push(DisassembledBasicBlock {
                    start_ip: cur_block_start,
                    end_ip,
                    instructions: std::mem::take(&mut cur_block_insns),
                    successors: succs,
                });
                cur_block_start = insn.ip();
            }
            cur_block_insns.push(*insn);
        }

        if !cur_block_insns.is_empty() {
            let end_ip = cur_block_insns.last().unwrap().next_ip();
            basic_blocks.push(DisassembledBasicBlock {
                start_ip: cur_block_start,
                end_ip,
                instructions: cur_block_insns,
                successors: Vec::new(),
            });
        }

        Some(DecompiledFunction {
            rva: func_rva,
            size: func_size,
            name: None,
            instructions,
            basic_blocks,
            calls,
            field_accesses,
            stack_size,
        })
    }

    /// Format disassembled instructions as clean NASM text
    pub fn format_instructions(instructions: &[Instruction]) -> Vec<String> {
        let mut formatter = NasmFormatter::new();
        formatter.options_mut().set_digit_separator("_");
        formatter.options_mut().set_uppercase_hex(true);

        let mut output = String::new();
        let mut result = Vec::new();
        for insn in instructions {
            output.clear();
            formatter.format(insn, &mut output);
            result.push(format!("0x{:08X}: {}", insn.ip(), output));
        }
        result
    }

    /// Scan for function prologues in code section
    pub fn find_functions(&self) -> Vec<FunctionEntry> {
        let mut functions = Vec::new();
        let mut cur = 0;

        while cur + 4 < self.code.len() {
            let b0 = self.code[cur];
            let b1 = self.code[cur + 1];
            let b2 = self.code[cur + 2];
            let b3 = self.code[cur + 3];

            let is_prologue = (b0 == 0x55 && b1 == 0x48 && b2 == 0x89 && b3 == 0xe5)
                || (b0 == 0x48 && b1 == 0x89 && b2 == 0x5c && b3 == 0x24)
                || (b0 == 0x48 && b1 == 0x83 && b2 == 0xec)
                || (b0 == 0x48 && b1 == 0x81 && b2 == 0xec)
                || (b0 == 0x40 && (b1 == 0x53 || b1 == 0x55 || b1 == 0x57) && (b2 == 0x48 || b2 == 0x56));

            if is_prologue {
                let func_rva = self.base_rva + cur as u32;
                let mut f_end = cur + 1;
                while f_end < self.code.len() {
                    if self.code[f_end] == 0xc3 {
                        f_end += 1;
                        break;
                    }
                    if self.code[f_end] == 0xc2 && f_end + 3 <= self.code.len() {
                        f_end += 3;
                        break;
                    }
                    f_end += 1;
                }

                let size = (f_end - cur) as u32;
                functions.push(FunctionEntry {
                    rva: func_rva,
                    size,
                    name: None,
                });
                cur = f_end;
                continue;
            }

            cur += 1;
        }

        functions
    }
}
