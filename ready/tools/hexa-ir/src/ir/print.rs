/// IR Pretty Printer — textual representation for debugging and self-hosting
use super::instr::HexaFunction;

pub fn print_function(func: &HexaFunction) -> String {
    let mut out = String::new();
    let params: Vec<String> = func.params.iter()
        .enumerate()
        .map(|(i, (name, ty))| format!("%{}: {:?}", name, ty))
        .collect();
    out.push_str(&format!("fn @{}({}) -> {:?} {{\n", func.name, params.join(", "), func.ret_ty));

    for block in &func.blocks {
        out.push_str(&format!("  bb{}:", block.id));
        if !block.successors.is_empty() {
            let succs: Vec<String> = block.successors.iter().map(|s| format!("bb{}", s)).collect();
            out.push_str(&format!("  ; succs=[{}]", succs.join(", ")));
        }
        out.push('\n');
        for instr in &block.instrs {
            if let Some(dest) = instr.dest {
                out.push_str(&format!("    %{} = {:?}", dest, instr.op));
            } else {
                out.push_str(&format!("    {:?}", instr.op));
            }
            if !instr.args.is_empty() {
                let args: Vec<String> = instr.args.iter().map(|a| format!("%{}", a)).collect();
                out.push_str(&format!(" {}", args.join(", ")));
            }
            out.push_str(&format!(" : {:?}\n", instr.ty));
        }
    }
    out.push_str("}\n");
    out
}
