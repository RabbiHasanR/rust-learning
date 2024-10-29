#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
enum Expression {
    Op { op: Operation, left: Box<Expression>, right: Box<Expression> },
    Value(i64),
}

fn eval(e: Expression) -> Result<i64, String> {
    match e {
        Expression::Op { op, left, right} => {
            let left = match eval(*left) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };
            let right = match eval(*right) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

            Ok(
                match op {
                    Operation::Add => left + right,
                    Operation::Sub => left - right,
                    Operation::Mul => left * right,
                    Operation::Div => {
                        if right == 0 {
                            return Err(String::from("division by zero"));
                        } else {
                            left /right
                        }
                    }
                }
            )
        },
        Expression::Value(v) => Ok(v),
    }
}



pub fn expression_evaluation() {
    let expr = Expression::Op {
        op: Operation::Sub,
        left: Box::new(Expression::Value(20)),
        right: Box::new(Expression::Value(10)),
    };
    println!("expr: {expr:?}");
    println!("result: {:?}", eval(expr));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        assert_eq!(eval(Expression::Value(19)), Ok(19));
    }

    #[test]
    fn test_sum() {
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Add,
                left: Box::new(Expression::Value(10)),
                right: Box::new(Expression::Value(20)),
            }),
            Ok(30)
        )
    }

    #[test]
    fn test_recursion() {
        let term1 = Expression::Op {
            op: Operation::Mul,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(9)),
        };
        let term2 = Expression::Op {
            op: Operation::Mul,
            left: Box::new(Expression::Op {
                op: Operation::Sub,
                left: Box::new(Expression::Value(3)),
                right: Box::new(Expression::Value(4)),
            }),
            right: Box::new(Expression::Value(5))
        };

        assert_eq!(eval(
            Expression::Op {
                op: Operation::Add,
                left: Box::new(term1),
                right: Box::new(term2),
            }
        ), Ok(85));
    }


    #[test]
    fn test_error() {
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Div,
                left: Box::new(Expression::Value(99)),
                right: Box::new(Expression::Value(0)),
            }),
            Err(String::from("division by zero"))
        )
    }
}