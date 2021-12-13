
// partial evaluation


use core::panic;
use std::sync::Arc;

use crate::cfir::handles::{SymbolRef, Symbol};
use crate::pass::richir::Context;

use crate::cfir::richir::{LetBinding, Expr, Value, Fun, Call, Literal};


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
        // fixme: if expr is a constant, we can evaluate the body immediately
        if value.is_literal() {
            ctx.set_local(name, &value);
            self.body.pe(ctx)
        } else {
            let body = self.body.pe(ctx);
            Expr::Let(LetBinding {
                bind: (name.clone(), value, type_.clone()),
                body: Box::new(body),
            })
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
                    return Expr::If(cond, Box::new(then), Box::new(els));
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
                    return Expr::While(cond, Box::new(body), Box::new(accum));
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
                Expr::While(cond, Box::new(body), Box::new(accum))
            },
            Expr::Begin(b) => Expr::Begin(b.iter().map(|e| e.pe(ctx.clone())).collect()),
            Expr::Store(var, value) => {
                let value = value.pe(ctx);
                Expr::Store(var.clone(), Box::new(value)) // todo
            },
            Expr::Val(v) => Expr::Val(v.pe(ctx)),
        }
    }
}

impl Pe for Value {
    type Target = Value;
    fn pe(&self, ctx: Context) -> Self::Target {
        match self {
            Value::Var(s) => s.pe(ctx),
            Value::Call(call) => call.pe(ctx),
            Value::Fun(f) => Value::Fun(Arc::new(f.pe(ctx))),
            Value::Lit(_) => self.clone(),
        }
    }
}

impl Pe for Call {
    type Target = Value;
    fn pe(&self, ctx: Context) -> Self::Target {
        let fun = self.fun.pe(ctx.clone());
        let args = self.args.iter().map(|x| x.pe(ctx.clone()));
        if !fun.is_literal() {
            return Value::Call(Call {
                fun: Box::new(fun),
                args: args.collect(),
            });
        }
        if let Some(SymbolRef::Symbol(s)) = fun.get_symbol() {
            return builtin_function_call_pe(s, &args.collect::<Vec<Value>>());
        }
        let fun = fun.get_fun();
        if fun.is_none() {
            panic!("TypeError: `Call` fun need function value");
        };
        let fun = fun.unwrap();
        if fun.args.len() != args.len() {
            panic!("TypeError: `Call` args number mismatch");
        }
        let ctx = ctx.new_level();
        for ((name, _typ), value) in fun.args.0.iter().zip(args.clone()) {
            // todo: type check
            if let Some(name) = name {
                ctx.set_local(name, &value)
            }
        }
        let fun = fun.pe(ctx);
        if !fun.body.is_literal() {
            return Value::Call(Call {
                fun: Box::new(Value::Fun(Arc::new(fun))),
                args: args.collect(),
            });
        }
        let result = fun.body.get_literal().unwrap();
        result.into()
    }
}

fn builtin_function_call_pe(s: &Symbol, args: &[Value]) -> Value {
    if args.len() != args.iter().filter(|x| x.is_literal()).count() {
        return Value::Call(Call {
            fun: Box::new(Value::Var(SymbolRef::Symbol(s.clone()))),
            args: args.to_owned(),
        });
    }
    let _args: Vec<Literal> = args.iter().map(|x| x.get_literal().unwrap()).collect();

    todo!()
}

impl Pe for SymbolRef {
    type Target = Value;

    fn pe(&self, ctx: Context) -> Self::Target {
        if let Some(x) = ctx.get(self) {
            match x {
                Literal::ConstVal(c) => Value::Lit(c),
                Literal::Fun(f) => Value::Fun(f),
            }
        } else {
            Value::Var(self.clone())
        }
    }
}

impl Pe for Fun {
    type Target = Fun;
    fn pe(&self, ctx: Context) -> Self::Target {
        Fun {
            args: self.args.clone(),
            body: Box::new(self.body.pe(ctx)),
        }
    }
}