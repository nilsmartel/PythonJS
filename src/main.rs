use rustpython_parser::{self, Mode};
use rustpython_parser::ast;

fn main() {
    let source_path = ".";
    let file = "some_path";
    let source = std::fs::read_to_string(file).expect("read file");

    let ast = rustpython_parser::parse(&source, Mode::Module, source_path);
    let ast = ast.expect("valid python programm");

    let module = match ast {
        ast::Mod::Module(m) => m,
        _ => panic!("expected module"),
    };

    for statement in module.body {
        match statement {
            ast::Stmt::FunctionDef(f) => s_functiondef(f),
            ast::Stmt::AsyncFunctionDef(_) => todo!(),
            ast::Stmt::ClassDef(_) => todo!(),
            ast::Stmt::Return(_) => todo!(),
            ast::Stmt::Delete(_) => todo!(),
            ast::Stmt::Assign(_) => todo!(),
            ast::Stmt::TypeAlias(_) => todo!(),
            ast::Stmt::AugAssign(_) => todo!(),
            ast::Stmt::AnnAssign(_) => todo!(),
            ast::Stmt::For(_) => todo!(),
            ast::Stmt::AsyncFor(_) => todo!(),
            ast::Stmt::While(_) => todo!(),
            ast::Stmt::If(_) => todo!(),
            ast::Stmt::With(_) => todo!(),
            ast::Stmt::AsyncWith(_) => todo!(),
            ast::Stmt::Match(_) => todo!(),
            ast::Stmt::Raise(_) => todo!(),
            ast::Stmt::Try(_) => todo!(),
            ast::Stmt::TryStar(_) => todo!(),
            ast::Stmt::Assert(_) => todo!(),
            ast::Stmt::Import(_) => todo!(),
            ast::Stmt::ImportFrom(_) => todo!(),
            ast::Stmt::Global(_) => todo!(),
            ast::Stmt::Nonlocal(_) => todo!(),
            ast::Stmt::Expr(_) => todo!(),
            ast::Stmt::Pass(_) => todo!(),
            ast::Stmt::Break(_) => todo!(),
            ast::Stmt::Continue(_) => todo!(),
        }
    }

}

fn s_functiondef(f: ast::StmtFunctionDef) -> String {
    // TODO decorators
    let ast::StmtFunctionDef{
        range,
        name, args, body, decorator_list, returns, type_comment, type_params };

    let mut out = String::with_capacity(256);
    out.push("function ");
    out.push(name.as_str());
    out.push("(");
    // TODO are these the right arguments?
    let args = args.posonlyargs.into_iter().map(|arg| {
        // TODO use annotation
        let output = arg.def.arg.as_str().to_owned();
        if let Some(expr) = arg.default {
            let expr = s_expr(expr);
            output.push("=");
            output.push_str(expr.as_str());
        }

        output
    }).join(", ");

    out.push_str(&args);

    let body = s_body(body);

    out.push(&body);

    out
}

fn s_body(body: Vec<ast::Stmt<_>>) -> String {
    todo!()
}

fn s_expr(expr: Box<ast::Expr<_>>) -> String {
    todo!()
}
