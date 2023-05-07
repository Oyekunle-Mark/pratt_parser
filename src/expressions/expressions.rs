trait Expression {
    fn print(&self) -> String;
}

struct AssignExpr {
    name: String,
    right: Box<dyn Expression>,
}

struct CallExpr {
    function: Box<dyn Expression>,
    args: Vec<Box<dyn Expression>>,
}

struct ConditionalExpr {
    condition: Box<dyn Expression>,
    then_arm: Box<dyn Expression>,
    else_arm: Box<dyn Expression>,
}

struct NameExpr {
    name: String,
}

struct OperatorExpr {
    left: Box<dyn Expression>,
    operator: String,
    right: Box<dyn Expression>,
}

struct PostfixExpr {
    left: Box<dyn Expression>,
    operator: String,
}

struct PrefixExpr {
    operator: String,
    right: Box<dyn Expression>,
}

impl Expression for AssignExpr {
    fn print(&self) -> String {
        format!("({}) = {})", self.name, self.right.print())
    }
}

impl Expression for CallExpr {
    fn print(&self) -> String {
        let mut args = String::from("");

        for expr in &self.args {
            args.push_str(&expr.print());
            args.push_str(",");
        }

        format!("{}({})", self.function.print(), args)
    }
}
