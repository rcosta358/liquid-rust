use z3::{ast::{Ast, Bool, Real}, Config, Context, SatResult, Solver};
use parser::ast::{BinOp, Expr, UnOp};

pub fn check_refinement(refinement: &Expr, value: &syn::Expr) -> bool {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);
    let var = Real::new_const(&ctx, "_");

    // translate ast to z3 boolean
    let z3_result = to_z3(&ctx, refinement, &var);

    // assign the value as a z3 constant to the "_" variable
    if let Some(val_ast) = value_as_real(&ctx, value) {
        solver.assert(&var._eq(&val_ast));
    }

    // assert the result of the refinement
    solver.assert(&z3_result);
    solver.check() != SatResult::Unsat
}

fn to_z3<'ctx>(ctx: &'ctx Context, expr: &Expr, var: &Real<'ctx>) -> Bool<'ctx> {
    match expr {
        Expr::Binary(left, op, right) => {
            match op {
                BinOp::And => {
                    let l = to_z3(ctx, left, var);
                    let r = to_z3(ctx, right, var);
                    Bool::and(ctx, &[&l, &r])
                }
                BinOp::Or => {
                    let l = to_z3(ctx, left, var);
                    let r = to_z3(ctx, right, var);
                    Bool::or(ctx, &[&l, &r])
                }
                BinOp::Eq => to_real(ctx, left, var)._eq(&to_real(ctx, right, var)),
                BinOp::Neq => to_real(ctx, left, var)._eq(&to_real(ctx, right, var)).not(),
                BinOp::Gt => to_real(ctx, left, var).gt(&to_real(ctx, right, var)),
                BinOp::Ge => to_real(ctx, left, var).ge(&to_real(ctx, right, var)),
                BinOp::Lt => to_real(ctx, left, var).lt(&to_real(ctx, right, var)),
                BinOp::Le => to_real(ctx, left, var).le(&to_real(ctx, right, var)),
                _ => Bool::from_bool(ctx, true),
            }
        }
        Expr::Unary(op, inner) => {
           match op {
                UnOp::Not => to_z3(ctx, inner, var).not(),
                UnOp::Neg => {
                    let inner_real = to_real(ctx, inner, var);
                    let zero = Real::from_real(ctx, 0, 1);
                    inner_real._eq(&zero).not()
                }
           }
        }
        Expr::Conditional(cond, then, els) => {
            let cond_bool = to_z3(ctx, cond, var);
            let then_bool = to_z3(ctx, then, var);
            let else_bool = to_z3(ctx, els, var);
            Bool::ite(&cond_bool, &then_bool, &else_bool)
        }
        Expr::Bool(b) => Bool::from_bool(ctx, *b),
        _ => Bool::from_bool(ctx, true),
    }
}

fn to_real<'ctx>(ctx: &'ctx Context, expr: &Expr, var: &Real<'ctx>) -> Real<'ctx> {
    match expr {
        Expr::Id(name) if name == "_" => var.clone(),
        Expr::Number(n) => Real::from_real(ctx, (*n) as i32, 1),
        Expr::Unary(UnOp::Neg, inner) => {
            let inner_real = to_real(ctx, inner, var);
            inner_real.unary_minus()
        }
        _ => Real::from_real(ctx, 0, 1),
    }
}

fn value_as_real<'ctx>(ctx: &'ctx Context, value: &syn::Expr) -> Option<Real<'ctx>> {
    match value {
        syn::Expr::Lit(lit) => match &lit.lit {
            syn::Lit::Int(n) => Some(Real::from_real(ctx, n.base10_parse::<i32>().unwrap(), 1)),
            _ => None,
        },
        syn::Expr::Unary(expr_unary) => { // negative numbers
            if let syn::UnOp::Neg(_) = expr_unary.op {
                if let syn::Expr::Lit(lit) = &*expr_unary.expr {
                    if let syn::Lit::Int(n) = &lit.lit {
                        let val = n.base10_parse::<i32>().unwrap();
                        return Some(Real::from_real(ctx, -val, 1));
                    }
                }
            }
            None
        }
        _ => None,
    }
}
