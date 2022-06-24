use log::error;
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn generate() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        error!("Usage: generate_ast <output dir>");
        std::process::exit(64);
    }

    let output_file = format!("{}/{}", args[1], "ast_types.rs");

    let mut file: File = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(output_file)?;

    define_base_class_and_boilerplate(&mut file)?;
    define_visitor(&mut file)?;
    define_ast_and_types(&mut file)
}

fn define_base_class_and_boilerplate(file: &mut File) -> std::io::Result<()> {
    file.write(b"use crate::token::Token;\n\n")?;

    file.write(b"#[derive(Debug)]")?;
    file.write(b"pub enum Expr {\n")?;
    file.write(b"\tLiteral(Literal),\n")?;
    file.write(b"\tGrouping(Grouping),\n")?;
    file.write(b"\tUnary(Unary),\n")?;
    file.write(b"}\n\n")?;

    file.write(b"pub trait ExprAccept<T> {\n")?;
    file.write(b"\tfn accept(&self, visitor: &impl Visitor<T>) -> T;\n")?;
    file.write(b"}\n\n")?;

    Ok(())
}

fn define_visitor(file: &mut File) -> std::io::Result<()> {
    file.write(b"pub trait Visitor<T> {\n")?;
    file.write(b"\tfn visit_binary(&self, expr: &Binary) -> T;\n")?;
    file.write(b"\tfn visit_literal(&self, expr: &Literal) -> T;\n")?;
    file.write(b"\tfn visit_grouping(&self, expr: &Grouping) -> T;\n")?;
    file.write(b"\tfn visit_unary(&self, expr: &Unary) -> T;\n")?;
    file.write(b"}\n\n")?;

    Ok(())
}

fn define_ast_and_types(file: &mut File) -> std::io::Result<()> {
    let types = vec![
        "Binary - left: T, operator: Token, right: T",
        "Grouping - expression: T",
        "Literal - value: T",
        "Unary - operator: Token, right: T",
    ];

    for ast_type in types.iter() {
        let struct_split: Vec<&str> = ast_type.split("-").collect();
        let base_type = struct_split[0].trim();
        let struct_def = format!("pub struct {}<T> {{\n", base_type);

        file.write(struct_def.as_bytes())?;

        let attributes: Vec<&str> = struct_split[1].split(",").collect();

        for attribute in attributes.iter() {
            let attr_def = format!("\t{},\n", attribute);
            file.write(attr_def.as_bytes())?;
        }
        file.write(b"}\n\n")?;

        define_visitor_implementation(file, base_type)?;
    }

    Ok(())
}

fn define_visitor_implementation(file: &mut File, base_type: &str) -> std::io::Result<()> {
    let impl_def = format!(
        "impl<T> Visitor<{}<T>> for {}<T> {{\n",
        base_type, base_type
    );
    file.write(impl_def.as_bytes())?;
    let impl_body = format!("\tfn visit(expr: &{}<T>) {{\n", base_type);
    file.write(impl_body.as_bytes())?;

    file.write(b"\t}\n")?;
    file.write(b"}\n\n")?;

    Ok(())
}
