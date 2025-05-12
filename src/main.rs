use itertools::join;
use rustpython_parser::ast::*;
use rustpython_parser::{self, Mode};

fn main() {
    let filepath = std::env::args().nth(1).unwrap_or("/dev/stdin".into());
    let source_path = ".";
    let ast = read_ast(source_path, &filepath);
    let ast = ast.expect("valid python programm");

    let module = match ast {
        Mod::Module(m) => m,
        _ => panic!("expected module"),
    };

    let output = s_body(&module.body);
    println!("{}", output);
}

fn read_ast(source_path: &str, filepath: &str) -> Result<Mod, rustpython_parser::ParseError> {
    let source = std::fs::read_to_string(filepath).expect("read file");

    rustpython_parser::parse(&source, Mode::Module, source_path)
}

fn s_functiondef(f: &StmtFunctionDef) -> String {
    let StmtFunctionDef {
        range,
        name,
        args,
        body,
        // TODO use decorators
        decorator_list,
        returns,
        type_comment,
        type_params,
    } = f;

    let mut out = String::with_capacity(256);
    out.push_str("function ");
    out.push_str(name.as_str());
    out.push_str("(");
    // TODO are these the right arguments?
    let args = args.posonlyargs.iter().map(|arg| {
        // TODO use annotation
        let mut output = arg.def.arg.as_str().to_owned();
        if let Some(expr) = &arg.default {
            let expr = s_expr(&expr);
            output.push('=');
            output.push_str(expr.as_str());
        }

        output
    });
    let args = join(args, ",");

    out.push_str(&args);

    out.push(')');
    out.push('{');

    let body = s_body(body);
    out.push_str(&body);
    out.push('}');

    out
}

fn s_body(body: &Vec<Stmt>) -> String {
    let elems = body.into_iter().map(s_stmt);

    join(elems, "\n")
}

fn s_stmt(body: &Stmt) -> String {
    match body {
        Stmt::FunctionDef(stmt_function_def) => s_functiondef(stmt_function_def),
        Stmt::Expr(ref stmt_expr) => s_expr(&stmt_expr.value),
        Stmt::AsyncFunctionDef(stmt_async_function_def) => todo(),
        Stmt::ClassDef(stmt_class_def) => todo(),
        Stmt::Return(stmt_return) => todo(),
        Stmt::Delete(stmt_delete) => todo(),
        Stmt::Assign(stmt_assign) => todo(),
        Stmt::TypeAlias(stmt_type_alias) => todo(),
        Stmt::AugAssign(stmt_aug_assign) => todo(),
        Stmt::AnnAssign(stmt_ann_assign) => todo(),
        Stmt::For(stmt_for) => todo(),
        Stmt::AsyncFor(stmt_async_for) => todo(),
        Stmt::While(stmt_while) => todo(),
        Stmt::If(stmt_if) => todo(),
        Stmt::With(stmt_with) => todo(),
        Stmt::AsyncWith(stmt_async_with) => todo(),
        Stmt::Match(stmt_match) => todo(),
        Stmt::Raise(stmt_raise) => todo(),
        Stmt::Try(stmt_try) => todo(),
        Stmt::TryStar(stmt_try_star) => todo(),
        Stmt::Assert(stmt_assert) => todo(),
        Stmt::Import(stmt_import) => todo(),
        Stmt::ImportFrom(stmt_import_from) => todo(),
        Stmt::Global(stmt_global) => todo(),
        Stmt::Nonlocal(stmt_nonlocal) => todo(),
        Stmt::Pass(stmt_pass) => todo(),
        Stmt::Break(stmt_break) => todo(),
        Stmt::Continue(stmt_continue) => todo(),
    }
}

fn s_expr(expr: &Expr) -> String {
    match expr {
        Expr::BoolOp(op) => {
            let joiner = match op.op {
                BoolOp::And => "&&",
                BoolOp::Or => "||",
            };

            let x = op.values.iter().map(|expr| s_expr(expr));
            join(x, joiner)
        }
        Expr::NamedExpr(_) => todo(),
        Expr::BinOp(_) => todo(),
        Expr::UnaryOp(_) => todo(),
        Expr::Lambda(_) => todo(),
        Expr::IfExp(_) => todo(),
        Expr::Dict(_) => todo(),
        Expr::Set(_) => todo(),
        Expr::ListComp(_) => todo(),
        Expr::SetComp(_) => todo(),
        Expr::DictComp(_) => todo(),
        Expr::GeneratorExp(_) => todo(),
        Expr::Await(_) => todo(),
        Expr::Yield(_) => todo(),
        Expr::YieldFrom(_) => todo(),
        Expr::Compare(_) => todo(),
        Expr::Call(_) => todo(),
        Expr::FormattedValue(_) => todo(),
        Expr::JoinedStr(_) => todo(),
        Expr::Constant(_) => todo(),
        Expr::Attribute(_) => todo(),
        Expr::Subscript(_) => todo(),
        Expr::Starred(_) => todo(),
        Expr::Name(_) => todo(),
        Expr::List(_) => todo(),
        Expr::Tuple(_) => todo(),
        Expr::Slice(_) => todo(),
    }
}

fn todo<T: Default>() -> T {
    Default::default()
}
