/// Declaration parser — tau=4 declaration kinds
///
/// Parses top-level declarations: fn, struct, enum, type alias

use crate::lexer::{TokenKind, Span};
use super::ast::*;
use super::error::{Parser, ParseError};
use super::stmt::{parse_block, parse_type_expr};

/// Parse a single top-level declaration
pub fn parse_decl(p: &mut Parser) -> Result<Decl, ParseError> {
    match p.peek() {
        TokenKind::Fn => parse_fn_decl(p).map(Decl::FnDecl),
        TokenKind::Struct => parse_struct_decl(p).map(Decl::StructDecl),
        TokenKind::Enum => parse_enum_decl(p).map(Decl::EnumDecl),
        TokenKind::Type => parse_type_alias(p).map(Decl::TypeAlias),
        _ => {
            let span = p.peek_span();
            Err(ParseError::new(span, format!(
                "expected declaration (fn/struct/enum/type), found {:?}", p.peek()
            )))
        }
    }
}

/// `fn name(param: Type, ...) [-> RetType] { body }`
fn parse_fn_decl(p: &mut Parser) -> Result<FnDecl, ParseError> {
    let start = p.peek_span();
    p.expect(&TokenKind::Fn)?;
    let (name, _) = p.expect_ident()?;

    // Parameters
    p.expect(&TokenKind::LParen)?;
    let mut params = Vec::new();
    while !p.at(&TokenKind::RParen) && !p.is_eof() {
        let (pname, _) = p.expect_ident()?;
        p.expect(&TokenKind::Colon)?;
        let pty = parse_type_expr(p)?;
        params.push((pname, pty));
        if !p.eat(&TokenKind::Comma) {
            break;
        }
    }
    p.expect(&TokenKind::RParen)?;

    // Optional return type
    let ret_ty = if p.eat(&TokenKind::Arrow) {
        Some(parse_type_expr(p)?)
    } else {
        None
    };

    // Body
    let body = parse_block(p)?;
    let span = start.merge(body.span);

    Ok(FnDecl { name, params, ret_ty, body, span })
}

/// `struct Name { field1: Type, field2: Type, ... }`
fn parse_struct_decl(p: &mut Parser) -> Result<StructDecl, ParseError> {
    let start = p.peek_span();
    p.expect(&TokenKind::Struct)?;
    let (name, _) = p.expect_ident()?;

    p.expect(&TokenKind::LBrace)?;
    let mut fields = Vec::new();
    while !p.at(&TokenKind::RBrace) && !p.is_eof() {
        let (fname, _) = p.expect_ident()?;
        p.expect(&TokenKind::Colon)?;
        let fty = parse_type_expr(p)?;
        fields.push((fname, fty));
        if !p.eat(&TokenKind::Comma) {
            break;
        }
    }
    let end = p.peek_span();
    p.expect(&TokenKind::RBrace)?;

    Ok(StructDecl { name, fields, span: start.merge(end) })
}

/// `enum Name { Variant1, Variant2(Type, ...), ... }`
fn parse_enum_decl(p: &mut Parser) -> Result<EnumDecl, ParseError> {
    let start = p.peek_span();
    p.expect(&TokenKind::Enum)?;
    let (name, _) = p.expect_ident()?;

    p.expect(&TokenKind::LBrace)?;
    let mut variants = Vec::new();
    while !p.at(&TokenKind::RBrace) && !p.is_eof() {
        let (vname, _) = p.expect_ident()?;

        // Optional tuple payload: Variant(Type, Type)
        let payload = if p.eat(&TokenKind::LParen) {
            let mut types = Vec::new();
            while !p.at(&TokenKind::RParen) && !p.is_eof() {
                types.push(parse_type_expr(p)?);
                if !p.eat(&TokenKind::Comma) {
                    break;
                }
            }
            p.expect(&TokenKind::RParen)?;
            Some(types)
        } else {
            None
        };

        variants.push((vname, payload));
        if !p.eat(&TokenKind::Comma) {
            break;
        }
    }
    let end = p.peek_span();
    p.expect(&TokenKind::RBrace)?;

    Ok(EnumDecl { name, variants, span: start.merge(end) })
}

/// `type Name = Type;`
fn parse_type_alias(p: &mut Parser) -> Result<TypeAliasDecl, ParseError> {
    let start = p.peek_span();
    p.expect(&TokenKind::Type)?;
    let (name, _) = p.expect_ident()?;
    p.expect(&TokenKind::Assign)?;
    let ty = parse_type_expr(p)?;
    let end = p.peek_span();
    // Optional semicolon (Go/Kotlin style)
    p.eat(&TokenKind::Semicolon);

    Ok(TypeAliasDecl { name, ty, span: start.merge(end) })
}
