/// Expression parser — Pratt parsing with sigma-tau=8 precedence levels
///
/// Precedence table (lowest to highest):
///   1: ||          (logic or)
///   2: &&          (logic and)
///   3: == != < > <= >=  (comparison)
///   4: | ^         (bitwise or/xor)
///   5: &           (bitwise and)
///   6: + -         (additive)
///   7: * / %       (multiplicative)
///   8: unary - !   (prefix)

use crate::lexer::{TokenKind, Span};
use super::ast::*;
use super::error::{Parser, ParseError};

/// Pratt parser entry point
pub fn parse_expr(p: &mut Parser) -> Result<Expr, ParseError> {
    parse_expr_bp(p, 0, false)
}

/// Parse expression without allowing struct-init (`Ident {` is not struct literal).
/// Used in if/while conditions where `{` starts a block, not a struct.
pub fn parse_expr_no_struct(p: &mut Parser) -> Result<Expr, ParseError> {
    parse_expr_bp(p, 0, true)
}

/// Pratt parser with minimum binding power
fn parse_expr_bp(p: &mut Parser, min_bp: u8, no_struct: bool) -> Result<Expr, ParseError> {
    let mut lhs = parse_prefix_inner(p, no_struct)?;

    loop {
        // Postfix operators (inner loop consumes all before checking infix)
        loop {
            lhs = match p.peek() {
                TokenKind::LParen => parse_call(p, lhs)?,
                TokenKind::Dot => parse_field_access(p, lhs)?,
                TokenKind::LBracket => parse_index(p, lhs)?,
                TokenKind::Question => {
                    let q_span = p.peek_span();
                    p.advance();
                    let span = lhs.span().merge(q_span);
                    Expr::TryExpr { expr: Box::new(lhs), span }
                }
                _ => break,
            };
        }

        // Infix binary operators
        let (op, l_bp, r_bp) = match infix_binding_power(p.peek()) {
            Some(triple) => triple,
            None => break,
        };

        if l_bp < min_bp {
            break;
        }

        p.advance(); // consume operator token
        let rhs = parse_expr_bp(p, r_bp, no_struct)?;
        let span = lhs.span().merge(rhs.span());
        lhs = Expr::Binary {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            span,
        };
    }

    Ok(lhs)
}

/// Parse prefix expressions (atoms + unary operators)
/// When `no_struct` is true, `Ident {` is NOT interpreted as struct-init
/// (used in if/while conditions where `{` starts a block).
fn parse_prefix_inner(p: &mut Parser, no_struct: bool) -> Result<Expr, ParseError> {
    let span = p.peek_span();

    match p.peek().clone() {
        TokenKind::IntLit(v) => {
            let v = v;
            p.advance();
            Ok(Expr::IntLit(v, span))
        }
        TokenKind::FloatLit(v) => {
            let v = v;
            p.advance();
            Ok(Expr::FloatLit(v, span))
        }
        TokenKind::True => {
            p.advance();
            Ok(Expr::BoolLit(true, span))
        }
        TokenKind::False => {
            p.advance();
            Ok(Expr::BoolLit(false, span))
        }
        TokenKind::StrLit(ref s) => {
            let s = s.clone();
            p.advance();
            Ok(Expr::StrLit(s, span))
        }
        TokenKind::Ident(ref name) => {
            let name = name.clone();
            p.advance();
            // Check for path expression: `Name::Variant` (enum construction)
            if p.at(&TokenKind::ColonColon) {
                parse_path_expr(p, name, span)
            // Check for struct initialization: `Name { field: val }`
            // Skip in no_struct mode (if/while conditions) where `{` starts a block
            } else if !no_struct && p.at(&TokenKind::LBrace) {
                parse_struct_init(p, name, span)
            } else {
                Ok(Expr::Ident(name, span))
            }
        }
        TokenKind::Match => {
            return parse_match_expr(p, span);
        }
        TokenKind::Minus => {
            p.advance();
            let operand = parse_expr_bp(p, prefix_bp(), no_struct)?;
            let full_span = span.merge(operand.span());
            Ok(Expr::Unary {
                op: UnOp::Neg,
                operand: Box::new(operand),
                span: full_span,
            })
        }
        TokenKind::Not => {
            p.advance();
            let operand = parse_expr_bp(p, prefix_bp(), no_struct)?;
            let full_span = span.merge(operand.span());
            Ok(Expr::Unary {
                op: UnOp::Not,
                operand: Box::new(operand),
                span: full_span,
            })
        }
        TokenKind::LParen => {
            p.advance(); // (
            let expr = parse_expr(p)?;
            p.expect(&TokenKind::RParen)?;
            Ok(expr)
        }
        TokenKind::LBracket => {
            return parse_array_literal(p, span);
        }
        // Closure: `|params| body` or `|| body`
        TokenKind::BitOr | TokenKind::Or => {
            return parse_closure(p, span);
        }
        other => Err(ParseError::new(
            span,
            format!("expected expression, found {:?}", other),
        )),
    }
}

/// Array literal: `[expr1, expr2, ...]`
fn parse_array_literal(p: &mut Parser, start_span: Span) -> Result<Expr, ParseError> {
    p.advance(); // consume [
    let mut elements = Vec::new();

    while !p.at(&TokenKind::RBracket) && !p.is_eof() {
        elements.push(parse_expr(p)?);
        if !p.eat(&TokenKind::Comma) {
            break;
        }
    }

    let end_span = p.peek_span();
    p.expect(&TokenKind::RBracket)?;
    Ok(Expr::ArrayLit {
        elements,
        span: start_span.merge(end_span),
    })
}

/// Closure expression: `|x, y| expr` or `|x: i64| -> i64 { body }` or `|| expr`
fn parse_closure(p: &mut Parser, start_span: Span) -> Result<Expr, ParseError> {
    let params = if p.eat(&TokenKind::Or) {
        // `||` — zero parameters
        Vec::new()
    } else {
        // `|` — parse params until closing `|`
        p.expect(&TokenKind::BitOr)?;
        let mut params = Vec::new();
        while !p.at(&TokenKind::BitOr) && !p.is_eof() {
            let (pname, _) = p.expect_ident()?;
            let ty = if p.eat(&TokenKind::Colon) {
                Some(super::stmt::parse_type_expr(p)?)
            } else {
                None
            };
            params.push((pname, ty));
            if !p.eat(&TokenKind::Comma) {
                break;
            }
        }
        p.expect(&TokenKind::BitOr)?;
        params
    };

    let ret_ty = if p.eat(&TokenKind::Arrow) {
        Some(Box::new(super::stmt::parse_type_expr(p)?))
    } else {
        None
    };

    let body = if p.at(&TokenKind::LBrace) {
        let block = super::stmt::parse_block(p)?;
        Expr::Block(block)
    } else {
        parse_expr(p)?
    };

    let span = start_span.merge(body.span());
    Ok(Expr::Closure { params, ret_ty, body: Box::new(body), span })
}

/// Struct initialization: `Name { field1: expr1, field2: expr2 }`
fn parse_struct_init(p: &mut Parser, name: String, start_span: Span) -> Result<Expr, ParseError> {
    p.expect(&TokenKind::LBrace)?;
    let mut fields = Vec::new();

    while !p.at(&TokenKind::RBrace) && !p.is_eof() {
        let (field_name, _) = p.expect_ident()?;
        p.expect(&TokenKind::Colon)?;
        let value = parse_expr(p)?;
        fields.push((field_name, value));

        if !p.eat(&TokenKind::Comma) {
            break;
        }
    }

    let end_span = p.peek_span();
    p.expect(&TokenKind::RBrace)?;
    Ok(Expr::StructInit {
        name,
        fields,
        span: start_span.merge(end_span),
    })
}

/// Function call: `expr(arg1, arg2, ...)`
fn parse_call(p: &mut Parser, func: Expr) -> Result<Expr, ParseError> {
    p.expect(&TokenKind::LParen)?;
    let mut args = Vec::new();

    while !p.at(&TokenKind::RParen) && !p.is_eof() {
        args.push(parse_expr(p)?);
        if !p.eat(&TokenKind::Comma) {
            break;
        }
    }

    let end_span = p.peek_span();
    p.expect(&TokenKind::RParen)?;
    Ok(Expr::Call {
        span: func.span().merge(end_span),
        func: Box::new(func),
        args,
    })
}

/// Field access: `expr.field`
fn parse_field_access(p: &mut Parser, obj: Expr) -> Result<Expr, ParseError> {
    p.expect(&TokenKind::Dot)?;
    let (name, name_span) = p.expect_ident()?;
    Ok(Expr::Field {
        span: obj.span().merge(name_span),
        obj: Box::new(obj),
        name,
    })
}


/// Index access: `expr[index]`
fn parse_index(p: &mut Parser, arr: Expr) -> Result<Expr, ParseError> {
    p.expect(&TokenKind::LBracket)?;
    let idx = parse_expr(p)?;
    let end_span = p.peek_span();
    p.expect(&TokenKind::RBracket)?;
    Ok(Expr::Index {
        span: arr.span().merge(end_span),
        arr: Box::new(arr),
        idx: Box::new(idx),
    })
}

/// Path expression: `Name::Variant` or `Name::Variant(args...)`
/// Desugared to a Call with the enum variant name as function
fn parse_path_expr(p: &mut Parser, enum_name: String, start_span: Span) -> Result<Expr, ParseError> {
    p.expect(&TokenKind::ColonColon)?;
    let (variant_name, variant_span) = p.expect_ident()?;
    let full_span = start_span.merge(variant_span);
    // Encode as Ident with qualified name "EnumName::VariantName"
    // This is resolved during lowering where enum_defs are available
    let qualified = format!("{}::{}", enum_name, variant_name);

    // Check for tuple payload: `Color::RGB(r, g, b)`
    if p.at(&TokenKind::LParen) {
        p.expect(&TokenKind::LParen)?;
        let mut args = Vec::new();
        while !p.at(&TokenKind::RParen) && !p.is_eof() {
            args.push(parse_expr(p)?);
            if !p.eat(&TokenKind::Comma) {
                break;
            }
        }
        let end_span = p.peek_span();
        p.expect(&TokenKind::RParen)?;
        Ok(Expr::Call {
            func: Box::new(Expr::Ident(qualified, full_span)),
            args,
            span: start_span.merge(end_span),
        })
    } else {
        Ok(Expr::Ident(qualified, full_span))
    }
}

/// Match expression: `match scrutinee { Pattern => body, ... }`
fn parse_match_expr(p: &mut Parser, start_span: Span) -> Result<Expr, ParseError> {
    p.expect(&TokenKind::Match)?;
    let scrutinee = parse_expr_no_struct(p)?;
    p.expect(&TokenKind::LBrace)?;

    let mut arms = Vec::new();
    while !p.at(&TokenKind::RBrace) && !p.is_eof() {
        let arm = parse_match_arm(p)?;
        arms.push(arm);
        // Optional comma between arms
        p.eat(&TokenKind::Comma);
    }

    let end_span = p.peek_span();
    p.expect(&TokenKind::RBrace)?;

    Ok(Expr::Match {
        scrutinee: Box::new(scrutinee),
        arms,
        span: start_span.merge(end_span),
    })
}

/// Parse a single match arm: `pattern => expr`
fn parse_match_arm(p: &mut Parser) -> Result<MatchArm, ParseError> {
    let start = p.peek_span();
    let pattern = parse_pattern(p)?;
    p.expect(&TokenKind::FatArrow)?;
    let body = parse_expr(p)?;
    let span = start.merge(body.span());
    Ok(MatchArm { pattern, body, span })
}

/// Parse a pattern with optional guard: `pat`, `pat if cond`
fn parse_pattern(p: &mut Parser) -> Result<Pattern, ParseError> {
    let inner = parse_pattern_atom(p)?;
    // Check for guard: `pat if cond`
    if p.at(&TokenKind::If) {
        p.advance();
        let condition = parse_expr(p)?;
        let span = inner.span().merge(condition.span());
        return Ok(Pattern::Guard {
            pattern: Box::new(inner),
            condition: Box::new(condition),
            span,
        });
    }
    Ok(inner)
}

/// Parse an atomic pattern (without guard)
fn parse_pattern_atom(p: &mut Parser) -> Result<Pattern, ParseError> {
    let span = p.peek_span();

    // Wildcard: `_`
    if let TokenKind::Ident(ref name) = p.peek().clone() {
        if name == "_" {
            p.advance();
            return Ok(Pattern::Wildcard(span));
        }
    }

    // Integer literal pattern
    if let TokenKind::IntLit(v) = p.peek().clone() {
        p.advance();
        return Ok(Pattern::Literal(v, span));
    }

    // Tuple pattern: `(pat1, pat2, ...)`
    if p.at(&TokenKind::LParen) {
        p.advance();
        let mut elements = Vec::new();
        while !p.at(&TokenKind::RParen) && !p.is_eof() {
            elements.push(parse_pattern(p)?);
            if !p.eat(&TokenKind::Comma) { break; }
        }
        let end = p.peek_span();
        p.expect(&TokenKind::RParen)?;
        return Ok(Pattern::Tuple { elements, span: span.merge(end) });
    }

    // Identifier-based patterns
    if let TokenKind::Ident(ref name) = p.peek().clone() {
        let ident_name = name.clone();
        p.advance();

        // Enum variant: `Name::Variant`
        if p.at(&TokenKind::ColonColon) {
            p.expect(&TokenKind::ColonColon)?;
            let (variant_name, variant_span) = p.expect_ident()?;
            let full_span = span.merge(variant_span);

            // Optional nested pattern bindings: `Variant(pat1, pat2)`
            let mut bindings = Vec::new();
            if p.at(&TokenKind::LParen) {
                p.expect(&TokenKind::LParen)?;
                while !p.at(&TokenKind::RParen) && !p.is_eof() {
                    bindings.push(parse_pattern(p)?);
                    if !p.eat(&TokenKind::Comma) { break; }
                }
                let end = p.peek_span();
                p.expect(&TokenKind::RParen)?;
                return Ok(Pattern::Variant {
                    enum_name: ident_name, variant_name, bindings,
                    span: span.merge(end),
                });
            }

            return Ok(Pattern::Variant {
                enum_name: ident_name, variant_name, bindings,
                span: full_span,
            });
        }

        // Struct destructure: `Name { field1, field2 }`
        if p.at(&TokenKind::LBrace) {
            p.expect(&TokenKind::LBrace)?;
            let mut fields = Vec::new();
            while !p.at(&TokenKind::RBrace) && !p.is_eof() {
                let (field_name, _) = p.expect_ident()?;
                let sub_pat = if p.eat(&TokenKind::Colon) {
                    Some(parse_pattern(p)?)
                } else {
                    None
                };
                fields.push((field_name, sub_pat));
                if !p.eat(&TokenKind::Comma) { break; }
            }
            let end = p.peek_span();
            p.expect(&TokenKind::RBrace)?;
            return Ok(Pattern::StructDestructure {
                struct_name: ident_name, fields, span: span.merge(end),
            });
        }

        // Plain identifier: variable binding
        return Ok(Pattern::Binding(ident_name, span));
    }

    Err(ParseError::new(span, format!("expected pattern, found {:?}", p.peek())))
}

/// Binding power for prefix (unary) operators — level 8 (highest)
fn prefix_bp() -> u8 {
    15 // higher than any infix
}

/// Infix binding power: returns (BinOp, left_bp, right_bp)
/// sigma-tau=8 precedence levels, mapped to binding power pairs
///
/// n6: Pratt binding powers = sequential pairs from σ-τ=8 levels.
/// Level L → (2L-1, 2L), max level = σ-τ=8, max bp = 2·(σ-τ)=φ·(σ-τ)=16=PHI_TAU.
fn infix_binding_power(kind: &TokenKind) -> Option<(BinOp, u8, u8)> {
    use crate::util::n6::{N, SIGMA_TAU, SIGMA_PHI, SIGMA, PHI_TAU};
    // n6: bp(L) = (2L-1, 2L) for L in 1..=SIGMA_TAU(=8)
    // Levels: MU=1(||), PHI=2(&&), N_PHI=3(cmp), TAU=4(bitor), SOPFR=5(bitand),
    //         N=6(add), N+MU=7(mul), SIGMA_TAU=8(pow)
    const BP_OR_L:   u8 = 1;  // n6: μ=1 → bp (1,2)
    const BP_OR_R:   u8 = 2;
    const BP_AND_L:  u8 = 3;  // n6: n/φ=3 → bp (3,4)
    const BP_AND_R:  u8 = 4;
    const BP_CMP_L:  u8 = 5;  // n6: sopfr=5 → bp (5,6)
    const BP_CMP_R:  u8 = 6;  // n6: n=6
    const BP_BOR_L:  u8 = (N as u8) + 1;      // n6: n+μ=7 → bp (7,8)
    const BP_BOR_R:  u8 = SIGMA_TAU as u8;     // n6: σ-τ=8
    const BP_BAND_L: u8 = (SIGMA_TAU as u8) + 1;   // n6: (σ-τ)+μ=9 → bp (9,10)
    const BP_BAND_R: u8 = SIGMA_PHI as u8;     // n6: σ-φ=10
    const BP_ADD_L:  u8 = (SIGMA_PHI as u8) + 1;   // n6: (σ-φ)+μ=11 → bp (11,12)
    const BP_ADD_R:  u8 = SIGMA as u8;         // n6: σ=12
    const BP_MUL_L:  u8 = (SIGMA as u8) + 1;       // n6: σ+μ=13 → bp (13,14)
    const BP_MUL_R:  u8 = (SIGMA as u8) + 2;       // n6: σ+φ=14
    const BP_POW_L:  u8 = PHI_TAU as u8;       // n6: 2^τ=16 (right-assoc: swap L/R)
    const BP_POW_R:  u8 = (PHI_TAU as u8) - 1; // n6: 2^τ-μ=15
    match kind {
        // Level 1: ||
        TokenKind::Or      => Some((BinOp::Or, BP_OR_L, BP_OR_R)),
        // Level 2: &&
        TokenKind::And     => Some((BinOp::And, BP_AND_L, BP_AND_R)),
        // Level 3: == != < > <= >=
        TokenKind::Eq      => Some((BinOp::Eq, BP_CMP_L, BP_CMP_R)),
        TokenKind::Neq     => Some((BinOp::Neq, BP_CMP_L, BP_CMP_R)),
        TokenKind::Lt      => Some((BinOp::Lt, BP_CMP_L, BP_CMP_R)),
        TokenKind::Gt      => Some((BinOp::Gt, BP_CMP_L, BP_CMP_R)),
        TokenKind::Le      => Some((BinOp::Le, BP_CMP_L, BP_CMP_R)),
        TokenKind::Ge      => Some((BinOp::Ge, BP_CMP_L, BP_CMP_R)),
        // Level 4: | ~
        TokenKind::BitOr   => Some((BinOp::BitOr, BP_BOR_L, BP_BOR_R)),
        TokenKind::BitXor  => Some((BinOp::BitXor, BP_BOR_L, BP_BOR_R)),
        // Level 5: &
        TokenKind::BitAnd  => Some((BinOp::BitAnd, BP_BAND_L, BP_BAND_R)),
        // Level 6: + -
        TokenKind::Plus    => Some((BinOp::Add, BP_ADD_L, BP_ADD_R)),
        TokenKind::Minus   => Some((BinOp::Sub, BP_ADD_L, BP_ADD_R)),
        // Level 7: * / %
        TokenKind::Star    => Some((BinOp::Mul, BP_MUL_L, BP_MUL_R)),
        TokenKind::Slash   => Some((BinOp::Div, BP_MUL_L, BP_MUL_R)),
        TokenKind::Percent => Some((BinOp::Mod, BP_MUL_L, BP_MUL_R)),
        // Level 8: ^ (right-associative — bp swapped)
        TokenKind::Caret   => Some((BinOp::Pow, BP_POW_L, BP_POW_R)),
        // Range
        TokenKind::DotDot  => Some((BinOp::Range, 0, 1)),
        TokenKind::DotDotEq => Some((BinOp::RangeInclusive, 0, 1)),
        _ => None,
    }
}
