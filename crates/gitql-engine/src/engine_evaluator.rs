use gitql_ast::expression::ArithmeticExpr;
use gitql_ast::expression::ArrayExpr;
use gitql_ast::expression::AssignmentExpr;
use gitql_ast::expression::BenchmarkCallExpr;
use gitql_ast::expression::BetweenExpr;
use gitql_ast::expression::BetweenKind;
use gitql_ast::expression::BitwiseExpr;
use gitql_ast::expression::BooleanExpr;
use gitql_ast::expression::CallExpr;
use gitql_ast::expression::CaseExpr;
use gitql_ast::expression::CastExpr;
use gitql_ast::expression::ColumnExpr;
use gitql_ast::expression::ComparisonExpr;
use gitql_ast::expression::ContainedByExpr;
use gitql_ast::expression::ContainsExpr;
use gitql_ast::expression::Expr;
use gitql_ast::expression::ExprKind::*;
use gitql_ast::expression::GlobExpr;
use gitql_ast::expression::GlobalVariableExpr;
use gitql_ast::expression::GroupComparisonExpr;
use gitql_ast::expression::InExpr;
use gitql_ast::expression::IndexExpr;
use gitql_ast::expression::IntervalExpr;
use gitql_ast::expression::IsNullExpr;
use gitql_ast::expression::LikeExpr;
use gitql_ast::expression::LogicalExpr;
use gitql_ast::expression::MemberAccessExpr;
use gitql_ast::expression::Number;
use gitql_ast::expression::NumberExpr;
use gitql_ast::expression::RegexExpr;
use gitql_ast::expression::RowExpr;
use gitql_ast::expression::SliceExpr;
use gitql_ast::expression::StringExpr;
use gitql_ast::expression::SymbolExpr;
use gitql_ast::expression::UnaryExpr;
use gitql_ast::operator::ArithmeticOperator;
use gitql_ast::operator::BinaryBitwiseOperator;
use gitql_ast::operator::BinaryLogicalOperator;
use gitql_ast::operator::ComparisonOperator;
use gitql_ast::operator::PrefixUnaryOperator;
use gitql_core::environment::Environment;
use gitql_core::values::array::ArrayValue;
use gitql_core::values::boolean::BoolValue;
use gitql_core::values::composite::CompositeValue;
use gitql_core::values::float::FloatValue;
use gitql_core::values::integer::IntValue;
use gitql_core::values::interval::IntervalValue;
use gitql_core::values::null::NullValue;
use gitql_core::values::row::RowValue;
use gitql_core::values::text::TextValue;
use gitql_core::values::Value;

use std::cmp::Ordering;
use std::string::String;

#[allow(clippy::borrowed_box)]
pub fn evaluate_expression(
    env: &mut Environment,
    expression: &Box<dyn Expr>,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    match expression.kind() {
        Assignment => {
            let expr = expression
                .as_any()
                .downcast_ref::<AssignmentExpr>()
                .unwrap();
            evaluate_assignment(env, expr, titles, object)
        }
        String => {
            let expr = expression.as_any().downcast_ref::<StringExpr>().unwrap();
            evaluate_string(expr)
        }
        Symbol => {
            let expr = expression.as_any().downcast_ref::<SymbolExpr>().unwrap();
            evaluate_symbol(expr, titles, object)
        }
        Array => {
            let expr = expression.as_any().downcast_ref::<ArrayExpr>().unwrap();
            evaluate_array(env, expr, titles, object)
        }
        GlobalVariable => {
            let expr = expression
                .as_any()
                .downcast_ref::<GlobalVariableExpr>()
                .unwrap();
            evaluate_global_variable(env, expr)
        }
        Number => {
            let expr = expression.as_any().downcast_ref::<NumberExpr>().unwrap();
            evaluate_number(expr)
        }
        Boolean => {
            let expr = expression.as_any().downcast_ref::<BooleanExpr>().unwrap();
            evaluate_boolean(expr)
        }
        Interval => {
            let expr = expression.as_any().downcast_ref::<IntervalExpr>().unwrap();
            evaluate_interval(expr)
        }
        PrefixUnary => {
            let expr = expression.as_any().downcast_ref::<UnaryExpr>().unwrap();
            evaluate_prefix_unary(env, expr, titles, object)
        }
        Index => {
            let expr = expression.as_any().downcast_ref::<IndexExpr>().unwrap();
            evaluate_collection_index(env, expr, titles, object)
        }
        Slice => {
            let expr = expression.as_any().downcast_ref::<SliceExpr>().unwrap();
            evaluate_collection_slice(env, expr, titles, object)
        }
        Arithmetic => {
            let expr = expression
                .as_any()
                .downcast_ref::<ArithmeticExpr>()
                .unwrap();
            evaluate_arithmetic(env, expr, titles, object)
        }
        Comparison => {
            let expr = expression
                .as_any()
                .downcast_ref::<ComparisonExpr>()
                .unwrap();
            evaluate_comparison(env, expr, titles, object)
        }
        GroupComparison => {
            let expr = expression
                .as_any()
                .downcast_ref::<GroupComparisonExpr>()
                .unwrap();
            evaluate_group_comparison(env, expr, titles, object)
        }
        Contains => {
            let expr = expression.as_any().downcast_ref::<ContainsExpr>().unwrap();
            evaluate_contains(env, expr, titles, object)
        }
        ContainedBy => {
            let expr = expression
                .as_any()
                .downcast_ref::<ContainedByExpr>()
                .unwrap();
            evaluate_contained_by(env, expr, titles, object)
        }
        Like => {
            let expr = expression.as_any().downcast_ref::<LikeExpr>().unwrap();
            evaluate_like(env, expr, titles, object)
        }
        Regex => {
            let expr = expression.as_any().downcast_ref::<RegexExpr>().unwrap();
            evaluate_regex(env, expr, titles, object)
        }
        Glob => {
            let expr = expression.as_any().downcast_ref::<GlobExpr>().unwrap();
            evaluate_glob(env, expr, titles, object)
        }
        Logical => {
            let expr = expression.as_any().downcast_ref::<LogicalExpr>().unwrap();
            evaluate_logical(env, expr, titles, object)
        }
        Bitwise => {
            let expr = expression.as_any().downcast_ref::<BitwiseExpr>().unwrap();
            evaluate_bitwise(env, expr, titles, object)
        }
        Call => {
            let expr = expression.as_any().downcast_ref::<CallExpr>().unwrap();
            evaluate_call(env, expr, titles, object)
        }
        BenchmarkCall => {
            let expr = expression
                .as_any()
                .downcast_ref::<BenchmarkCallExpr>()
                .unwrap();
            evaluate_benchmark_call(env, expr, titles, object)
        }
        Between => {
            let expr = expression.as_any().downcast_ref::<BetweenExpr>().unwrap();
            evaluate_between(env, expr, titles, object)
        }
        Case => {
            let expr = expression.as_any().downcast_ref::<CaseExpr>().unwrap();
            evaluate_case(env, expr, titles, object)
        }
        In => {
            let expr = expression.as_any().downcast_ref::<InExpr>().unwrap();
            evaluate_in(env, expr, titles, object)
        }
        IsNull => {
            let expr = expression.as_any().downcast_ref::<IsNullExpr>().unwrap();
            evaluate_is_null(env, expr, titles, object)
        }
        Cast => {
            let expr = expression.as_any().downcast_ref::<CastExpr>().unwrap();
            evaluate_cast(env, expr, titles, object)
        }
        Column => {
            let expr = expression.as_any().downcast_ref::<ColumnExpr>().unwrap();
            evaluate_column(env, expr, titles, object)
        }
        Row => {
            let expr = expression.as_any().downcast_ref::<RowExpr>().unwrap();
            evaluate_row(env, expr, titles, object)
        }
        MemberAccess => {
            let expr = expression
                .as_any()
                .downcast_ref::<MemberAccessExpr>()
                .unwrap();
            evaluate_member_access(env, expr, titles, object)
        }
        Null => Ok(Box::new(NullValue)),
    }
}

fn evaluate_assignment(
    env: &mut Environment,
    expr: &AssignmentExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let value = evaluate_expression(env, &expr.value, titles, object)?;
    env.globals.insert(expr.symbol.to_string(), value.clone());
    Ok(value)
}

fn evaluate_string(expr: &StringExpr) -> Result<Box<dyn Value>, String> {
    Ok(Box::new(TextValue::new(expr.value.to_owned())))
}

fn evaluate_symbol(
    expr: &SymbolExpr,
    titles: &[String],
    object: &[Box<dyn Value>],
) -> Result<Box<dyn Value>, String> {
    for (index, title) in titles.iter().enumerate() {
        if expr.value.eq(title) {
            return Ok(object[index].clone());
        }
    }
    Err(format!("Invalid column name `{}`", &expr.value))
}

fn evaluate_array(
    env: &mut Environment,
    expr: &ArrayExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let mut values: Vec<Box<dyn Value>> = Vec::with_capacity(expr.values.len());
    for value in &expr.values {
        values.push(evaluate_expression(env, value, titles, object)?);
    }
    Ok(Box::new(ArrayValue::new(values, expr.element_type.clone())))
}

fn evaluate_global_variable(
    env: &mut Environment,
    expr: &GlobalVariableExpr,
) -> Result<Box<dyn Value>, String> {
    let name = &expr.name;
    if env.globals.contains_key(name) {
        return Ok(env.globals[name].clone());
    }

    Err(format!(
        "The value of `{name}` may be not exists or calculated yet",
    ))
}

fn evaluate_number(expr: &NumberExpr) -> Result<Box<dyn Value>, String> {
    Ok(match expr.value {
        Number::Int(integer) => Box::new(IntValue::new(integer)),
        Number::Float(float) => Box::new(FloatValue::new(float)),
    })
}

fn evaluate_boolean(expr: &BooleanExpr) -> Result<Box<dyn Value>, String> {
    Ok(Box::new(BoolValue::new(expr.is_true)))
}

fn evaluate_interval(expr: &IntervalExpr) -> Result<Box<dyn Value>, String> {
    Ok(Box::new(IntervalValue::new(expr.interval.clone())))
}

fn evaluate_collection_index(
    env: &mut Environment,
    expr: &IndexExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let array = evaluate_expression(env, &expr.collection, titles, object)?;
    let index = evaluate_expression(env, &expr.index, titles, object)?;
    array.index_op(&index)
}

fn evaluate_collection_slice(
    env: &mut Environment,
    expr: &SliceExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let array = evaluate_expression(env, &expr.collection, titles, object)?;

    let start = if let Some(start_expr) = &expr.start {
        Some(evaluate_expression(env, start_expr, titles, object)?)
    } else {
        None
    };

    let end = if let Some(end_expr) = &expr.end {
        Some(evaluate_expression(env, end_expr, titles, object)?)
    } else {
        None
    };

    array.slice_op(&start, &end)
}

fn evaluate_prefix_unary(
    env: &mut Environment,
    expr: &UnaryExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let rhs = evaluate_expression(env, &expr.right, titles, object)?;
    match expr.operator {
        PrefixUnaryOperator::Negative => rhs.neg_op(),
        PrefixUnaryOperator::Bang => rhs.bang_op(),
        PrefixUnaryOperator::Not => rhs.not_op(),
    }
}

fn evaluate_arithmetic(
    env: &mut Environment,
    expr: &ArithmeticExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let lhs = evaluate_expression(env, &expr.left, titles, object)?;
    let rhs = evaluate_expression(env, &expr.right, titles, object)?;
    match expr.operator {
        ArithmeticOperator::Plus => lhs.add_op(&rhs),
        ArithmeticOperator::Minus => lhs.sub_op(&rhs),
        ArithmeticOperator::Star => lhs.mul_op(&rhs),
        ArithmeticOperator::Slash => lhs.div_op(&rhs),
        ArithmeticOperator::Modulus => lhs.rem_op(&rhs),
        ArithmeticOperator::Exponentiation => lhs.caret_op(&rhs),
    }
}

fn evaluate_comparison(
    env: &mut Environment,
    expr: &ComparisonExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let lhs = evaluate_expression(env, &expr.left, titles, object)?;
    let rhs = evaluate_expression(env, &expr.right, titles, object)?;
    match expr.operator {
        ComparisonOperator::Greater => lhs.gt_op(&rhs),
        ComparisonOperator::GreaterEqual => lhs.gte_op(&rhs),
        ComparisonOperator::Less => lhs.lt_op(&rhs),
        ComparisonOperator::LessEqual => lhs.lte_op(&rhs),
        ComparisonOperator::Equal => lhs.eq_op(&rhs),
        ComparisonOperator::NotEqual => lhs.bang_eq_op(&rhs),
        ComparisonOperator::NullSafeEqual => lhs.null_safe_eq_op(&rhs),
    }
}

fn evaluate_group_comparison(
    env: &mut Environment,
    expr: &GroupComparisonExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let lhs = evaluate_expression(env, &expr.left, titles, object)?;
    let rhs = evaluate_expression(env, &expr.right, titles, object)?;
    match expr.comparison_operator {
        ComparisonOperator::Greater => lhs.group_gt_op(&rhs, &expr.group_operator),
        ComparisonOperator::GreaterEqual => lhs.group_gte_op(&rhs, &expr.group_operator),
        ComparisonOperator::Less => lhs.group_lt_op(&rhs, &expr.group_operator),
        ComparisonOperator::LessEqual => lhs.group_lte_op(&rhs, &expr.group_operator),
        ComparisonOperator::Equal => lhs.group_eq_op(&rhs, &expr.group_operator),
        ComparisonOperator::NotEqual => lhs.group_bang_eq_op(&rhs, &expr.group_operator),
        ComparisonOperator::NullSafeEqual => lhs.group_null_safe_eq_op(&rhs, &expr.group_operator),
    }
}

fn evaluate_contains(
    env: &mut Environment,
    expr: &ContainsExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let lhs = evaluate_expression(env, &expr.left, titles, object)?;
    let rhs = evaluate_expression(env, &expr.right, titles, object)?;
    lhs.contains_op(&rhs)
}

fn evaluate_contained_by(
    env: &mut Environment,
    expr: &ContainedByExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let lhs = evaluate_expression(env, &expr.left, titles, object)?;
    let rhs = evaluate_expression(env, &expr.right, titles, object)?;
    rhs.contains_op(&lhs)
}

fn evaluate_like(
    env: &mut Environment,
    expr: &LikeExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let input = evaluate_expression(env, &expr.input, titles, object)?;
    let pattern = evaluate_expression(env, &expr.pattern, titles, object)?;
    input.like_op(&pattern)
}

fn evaluate_regex(
    env: &mut Environment,
    expr: &RegexExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let input = evaluate_expression(env, &expr.input, titles, object)?;
    let pattern = evaluate_expression(env, &expr.pattern, titles, object)?;
    input.regexp_op(&pattern)
}

fn evaluate_glob(
    env: &mut Environment,
    expr: &GlobExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let input = evaluate_expression(env, &expr.input, titles, object)?;
    let pattern = evaluate_expression(env, &expr.pattern, titles, object)?;
    input.glob_op(&pattern)
}

fn evaluate_logical(
    env: &mut Environment,
    expr: &LogicalExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let lhs = evaluate_expression(env, &expr.left, titles, object)?;
    let rhs = evaluate_expression(env, &expr.right, titles, object)?;
    match expr.operator {
        BinaryLogicalOperator::And => lhs.logical_and_op(&rhs),
        BinaryLogicalOperator::Or => lhs.logical_or_op(&rhs),
        BinaryLogicalOperator::Xor => lhs.logical_xor_op(&rhs),
    }
}

fn evaluate_bitwise(
    env: &mut Environment,
    expr: &BitwiseExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let lhs = evaluate_expression(env, &expr.left, titles, object)?;
    let rhs = evaluate_expression(env, &expr.right, titles, object)?;
    match expr.operator {
        BinaryBitwiseOperator::Or => lhs.or_op(&rhs),
        BinaryBitwiseOperator::And => lhs.and_op(&rhs),
        BinaryBitwiseOperator::Xor => lhs.xor_op(&rhs),
        BinaryBitwiseOperator::RightShift => lhs.shr_op(&rhs),
        BinaryBitwiseOperator::LeftShift => lhs.shl_op(&rhs),
    }
}

fn evaluate_call(
    env: &mut Environment,
    expr: &CallExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let function_name = expr.function_name.as_str();
    let mut arguments = Vec::with_capacity(expr.arguments.len());
    for arg in expr.arguments.iter() {
        arguments.push(evaluate_expression(env, arg, titles, object)?);
    }
    let function = env.std_function(function_name).unwrap();
    Ok(function(&arguments))
}

fn evaluate_benchmark_call(
    env: &mut Environment,
    expr: &BenchmarkCallExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let number_of_execution = evaluate_expression(env, &expr.count, titles, object)?;
    if let Some(number) = number_of_execution.as_any().downcast_ref::<IntValue>() {
        for _ in 0..number.value {
            evaluate_expression(env, &expr.expression, titles, object)?;
        }
    }
    Ok(Box::new(IntValue::new_zero()))
}

fn evaluate_between(
    env: &mut Environment,
    expr: &BetweenExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let value = evaluate_expression(env, &expr.value, titles, object)?;
    let range_start = evaluate_expression(env, &expr.range_start, titles, object)?;
    let range_end = evaluate_expression(env, &expr.range_end, titles, object)?;
    let comparing_result = match expr.kind {
        BetweenKind::Symmetric => {
            let (start, end) = if let Some(order) = range_start.compare(&range_end) {
                if Ordering::is_gt(order) {
                    (range_end, range_start)
                } else {
                    (range_start, range_end)
                }
            } else {
                (range_start, range_end)
            };
            value.compare(&start).unwrap().is_ge() && value.compare(&end).unwrap().is_le()
        }
        BetweenKind::Asymmetric => {
            value.compare(&range_start).unwrap().is_ge()
                && value.compare(&range_end).unwrap().is_le()
        }
    };
    Ok(Box::new(BoolValue::new(comparing_result)))
}

fn evaluate_case(
    env: &mut Environment,
    expr: &CaseExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let conditions = &expr.conditions;
    let values = &expr.values;

    for i in 0..conditions.len() {
        let condition = evaluate_expression(env, &conditions[i], titles, object)?;
        if let Some(bool_value) = condition.as_any().downcast_ref::<BoolValue>() {
            if bool_value.value {
                return evaluate_expression(env, &values[i], titles, object);
            }
        }
    }

    match &expr.default_value {
        Some(default_value) => evaluate_expression(env, default_value, titles, object),
        _ => Err("Invalid case statement".to_owned()),
    }
}

fn evaluate_in(
    env: &mut Environment,
    expr: &InExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let argument = evaluate_expression(env, &expr.argument, titles, object)?;
    for value_expr in &expr.values {
        let value = evaluate_expression(env, value_expr, titles, object)?;
        if argument.equals(&value) {
            return Ok(Box::new(BoolValue::new(!expr.has_not_keyword)));
        }
    }
    Ok(Box::new(BoolValue::new(expr.has_not_keyword)))
}

fn evaluate_is_null(
    env: &mut Environment,
    expr: &IsNullExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let argument = evaluate_expression(env, &expr.argument, titles, object)?;
    let is_null = argument.as_any().downcast_ref::<NullValue>().is_some();
    let result = if expr.has_not { !is_null } else { is_null };
    Ok(Box::new(BoolValue::new(result)))
}

fn evaluate_cast(
    env: &mut Environment,
    expr: &CastExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let value = evaluate_expression(env, &expr.value, titles, object)?;
    value.cast_op(&expr.result_type)
}

fn evaluate_column(
    env: &mut Environment,
    expr: &ColumnExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let value = evaluate_expression(env, &expr.expr, titles, object)?;
    Ok(value)
}

fn evaluate_row(
    env: &mut Environment,
    expr: &RowExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let mut values = Vec::with_capacity(expr.exprs.len());
    for column in expr.exprs.iter() {
        values.push(evaluate_expression(env, column, titles, object)?);
    }
    Ok(Box::new(RowValue::new(values, expr.row_type.to_owned())))
}

fn evaluate_member_access(
    env: &mut Environment,
    expr: &MemberAccessExpr,
    titles: &[String],
    object: &Vec<Box<dyn Value>>,
) -> Result<Box<dyn Value>, String> {
    let value = evaluate_expression(env, &expr.composite, titles, object)?;
    if let Some(composite_value) = value.as_any().downcast_ref::<CompositeValue>() {
        let member_name = &expr.member_name;
        return Ok(composite_value.members.get(member_name).unwrap().clone());
    }
    Err("Invalid value for Member access expression".to_owned())
}
