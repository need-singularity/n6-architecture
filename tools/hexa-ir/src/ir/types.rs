/// HEXA-IR Types — σ-τ=8 primitives + n=6 compound = σ+φ=14 total
use crate::util::n6::*;

#[derive(Clone, Debug, PartialEq)]
pub enum HexaType {
    // Primitives (σ-τ=8)
    I64, F64, Bool, Char, Str, Byte, Void, Any,
    // Compound (n=6)
    Struct(Vec<HexaType>),
    Enum(Vec<HexaType>),
    Array(Box<HexaType>, usize),
    Fn(Vec<HexaType>, Box<HexaType>),
    /// Trait object: (data_ptr, vtable_ptr) — fat pointer for dynamic dispatch
    /// Contains the trait name for vtable resolution
    TraitObject(String),
    /// Closure: (fn_ptr, env_ptr) — lambda-lifted callable
    ClosureObj(Vec<HexaType>, Box<HexaType>),
}

impl HexaType {
    pub fn size_bytes(&self) -> usize {
        match self {
            HexaType::I64 | HexaType::F64 => SIGMA_TAU, // σ-τ = 8 bytes
            HexaType::Bool | HexaType::Byte => MU,       // μ = 1 byte
            HexaType::Char => TAU,                        // τ = 4 bytes (UTF-32)
            HexaType::Str => PHI_TAU,                     // 2^τ = 16 (ptr+len)
            HexaType::Void => 0,
            HexaType::Any => SIGMA_TAU,
            HexaType::Array(inner, count) => inner.size_bytes() * count,
            HexaType::Struct(fields) => fields.iter().map(|f| f.size_bytes()).sum(),
            // Fat pointers: data_ptr + vtable_ptr = 2 * 8 = 16 bytes
            HexaType::TraitObject(_) => PHI_TAU,
            // Closure object: fn_ptr + env_ptr = 2 * 8 = 16 bytes
            HexaType::ClosureObj(_, _) => PHI_TAU,
            _ => SIGMA_TAU, // pointer-sized for Enum, Fn
        }
    }

    pub fn is_primitive(&self) -> bool {
        matches!(self,
            HexaType::I64 | HexaType::F64 | HexaType::Bool | HexaType::Char |
            HexaType::Str | HexaType::Byte | HexaType::Void | HexaType::Any
        )
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self, HexaType::I64 | HexaType::F64)
    }
}
