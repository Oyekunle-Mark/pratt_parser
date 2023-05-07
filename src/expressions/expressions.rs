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

impl Expression for ConditionalExpr {
    fn print(&self) -> String {
        format!(
            "({} ? {} : {})",
            self.condition.print(),
            self.then_arm.print(),
            self.else_arm.print()
        )
    }
}

impl Expression for NameExpr {
    fn print(&self) -> String {
        format!("{}", self.name)
    }
}

impl Expression for OperatorExpr {
    fn print(&self) -> String {
        format!(
            "({} {} {})",
            self.left.print(),
            self.operator,
            self.right.print()
        )
    }
}

impl Expression for PostfixExpr {
    fn print(&self) -> String {
        format!("({}{})", self.left.print(), self.operator)
    }
}

impl Expression for PrefixExpr {
    fn print(&self) -> String {
        format!("({}{})", self.operator, self.right.print())
    }
}
