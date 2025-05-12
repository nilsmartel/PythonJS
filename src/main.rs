use std::any::Any;

use itertools::join;
use rustpython_parser::ast;
use rustpython_parser::{self, Mode};

fn main() {
    let filepath = std::env::args().nth(1).expect("filepath as first argument");
    let source_path = ".";
    let ast = read_ast(source_path, &filepath);
    let ast = ast.expect("valid python programm");

    let module = match ast {
        ast::Mod::Module(m) => m,
        _ => panic!("expected module"),
    };

    let output = s_body(module.body);
    println!("{}", output);
}

fn read_ast(source_path: &str, filepath: &str) -> Result<ast::Mod, rustpython_parser::ParseError> {
    let source = std::fs::read_to_string(filepath).expect("read file");

    rustpython_parser::parse(&source, Mode::Module, source_path)
}

fn s_functiondef(f: ast::StmtFunctionDef) -> String {
    // TODO decorators
    let ast::StmtFunctionDef {
        range,
        name,
        args,
        body,
        decorator_list,
        returns,
        type_comment,
        type_params,
    };

    let mut out = String::with_capacity(256);
    out.push_str("function ");
    out.push_str(name.as_str());
    out.push_str("(");
    // TODO are these the right arguments?
    let args = args
        .posonlyargs
        .into_iter()
        .map(|arg| {
            // TODO use annotation
            let mut output = arg.def.arg.as_str().to_owned();
            if let Some(expr) = arg.default {
                let expr = s_expr(expr);
                output.push('=');
                output.push_str(expr.as_str());
            }

            output
        })
        .join(",");

    out.push_str(&args);

    out.push(')');
    out.push('{');

    let body = s_body(body);
    out.push_str(&body);
    out.push('}');

    out
}

fn s_body(body: Vec<ast::Stmt<impl Any>>) -> String {
    let elems = body.into_iter().map(s_stmt);

    join(elems, "\n")
}

fn s_stmt(body: ast::Stmt<_>) -> String {
    match body {
        ast::Stmt::FunctionDef(stmt_function_def) => s_functiondef(stmt_function_def),
        ast::Stmt::Expr(stmt_expr) => s_expr(stmt_expr),
        ast::Stmt::AsyncFunctionDef(stmt_async_function_def) => todo(),
        ast::Stmt::ClassDef(stmt_class_def) => todo(),
        ast::Stmt::Return(stmt_return) => todo(),
        ast::Stmt::Delete(stmt_delete) => todo(),
        ast::Stmt::Assign(stmt_assign) => todo(),
        ast::Stmt::TypeAlias(stmt_type_alias) => todo(),
        ast::Stmt::AugAssign(stmt_aug_assign) => todo(),
        ast::Stmt::AnnAssign(stmt_ann_assign) => todo(),
        ast::Stmt::For(stmt_for) => todo(),
        ast::Stmt::AsyncFor(stmt_async_for) => todo(),
        ast::Stmt::While(stmt_while) => todo(),
        ast::Stmt::If(stmt_if) => todo(),
        ast::Stmt::With(stmt_with) => todo(),
        ast::Stmt::AsyncWith(stmt_async_with) => todo(),
        ast::Stmt::Match(stmt_match) => todo(),
        ast::Stmt::Raise(stmt_raise) => todo(),
        ast::Stmt::Try(stmt_try) => todo(),
        ast::Stmt::TryStar(stmt_try_star) => todo(),
        ast::Stmt::Assert(stmt_assert) => todo(),
        ast::Stmt::Import(stmt_import) => todo(),
        ast::Stmt::ImportFrom(stmt_import_from) => todo(),
        ast::Stmt::Global(stmt_global) => todo(),
        ast::Stmt::Nonlocal(stmt_nonlocal) => todo(),
        ast::Stmt::Pass(stmt_pass) => todo(),
        ast::Stmt::Break(stmt_break) => todo(),
        ast::Stmt::Continue(stmt_continue) => todo(),
    }
}

fn s_expr(expr: Box<ast::Expr<_>>) -> String {
    match expr {
        ast::Expr::BoolOp(op) => {
            let joiner = match op.op {
                ast::BoolOp::And => "&&",
                ast::BoolOp::Or => "||",
            };

            let x = op.values.into_iter().map(|expr| s_expr(expr));
            join(x, joiner)
        }
        ast::Expr::NamedExpr(_) => todo(),
        ast::Expr::BinOp(_) => todo(),
        ast::Expr::UnaryOp(_) => todo(),
        ast::Expr::Lambda(_) => todo(),
        ast::Expr::IfExp(_) => todo(),
        ast::Expr::Dict(_) => todo(),
        ast::Expr::Set(_) => todo(),
        ast::Expr::ListComp(_) => todo(),
        ast::Expr::SetComp(_) => todo(),
        ast::Expr::DictComp(_) => todo(),
        ast::Expr::GeneratorExp(_) => todo(),
        ast::Expr::Await(_) => todo(),
        ast::Expr::Yield(_) => todo(),
        ast::Expr::YieldFrom(_) => todo(),
        ast::Expr::Compare(_) => todo(),
        ast::Expr::Call(_) => todo(),
        ast::Expr::FormattedValue(_) => todo(),
        ast::Expr::JoinedStr(_) => todo(),
        ast::Expr::Constant(_) => todo(),
        ast::Expr::Attribute(_) => todo(),
        ast::Expr::Subscript(_) => todo(),
        ast::Expr::Starred(_) => todo(),
        ast::Expr::Name(_) => todo(),
        ast::Expr::List(_) => todo(),
        ast::Expr::Tuple(_) => todo(),
        ast::Expr::Slice(_) => todo(),
    }
}

fn todo<T: Default>() -> T {
    Default::default()
}
