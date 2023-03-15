pub mod interpreter;
pub mod token;
pub mod lexer;
pub mod expr;
pub mod astprinter;

pub fn run(interpreter: &mut interpreter::Interpreter) {
    let lr = expr::ExprType::Literal(expr::Lit { value : "123" });
    let l = expr::ExprType::Unary(expr::Unr {
        right: &lr,
        operator: token::Token::new(token::TokenType::Minus, "-", None, 1, 0, 0),
    });

    let re = expr::ExprType::Literal(expr::Lit { value : "45.67" });
    let r = expr::ExprType::Grouping(expr::Grp{
        expr: &re
    });

    let expr = expr::ExprType::Binary(expr::Bin {
        left: &l,

        operator: token::Token::new(token::TokenType::Star, "*", None, 1, 0, 0),

        right: &r,
    });

    println!("{}", (astprinter::AstPrinter {}).print(expr));
}