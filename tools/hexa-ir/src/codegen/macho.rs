/// Minimal Mach-O 64-bit Writer
///
/// Produces a valid Mach-O executable for macOS (x86_64 and arm64).
/// Includes __PAGEZERO + __TEXT segments as required by dyld.

const MH_MAGIC_64: u32 = 0xfeedfacf;
const CPU_TYPE_X86_64: u32 = 0x01000007;
const CPU_TYPE_ARM64: u32 = 0x0100000c;
const CPU_SUBTYPE_ALL: u32 = 3;
const MH_EXECUTE: u32 = 2;
const LC_SEGMENT_64: u32 = 0x19;
const LC_MAIN: u32 = 0x80000028;

const PAGE_SIZE: u64 = 0x4000; // 16K pages on arm64 macOS

fn write_seg_cmd(out: &mut Vec<u8>, name: &[u8], vmaddr: u64, vmsize: u64,
                  fileoff: u64, filesize: u64, maxprot: u32, initprot: u32,
                  nsects: u32) {
    let cmdsize: u32 = 72 + nsects * 80;
    out.extend_from_slice(&LC_SEGMENT_64.to_le_bytes());
    out.extend_from_slice(&cmdsize.to_le_bytes());
    let mut segname = [0u8; 16];
    let len = name.len().min(16);
    segname[..len].copy_from_slice(&name[..len]);
    out.extend_from_slice(&segname);
    out.extend_from_slice(&vmaddr.to_le_bytes());
    out.extend_from_slice(&vmsize.to_le_bytes());
    out.extend_from_slice(&fileoff.to_le_bytes());
    out.extend_from_slice(&filesize.to_le_bytes());
    out.extend_from_slice(&maxprot.to_le_bytes());
    out.extend_from_slice(&initprot.to_le_bytes());
    out.extend_from_slice(&nsects.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes()); // flags
}

fn write_section(out: &mut Vec<u8>, sectname: &[u8], segname: &[u8],
                  addr: u64, size: u64, offset: u32, flags: u32) {
    let mut sn = [0u8; 16];
    sn[..sectname.len().min(16)].copy_from_slice(&sectname[..sectname.len().min(16)]);
    out.extend_from_slice(&sn);
    let mut sg = [0u8; 16];
    sg[..segname.len().min(16)].copy_from_slice(&segname[..segname.len().min(16)]);
    out.extend_from_slice(&sg);
    out.extend_from_slice(&addr.to_le_bytes());    // addr
    out.extend_from_slice(&size.to_le_bytes());    // size
    out.extend_from_slice(&offset.to_le_bytes());  // offset
    out.extend_from_slice(&2u32.to_le_bytes());    // align = 2^2 = 4
    out.extend_from_slice(&0u32.to_le_bytes());    // reloff
    out.extend_from_slice(&0u32.to_le_bytes());    // nreloc
    out.extend_from_slice(&flags.to_le_bytes());   // flags
    out.extend_from_slice(&0u32.to_le_bytes());    // reserved1
    out.extend_from_slice(&0u32.to_le_bytes());    // reserved2
    out.extend_from_slice(&0u32.to_le_bytes());    // reserved3
}

/// Write a minimal Mach-O 64-bit executable
pub fn write_macho(code: &[u8], text_vaddr: u64, arm64: bool) -> Vec<u8> {
    let mut macho = Vec::new();

    // Layout: header(32) + pagezero_cmd(72) + text_cmd(72+80) + main_cmd(24) + padding + code
    let header_size: u32 = 32;
    let pagezero_cmd_size: u32 = 72;     // LC_SEGMENT_64 (no sections)
    let text_cmd_size: u32 = 72 + 80;    // LC_SEGMENT_64 + 1 section
    let main_cmd_size: u32 = 24;         // LC_MAIN
    let ncmds: u32 = 3;
    let total_cmds_size = pagezero_cmd_size + text_cmd_size + main_cmd_size;
    let header_total = header_size + total_cmds_size;

    // Pad to page boundary for text segment
    let text_offset = ((header_total as u64 + PAGE_SIZE - 1) / PAGE_SIZE * PAGE_SIZE) as u32;
    let text_vmaddr = text_vaddr + text_offset as u64;

    // ── Mach-O Header (32 bytes) ──
    macho.extend_from_slice(&MH_MAGIC_64.to_le_bytes());
    let cpu_type = if arm64 { CPU_TYPE_ARM64 } else { CPU_TYPE_X86_64 };
    macho.extend_from_slice(&cpu_type.to_le_bytes());
    macho.extend_from_slice(&CPU_SUBTYPE_ALL.to_le_bytes());
    macho.extend_from_slice(&MH_EXECUTE.to_le_bytes());
    macho.extend_from_slice(&ncmds.to_le_bytes());
    macho.extend_from_slice(&total_cmds_size.to_le_bytes());
    let flags: u32 = if arm64 { 0x00200085 } else { 0x00000085 }; // PIE + DYLDLINK + TWOLEVEL
    macho.extend_from_slice(&flags.to_le_bytes());
    macho.extend_from_slice(&0u32.to_le_bytes());  // reserved

    // ── LC_SEGMENT_64: __PAGEZERO (required by dyld) ──
    write_seg_cmd(&mut macho, b"__PAGEZERO", 0, text_vaddr, 0, 0, 0, 0, 0);

    // ── LC_SEGMENT_64: __TEXT ──
    let text_file_size = text_offset as u64 + code.len() as u64;
    write_seg_cmd(&mut macho, b"__TEXT", text_vaddr, text_file_size,
                   0, text_file_size, 5, 5, 1);

    // Section: __text within __TEXT
    write_section(&mut macho, b"__text", b"__TEXT",
                   text_vmaddr, code.len() as u64, text_offset,
                   0x80000400); // S_ATTR_PURE_INSTRUCTIONS | S_ATTR_SOME_INSTRUCTIONS

    // ── LC_MAIN ──
    macho.extend_from_slice(&LC_MAIN.to_le_bytes());
    macho.extend_from_slice(&main_cmd_size.to_le_bytes());
    macho.extend_from_slice(&(text_offset as u64).to_le_bytes()); // entryoff
    macho.extend_from_slice(&0u64.to_le_bytes());                 // stacksize

    // ── Pad to text_offset ──
    while macho.len() < text_offset as usize {
        macho.push(0);
    }

    // ── .text section (machine code) ──
    macho.extend_from_slice(code);

    macho
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macho_magic() {
        let code = vec![0xc3];
        let macho = write_macho(&code, 0x100000000, false);
        let magic = u32::from_le_bytes(macho[0..4].try_into().unwrap());
        assert_eq!(magic, MH_MAGIC_64);
    }

    #[test]
    fn test_macho_arm64_cpu() {
        let code = vec![0xd2, 0x80, 0x00, 0x00];
        let macho = write_macho(&code, 0x100000000, true);
        let cpu = u32::from_le_bytes(macho[4..8].try_into().unwrap());
        assert_eq!(cpu, CPU_TYPE_ARM64);
    }
}
