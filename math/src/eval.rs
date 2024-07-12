use crate::parse::ExprRef;
use crate::parse::{BinOp, EqType, ExprNode, ExprTree, MathTopLevel};
use std::collections::HashMap;
use std::ops::{Range, RangeInclusive};

pub fn eval(formula: MathTopLevel, constants: &HashMap<char, f64>) -> Result<f64, EvalError> {
    match formula.ty {
        EqType::Equality(lhs_ref, rhs_ref) => todo!(),
        EqType::Expr(expr_ref) => eval_expr(expr_ref, &formula.tree, constants),
    }
}

pub fn eval_expr(expr_ref: ExprRef, tree: &ExprTree, constants: &HashMap<char, f64>) -> Result<f64, EvalError> {
    match tree.get(expr_ref) {
        ExprNode::Binary(op, lhs, rhs) => {
            let lhs_result = eval_expr(lhs, &tree, constants)?;
            let rhs_result = eval_expr(rhs, &tree, constants)?;
            Ok(binop_to_fn(op)(lhs_result, rhs_result))
        }
        ExprNode::Num(n) => Ok(n),
        ExprNode::Var(v) => constants.get(&v).ok_or(EvalError::VariableNotFound).copied(),
    }
}

pub fn eval_range(formula: MathTopLevel, constants: &HashMap<char, f64>, range: VariableRange) -> Result<Vec<f64>, EvalError> {
    let values = range.collect();
    let mut results = Vec::new();
    let mut constants = constants.clone();

    for value in values {
        constants.insert(range.name, value);
        results.push(eval(formula.clone(), &constants)?);
    }

    Ok(results)
}


pub struct VariableRange {
    range: Range<f64>,
    resolution: u64,
    name: char,
}

impl VariableRange {
    pub fn new(name: char, range: Range<f64>, resolution: u64) -> Self {
        Self { range, resolution, name }
    }

    pub fn collect(&self) -> Vec<f64> {
        let Range { start, end } = self.range;
        let range = end-start;
        let step = range / (self.resolution - 1) as f64;
        (0..self.resolution).map(|i| start + (i as f64 * step)).collect()
    }
}

pub fn binop_to_fn(binop: BinOp) -> fn(f64, f64) -> f64 {
    use std::ops::*;
    match binop {
        BinOp::Add => Add::add,
        BinOp::Sub => Sub::sub,
        BinOp::Div => Div::div,
        BinOp::Mul => Mul::mul,
        BinOp::Pow => f64::powf,
    }
}

#[derive(Debug, PartialEq)]
pub enum EvalError {
    VariableNotFound,
    MathError,
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
        assert_eq!(eval(addition, &HashMap::new()), Ok(2.0 + 3.0));
        
        let repeated = parse(&keysim!("2+3+4+9")).unwrap();
        assert_eq!(eval(repeated, &HashMap::new()), Ok(2.0 + 3.0 + 4.0 + 9.0));
    }

    #[test]
    fn variables() {
        let var_mult = parse(&keysim!("2a")).unwrap();
        assert_eq!(eval(var_mult, &HashMap::new()), Err(EvalError::VariableNotFound));

        let var_mult = parse(&keysim!("2a")).unwrap();
        let mut vars = HashMap::new();
        vars.insert('a', 39.0);
        assert_eq!(eval(var_mult, &vars), Ok(2.0 * 39.0));
    }

    #[test]
    fn range() {
        let var_mult = parse(&keysim!("2a")).unwrap();
        let var = VariableRange::new('a', 0.0..10.0, 6);
        assert_eq!(eval_range(var_mult, &HashMap::new(), var), Ok(vec![0.0, 4.0, 8.0, 12.0, 16.0, 20.0]));
    }
}
