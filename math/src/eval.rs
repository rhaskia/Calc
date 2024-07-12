use crate::parse::ExprRef;
use crate::parse::{BinOp, EqType, ExprNode, ExprTree, MathTopLevel};
use std::collections::HashMap;

pub fn eval(formula: MathTopLevel) -> f64 {
    match formula.ty {
        EqType::Equality(lhs_ref, rhs_ref) => todo!(),
        EqType::Expr(expr_ref) => eval_expr(expr_ref, &formula.tree),
    }
}

pub fn eval_expr(expr_ref: ExprRef, tree: &ExprTree) -> f64 {
    match tree.get(expr_ref) {
        ExprNode::Binary(op, lhs, rhs) => {
            let lhs_result = eval_expr(lhs, &tree);
            let rhs_result = eval_expr(rhs, &tree);
            binop_to_fn(op)(lhs_result, rhs_result)
        }
        ExprNode::Num(n) => n,
        ExprNode::Var(v) => panic!("Variables not implemented yet"),
    }
}

pub fn binop_to_fn(binop: BinOp) -> fn(f64, f64) -> f64 {
    use std::ops::*;
    match binop {
        BinOp::Add => Add::add,
        BinOp::Sub => Sub::sub,
        BinOp::Div => Div::div,
        BinOp::Mul => Mul::mul,
        BinOp::Pow => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keysim;
    use crate::parse::parse;
    use crate::tok::apply_keystroke_seq;
    use crate::tok::MathTokenTree;

    #[test]
    fn basic() {
        let addition = parse(&keysim!("2+3")).unwrap();
        assert_eq!(eval(addition), (2.0 + 3.0));
        
        let repeated = parse(&keysim!("2+3+4+9")).unwrap();
        assert_eq!(eval(repeated), 2.0 + 3.0 + 4.0 + 9.0);
    }
}
