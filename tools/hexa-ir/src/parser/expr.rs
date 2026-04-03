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
    parse_expr_bp(p, 0)
}

/// Pratt parser with minimum binding power
fn parse_expr_bp(p: &mut Parser, min_bp: u8) -> Result<Expr, ParseError> {
    let mut lhs = parse_prefix(p)?;

    loop {
        // Check for postfix operations: call, field, index
        lhs = match p.peek() {
            TokenKind::LParen => parse_call(p, lhs)?,
            TokenKind::Dot => parse_field_access(p, lhs)?,
            TokenKind::LBracket => parse_index(p, lhs)?,
            _ => lhs,
        };

        // Infix binary operators
        let (op, l_bp, r_bp) = match infix_binding_power(p.peek()) {
            Some(triple) => triple,
            None => break,
        };

        if l_bp < min_bp {
            break;
        }

        p.advance(); // consume operator token
        let rhs = parse_expr_bp(p, r_bp)?;
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
fn parse_prefix(p: &mut Parser) -> Result<Expr, ParseError> {
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
            // Check for struct initialization: `Name { field: val }`
            if p.at(&TokenKind::LBrace) {
                parse_struct_init(p, name, span)
            } else {
                Ok(Expr::Ident(name, span))
            }
        }
        TokenKind::Minus => {
            p.advance();
            let operand = parse_expr_bp(p, prefix_bp())?;
            let full_span = span.merge(operand.span());
            Ok(Expr::Unary {
                op: UnOp::Neg,
                operand: Box::new(operand),
                span: full_span,
            })
        }
        TokenKind::Not => {
            p.advance();
            let operand = parse_expr_bp(p, prefix_bp())?;
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
        other => Err(ParseError::new(
            span,
            format!("expected expression, found {:?}", other),
        )),
    }
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
        _ => None,
    }
}
