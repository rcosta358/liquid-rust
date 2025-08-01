use z3::{ast::{Ast, Bool, Real}, Config, Context, SatResult, Solver};
use parser::ast::{BinOp, Expr, UnOp};

pub fn check_refinement(refinement: &Expr, value: &syn::Expr) -> Result<(), String> {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);
    let var = Real::new_const(&ctx, "_");

    // translate ast to z3 boolean
    let z3_result = to_z3_bool(&ctx, refinement, &var)?;

    // assign the value as a z3 constant to the "_" variable
    if let Some(val_ast) = value_as_real(&ctx, value) {
        solver.assert(&var._eq(&val_ast));
    }

    // assert and check the result of the refinement
    solver.assert(&z3_result);
    if solver.check() == SatResult::Unsat {
        Err("Value does not satisfy the refinement".into())
    } else {
        Ok(())
    }
}

fn to_z3_bool<'ctx>(ctx: &'ctx Context, expr: &Expr, var: &Real<'ctx>) -> Result<Bool<'ctx>, String> {
    match expr {
        Expr::Binary(left, op, right) => {
            let res = match op {
                BinOp::And => {
                    let l = to_z3_bool(ctx, left, var)?;
                    let r = to_z3_bool(ctx, right, var)?;
                    Bool::and(ctx, &[&l, &r])
                }
                BinOp::Or => {
                    let l = to_z3_bool(ctx, left, var)?;
                    let r = to_z3_bool(ctx, right, var)?;
                    Bool::or(ctx, &[&l, &r])
                }
                BinOp::Eq => to_real(ctx, left, var)._eq(&to_real(ctx, right, var)),
                BinOp::Neq => to_real(ctx, left, var)._eq(&to_real(ctx, right, var)).not(),
                BinOp::Gt => to_real(ctx, left, var).gt(&to_real(ctx, right, var)),
                BinOp::Ge => to_real(ctx, left, var).ge(&to_real(ctx, right, var)),
                BinOp::Lt => to_real(ctx, left, var).lt(&to_real(ctx, right, var)),
                BinOp::Le => to_real(ctx, left, var).le(&to_real(ctx, right, var)),
                BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod => return Err("Non-boolean expression".into()),
            };
            Ok(res)
        }
        Expr::Unary(op, inner) => {
            let res = match op {
                UnOp::Not => to_z3_bool(ctx, inner, var)?.not(),
                UnOp::Neg => {
                    let inner_real = to_real(ctx, inner, var);
                    let zero = from_real(ctx, 0);
                    inner_real._eq(&zero).not()
                }
            };
            Ok(res)
        }
        Expr::Conditional(cond, then, els) => {
            let cond_bool = to_z3_bool(ctx, cond, var)?;
            let then_bool = to_z3_bool(ctx, then, var)?;
            let else_bool = to_z3_bool(ctx, els, var)?;
            let res = Bool::ite(&cond_bool, &then_bool, &else_bool);
            Ok(res)
        }
        Expr::Bool(b) => Ok(Bool::from_bool(ctx, *b)),
        _ => Err("Non-boolean expression".into()),
    }
}

fn to_real<'ctx>(ctx: &'ctx Context, expr: &Expr, var: &Real<'ctx>) -> Real<'ctx> {
    match expr {
        Expr::Id => var.clone(),
        Expr::Int(n) => from_real(ctx, (*n) as i32),
        Expr::Unary(UnOp::Neg, inner) => {
            let value = to_real(ctx, inner, var);
            Real::sub(ctx, &[&from_real(ctx, 0), &value])
        }
        Expr::Binary(left, op, right) => {
            let l = to_real(ctx, left, var);
            let r = to_real(ctx, right, var);
            match op {
                BinOp::Add => Real::add(ctx, &[&l, &r]),
                BinOp::Sub => Real::sub(ctx, &[&l, &r]),
                BinOp::Mul => Real::mul(ctx, &[&l, &r]),
                BinOp::Div => l.div(&r),
                BinOp::Mod => {
                    let frac = l.div(&r);
                    let floored_int = z3::ast::Int::from_real(&frac);
                    let floored_r = z3::ast::Int::to_real(&floored_int);
                    Real::sub(ctx, &[&l, &Real::mul(ctx, &[&floored_r, &r])])
                }
                _ => from_real(ctx, 0)
            }
        }
        _ => from_real(ctx, 0),
    }
}

fn value_as_real<'ctx>(ctx: &'ctx Context, value: &syn::Expr) -> Option<Real<'ctx>> {
    match value {
        syn::Expr::Lit(lit) => match &lit.lit { // literals
            syn::Lit::Int(n) => Some(from_real(ctx, n.base10_parse::<i32>().unwrap())),
            _ => None,
        },
        syn::Expr::Unary(expr) => { // negative numbers
            if let syn::UnOp::Neg(_) = expr.op {
                if let syn::Expr::Lit(lit) = &*expr.expr {
                    if let syn::Lit::Int(n) = &lit.lit {
                        let val = n.base10_parse::<i32>().unwrap();
                        return Some(from_real(ctx, -val));
                    }
                }
            }
            None
        }
        _ => None,
    }
}

fn from_real<'ctx>(ctx: &'ctx Context, value: i32) -> Real<'ctx> {
    Real::from_real(ctx, value, 1)
}