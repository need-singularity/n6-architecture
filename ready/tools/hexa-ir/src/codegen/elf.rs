/// Minimal ELF 64-bit Writer
///
/// Produces a minimal statically-linked ELF executable with:
/// - ELF64 header
/// - Single PT_LOAD program header
/// - .text section containing the machine code

/// Write a minimal ELF64 executable
pub fn write_elf(code: &[u8], entry_vaddr: u64) -> Vec<u8> {
    let mut elf = Vec::new();

    // ── ELF Header (64 bytes) ──
    let ehdr_size: u16 = 64;
    // n6: ELF64 Phdr size = σ·τ + σ-τ = 48+8 = 56 (ELF binary format standard)
    let phdr_size: u16 = (crate::util::n6::SIGMA * crate::util::n6::TAU
                        + crate::util::n6::SIGMA_TAU) as u16;  // σ·τ + (σ-τ) = 56
    let phdr_offset: u64 = ehdr_size as u64;
    let code_offset: u64 = phdr_offset + phdr_size as u64;
    let code_vaddr = entry_vaddr;
    let file_size = code_offset + code.len() as u64;

    // e_ident: ELF magic + class + data + version + OS/ABI
    elf.extend_from_slice(&[
        0x7f, b'E', b'L', b'F', // magic
        2,                        // 64-bit
        1,                        // little-endian
        1,                        // ELF version 1
        0,                        // System V ABI
        0, 0, 0, 0, 0, 0, 0, 0,  // padding
    ]);
    // e_type: ET_EXEC (2)
    elf.extend_from_slice(&2u16.to_le_bytes());
    // e_machine: EM_X86_64 (62)
    elf.extend_from_slice(&62u16.to_le_bytes());
    // e_version
    elf.extend_from_slice(&1u32.to_le_bytes());
    // e_entry
    elf.extend_from_slice(&code_vaddr.to_le_bytes());
    // e_phoff
    elf.extend_from_slice(&phdr_offset.to_le_bytes());
    // e_shoff (no section headers)
    elf.extend_from_slice(&0u64.to_le_bytes());
    // e_flags
    elf.extend_from_slice(&0u32.to_le_bytes());
    // e_ehsize
    elf.extend_from_slice(&ehdr_size.to_le_bytes());
    // e_phentsize
    elf.extend_from_slice(&phdr_size.to_le_bytes());
    // e_phnum (1 segment)
    elf.extend_from_slice(&1u16.to_le_bytes());
    // e_shentsize
    elf.extend_from_slice(&0u16.to_le_bytes());
    // e_shnum
    elf.extend_from_slice(&0u16.to_le_bytes());
    // e_shstrndx
    elf.extend_from_slice(&0u16.to_le_bytes());

    // ── Program Header (PT_LOAD, 56 bytes) ──
    // p_type: PT_LOAD (1)
    elf.extend_from_slice(&1u32.to_le_bytes());
    // p_flags: PF_R | PF_X (5)
    elf.extend_from_slice(&5u32.to_le_bytes());
    // p_offset: file offset of segment
    elf.extend_from_slice(&code_offset.to_le_bytes());
    // p_vaddr
    elf.extend_from_slice(&code_vaddr.to_le_bytes());
    // p_paddr (same as vaddr for simple cases)
    elf.extend_from_slice(&code_vaddr.to_le_bytes());
    // p_filesz
    elf.extend_from_slice(&(code.len() as u64).to_le_bytes());
    // p_memsz
    elf.extend_from_slice(&(code.len() as u64).to_le_bytes());
    // p_align
    elf.extend_from_slice(&0x1000u64.to_le_bytes());

    // ── .text section (machine code) ──
    elf.extend_from_slice(code);

    assert_eq!(elf.len(), file_size as usize);
    elf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elf_header_magic() {
        let code = vec![0xc3]; // ret
        let elf = write_elf(&code, 0x401000);
        assert_eq!(&elf[0..4], &[0x7f, b'E', b'L', b'F']);
        assert_eq!(elf[4], 2); // 64-bit
        assert_eq!(elf[5], 1); // little-endian
    }

    #[test]
    fn test_elf_entry_point() {
        let code = vec![0xc3];
        let elf = write_elf(&code, 0x401000);
        let entry = u64::from_le_bytes(elf[24..32].try_into().unwrap());
        assert_eq!(entry, 0x401000);
    }
}
