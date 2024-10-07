use std::fmt::Debug;

use crate::{Program, WORD_SIZE_BYTES};

mod debug;

pub struct Memory([u8; 65536]);

impl Default for Memory {
    fn default() -> Self {
        Memory([0; 65536])
    }
}

impl Debug for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Memory").field(&"buncha-memory").finish()
    }
}

impl From<Program> for Memory {
    fn from(p: Program) -> Self {
        let mut memory = Memory::default();
        for (i, inst) in p.instructions.iter().enumerate() {
            let memory_slice = &mut memory.0[i * WORD_SIZE_BYTES..(i + 1) * WORD_SIZE_BYTES];
            let instruction_bytes: [u8; 2] = inst.into();
            memory_slice.clone_from_slice(&instruction_bytes);
        }

        memory
    }
}

#[cfg(test)]
mod tests {
    use crate::{memory::debug::dump_slice_to_binary, Memory, Program};

    #[test]
    pub fn test_fill_memory() -> eyre::Result<()> {
        let assembly = r#"ADD R1, R2, 0; this is a program
ADD R3, R4, 20"#;
        let prog = Program::from_assembly(assembly)?;

        let memory: Memory = Memory::from(prog);

        let memory_repr = dump_slice_to_binary(&memory.0[0..10]);

        let expected = r#"0001001010100000
0000000000000000
0000000000000000
0000000000000000
0000000000000000
"#;

        assert_eq!(expected, memory_repr);

        Ok(())
    }
}
