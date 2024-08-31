use std::{collections::HashMap, rc::Rc};

#[derive(Debug)]
struct Value(u64);

impl Clone for Value {
    fn clone(&self) -> Self {
        println!("Cloned! val {:?}", self.0);
        Self(self.0)
    }
}
#[derive(Debug)]
struct Context {
    vars: HashMap<u16, Rc<Value>>,
}

impl Context {
    fn new() -> Self {
        let vars = HashMap::new();
        Self { vars }
    }
}

enum Expression {
    Value(Value),
    Variable(u16),
    Add(Box<Expression>, Box<Expression>),
    Assign(u16, Box<Expression>),
}

fn evaluate(ctx: &mut Context, expr: Expression) -> Option<Rc<Value>> {
    match expr {
        Expression::Value(val) => Some(Rc::from(val)),
        Expression::Variable(id) => ctx.vars.get(&id).cloned(),
        Expression::Add(a, b) => {
            let mut a = evaluate(ctx, *a).unwrap();
            let mut b = evaluate(ctx, *b).unwrap();
            let a_val = Rc::make_mut(&mut a);
            let b_val = Rc::make_mut(&mut b);

            let Value(a_val) = a_val;
            let Value(b_val) = b_val;
            *a_val += *b_val;

            Some(a)
        }
        Expression::Assign(id, expr) => {
            let value = evaluate(ctx, *expr).unwrap();
            ctx.vars.insert(id, value);
            None
        }
    }
}

fn main() {
    let mut context = Context::new();
    context.vars.insert(0, Rc::new(Value(9)));
    let expr = Expression::Assign(1, Box::new(Expression::Variable(0)));
    evaluate(&mut context, expr);
    let expr = Expression::Assign(
        1,
        Box::new(Expression::Add(
            Box::new(Expression::Variable(1)),
            Box::new(Expression::Value(Value(2))),
        )),
    );
    evaluate(&mut context, expr);
    println!("{:?}", context);
}
