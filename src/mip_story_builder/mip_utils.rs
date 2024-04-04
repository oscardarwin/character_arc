use core::ops::Add;
use good_lp::Expression;
use ndarray::{ArrayView, ArrayView1, Dimension};

pub fn sum_optional_expressions(array_view: ArrayView1<Option<Expression>>) -> Option<Expression> {
    array_view
        .to_owned()
        .into_iter()
        .filter_map(|expr| expr)
        .reduce(|e_1, e_2| e_1.add(e_2))
}

pub fn sum_expressions<D: Dimension>(array_view: ArrayView<Expression, D>) -> Expression {
    array_view
        .to_owned()
        .into_iter()
        .reduce(|e_1, e_2| e_1.add(e_2))
        .unwrap()
}
