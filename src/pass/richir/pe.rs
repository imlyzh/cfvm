
// partial evaluation


use core::panic;
use std::sync::Arc;

use crate::pass::richir::Context;

use crate::cfir::richir::{LetBinding, Expr, Value, Fun, Call};


trait Pe {
    type Target;
    fn pe(&self, ctx: Context) -> Self::Target;
}

impl Pe for LetBinding {
    type Target = Expr;
    fn pe(&self, ctx: Context) -> Self::Target {
        let (name, value, type_) = &self.bind;
        let value = value.pe(ctx.clone());
        if let Some(_t) = type_ {
            // todo: type check
        }
        let ctx = ctx.new_level();
        let boxed_value = Arc::new(value);
        ctx.set_local(name, boxed_value.clone());
        let body = self.body.pe(ctx);
        // fixme: if expr is a constant, we can evaluate the body immediately
        if boxed_value.is_literal() {
            body
        } else {
            Expr::Let(Arc::new(LetBinding {
                bind: (name.clone(), boxed_value, type_.clone()),
                body: Arc::new(body),
            }))
        }
    }
}

impl Pe for Expr {
    type Target = Expr;
    fn pe(&self, ctx: Context) -> Self::Target {
        match self {
            Expr::Let(l) => l.pe(ctx),
            Expr::If(cond, then, els) => {
                let cond = cond.pe(ctx.clone());
                if !cond.is_literal() {
                    let then = then.pe(ctx.clone());
                    let els = els.pe(ctx);
                    return Expr::If(Arc::new(cond), Arc::new(then), Arc::new(els));
                }
                let cond = if let Some(c) = cond.get_bool_lit() {
                    c
                } else {
                    panic!("TypeError: `If` cond need boolean value");
                };
                let expr = if cond { then } else { els };
                expr.pe(ctx)
            },
            Expr::While(cond, body, accum) => {
                let cond = cond.pe(ctx.clone());
                if !cond.is_literal() {
                    let body = body.pe(ctx.clone());
                    let accum = accum.pe(ctx);
                    return Expr::While(Arc::new(cond), Arc::new(body), Arc::new(accum));
                }
                let c = if let Some(c) = cond.get_bool_lit() {
                    c
                } else {
                    panic!("TypeError: `While` cond need boolean value");
                };
                if !c {
                    return Expr::Begin(vec![]);
                }
                let body = body.pe(ctx.clone());
                let accum = accum.pe(ctx);
                Expr::While(Arc::new(cond), Arc::new(body), Arc::new(accum))
            },
            Expr::Begin(b) => Expr::Begin(b.iter().map(|e| e.pe(ctx.clone())).collect()),
            Expr::Store(var, value) => {
                let value = value.pe(ctx);
                Expr::Store(var.clone(), Arc::new(value)) // todo
            },
            Expr::Val(v) => Expr::Val(v.pe(ctx)),
        }
    }
}

impl Pe for Value {
    type Target = Value;
    fn pe(&self, ctx: Context) -> Self::Target {
        match self {
            Value::Var(s) => todo!(),
            Value::Call(call) => call.pe(ctx),
            Value::Fun(f) => Value::Fun(Arc::new(f.pe(ctx))),
            Value::Lit(_) => self.clone(),
        }
    }
}

impl Pe for Call {
    type Target = Value;
    fn pe(&self, ctx: Context) -> Self::Target {
        todo!()
    }
}

impl Pe for Fun {
    type Target = Fun;
    fn pe(&self, ctx: Context) -> Self::Target {
        Fun {
            params: self.params.clone(),
            body: Arc::new(self.body.pe(ctx)),
        }
    }
}