/// Declaration parser — tau=4 declaration kinds
///
/// Parses top-level declarations: fn, struct, enum, type alias

use crate::lexer::{TokenKind, Span};
use super::ast::*;
use super::error::{Parser, ParseError};
use super::stmt::{parse_block, parse_type_expr};

/// Parse a single top-level declaration
pub fn parse_decl(p: &mut Parser) -> Result<Decl, ParseError> {
    // Check for `pub` visibility modifier
    let vis = if p.at(&TokenKind::Pub) {
        p.advance();
        Visibility::Public
    } else {
        Visibility::Private
    };

    match p.peek() {
        TokenKind::Fn => {
            let mut f = parse_fn_decl(p)?;
            f.vis = vis;
            Ok(Decl::FnDecl(f))
        }
        TokenKind::Struct => parse_struct_decl(p).map(Decl::StructDecl),
        TokenKind::Enum => parse_enum_decl(p).map(Decl::EnumDecl),
        TokenKind::Type => parse_type_alias(p).map(Decl::TypeAlias),
        TokenKind::Mod => parse_module_decl(p).map(Decl::ModuleDecl),
        TokenKind::Use => parse_use_decl(p).map(Decl::UseDecl),
        TokenKind::Trait => parse_trait_decl(p).map(Decl::TraitDecl),
        TokenKind::Impl => parse_impl_block(p).map(Decl::ImplBlock),
        _ => {
            let span = p.peek_span();
            Err(ParseError::new(span, format!(
                "expected declaration (fn/struct/enum/type/trait/impl), found {:?}", p.peek()
            )))
        }
    }
}

/// `fn name(param: Type, ...) [-> RetType] { body }`
fn parse_fn_decl(p: &mut Parser) -> Result<FnDecl, ParseError> {
    let start = p.peek_span();
    p.expect(&TokenKind::Fn)?;
    let (name, _) = p.expect_ident()?;

    // Optional generic type parameters: `<T, U>`
    let type_params = if p.eat(&TokenKind::Lt) {
        let mut tps = Vec::new();
        while !p.at(&TokenKind::Gt) && !p.is_eof() {
            let (tp_name, _) = p.expect_ident()?;
            tps.push(tp_name);
            if !p.eat(&TokenKind::Comma) {
                break;
            }
        }
        p.expect(&TokenKind::Gt)?;
        tps
    } else {
        Vec::new()
    };

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

    Ok(FnDecl { name, vis: Visibility::Private, type_params, params, ret_ty, body, span })
}

/// `mod name { decl* }`
fn parse_module_decl(p: &mut Parser) -> Result<ModuleDecl, ParseError> {
    let start = p.peek_span();
    p.expect(&TokenKind::Mod)?;
    let (name, _) = p.expect_ident()?;

    p.expect(&TokenKind::LBrace)?;
    let mut decls = Vec::new();
    while !p.at(&TokenKind::RBrace) && !p.is_eof() {
        match parse_decl(p) {
            Ok(d) => decls.push(d),
            Err(e) => p.error_and_sync(e),
        }
    }
    let end = p.peek_span();
    p.expect(&TokenKind::RBrace)?;

    Ok(ModuleDecl { name, decls, span: start.merge(end) })
}

/// `use path::to::item;`
fn parse_use_decl(p: &mut Parser) -> Result<UseDecl, ParseError> {
    let start = p.peek_span();
    p.expect(&TokenKind::Use)?;

    let mut path = Vec::new();
    let (first, _) = p.expect_ident()?;
    path.push(first);

    // Parse `::segment` chains
    while p.eat(&TokenKind::ColonColon) {
        let (seg, _) = p.expect_ident()?;
        path.push(seg);
    }

    let end = p.peek_span();
    p.eat(&TokenKind::Semicolon);

    Ok(UseDecl { path, span: start.merge(end) })
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

/// `trait Name { fn method(&self, param: Type) -> RetType; ... }`
fn parse_trait_decl(p: &mut Parser) -> Result<TraitDecl, ParseError> {
    let start = p.peek_span();
    p.expect(&TokenKind::Trait)?;
    let (name, _) = p.expect_ident()?;
    p.expect(&TokenKind::LBrace)?;
    let mut methods = Vec::new();
    while !p.at(&TokenKind::RBrace) && !p.is_eof() {
        methods.push(parse_trait_method_sig(p)?);
    }
    let end = p.peek_span();
    p.expect(&TokenKind::RBrace)?;
    Ok(TraitDecl { name, methods, span: start.merge(end) })
}

/// Parse trait method signature: `fn name(&self, p: T, ...) [-> R];`
fn parse_trait_method_sig(p: &mut Parser) -> Result<TraitMethodSig, ParseError> {
    let start = p.peek_span();
    p.expect(&TokenKind::Fn)?;
    let (name, _) = p.expect_ident()?;
    p.expect(&TokenKind::LParen)?;
    // Skip &self or self
    if p.at(&TokenKind::BitAnd) {
        p.advance();
        let _ = p.expect_ident()?;
        p.eat(&TokenKind::Comma);
    } else if let TokenKind::Ident(ref s) = p.peek().clone() {
        if s == "self" { p.advance(); p.eat(&TokenKind::Comma); }
    }
    let mut params = Vec::new();
    while !p.at(&TokenKind::RParen) && !p.is_eof() {
        let (pn, _) = p.expect_ident()?;
        p.expect(&TokenKind::Colon)?;
        let pt = parse_type_expr(p)?;
        params.push((pn, pt));
        if !p.eat(&TokenKind::Comma) { break; }
    }
    p.expect(&TokenKind::RParen)?;
    let ret_ty = if p.eat(&TokenKind::Arrow) { Some(parse_type_expr(p)?) } else { None };
    let end = p.peek_span();
    p.eat(&TokenKind::Semicolon);
    Ok(TraitMethodSig { name, params, ret_ty, span: start.merge(end) })
}

/// `impl TraitName for TypeName { fn method(&self) { ... } ... }`
fn parse_impl_block(p: &mut Parser) -> Result<ImplBlock, ParseError> {
    let start = p.peek_span();
    p.expect(&TokenKind::Impl)?;
    let (trait_name, _) = p.expect_ident()?;
    p.expect(&TokenKind::For)?;
    let (target_type, _) = p.expect_ident()?;
    p.expect(&TokenKind::LBrace)?;
    let mut methods = Vec::new();
    while !p.at(&TokenKind::RBrace) && !p.is_eof() {
        methods.push(parse_impl_method(p)?);
    }
    let end = p.peek_span();
    p.expect(&TokenKind::RBrace)?;
    Ok(ImplBlock { trait_name, target_type, methods, span: start.merge(end) })
}

/// Parse a method inside impl block (fn with possible &self first param)
fn parse_impl_method(p: &mut Parser) -> Result<FnDecl, ParseError> {
    let start = p.peek_span();
    p.expect(&TokenKind::Fn)?;
    let (name, _) = p.expect_ident()?;
    p.expect(&TokenKind::LParen)?;
    let mut params = Vec::new();
    // Check for &self or self
    if p.at(&TokenKind::BitAnd) {
        p.advance();
        let _ = p.expect_ident()?; // "self"
        params.push(("self".to_string(), TypeExpr::Named("Self".to_string(), start)));
        p.eat(&TokenKind::Comma);
    } else if let TokenKind::Ident(ref s) = p.peek().clone() {
        if s == "self" {
            p.advance();
            params.push(("self".to_string(), TypeExpr::Named("Self".to_string(), start)));
            p.eat(&TokenKind::Comma);
        }
    }
    while !p.at(&TokenKind::RParen) && !p.is_eof() {
        let (pn, _) = p.expect_ident()?;
        p.expect(&TokenKind::Colon)?;
        let pt = parse_type_expr(p)?;
        params.push((pn, pt));
        if !p.eat(&TokenKind::Comma) { break; }
    }
    p.expect(&TokenKind::RParen)?;
    let ret_ty = if p.eat(&TokenKind::Arrow) { Some(parse_type_expr(p)?) } else { None };
    let body = parse_block(p)?;
    let span = start.merge(body.span);
    Ok(FnDecl { name, vis: Visibility::Private, type_params: vec![], params, ret_ty, body, span })
}
