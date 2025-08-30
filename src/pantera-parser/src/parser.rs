use std::string::ToString;
use pantera_ast::expression::{ArrayExpression, AssignmentExpression, BinaryExpression, CallExpression, Expression, GroupExpression, Identifier, MemberExpression, ObjectExpression, Operator, UnaryExpression};
use crate::token::{Token, TokenType};
use pantera_ast::statement::{BlockStatement, DeclarationKind, DeclarationStatement, ExpressionStatement, FunctionDeclarationStatement, GlobalStatement, IfStatement, LoopStatement, MultiDeclarationStatement, PrintStatement, Range, ReturnStatement, Statement};
use crate::errors::ParseError;

pub struct Parser {
    pub source: Vec<Token>,
    start: i32,
    current: i32
}

const FUNCTION_NAME_SEPARATOR: &str = "_";

type ParserResult<T> = Result<T, ParseError>;

impl Parser{
    pub fn parse_program(&mut self) -> ParserResult<Vec<GlobalStatement>> {
        let mut stmts = vec![];
        while !self.at_end() {
            let token = self.peek();
            if token.typ == TokenType::Fun {
                stmts.push(self.parse_function_declaration()?);
            } else {
                stmts.push(GlobalStatement::Statement(self.parse_statement()?));
            }
        };
        Ok(stmts)
    }

    pub fn parse_statement(&mut self) -> ParserResult<Statement> {
        let token = self.peek();
        match token.typ {
            TokenType::Break => {
                self.advance();
                self.consume(TokenType::Semicolon, "Expected ; at the end of the statement")?;
                Ok(Statement::Break)
            },
            TokenType::Print => {
                self.parse_print_stmt()
            },
            TokenType::Return => {
                self.parse_return_stmt()
            }
            TokenType::LeftParen => {
                // should change once spreading is allowed
                if self.peek_nth(2).typ == TokenType::Colon {
                    self.parse_expression_statement()
                } else {
                    self.parse_block_stmt(false)
                }
            },
            TokenType::If => {
                self.parse_if_stmt()
            },
            TokenType::Loop => {
                self.parse_loop_stmt()
            },
            TokenType::Const | TokenType::Var => {
                self.parse_decl_statement()
            }
            _ => {
                self.parse_expression_statement()
            }
        }
    }

    pub fn parse_function_declaration(&mut self) -> ParserResult<GlobalStatement> {
        self.advance();
        let mut id_parts = vec![];
        let mut params = vec![];
        if self.peek().typ == TokenType::LeftParen {
            return Err(ParseError{
                message: "Expected function name".to_string(),
                line: 1
            });
        }
        while self.peek().typ != TokenType::LeftParen {
            let token = self.peek();
            if let TokenType::Identifier(ident) = &token.typ {
                id_parts.push(ident.clone());
                self.advance();
                if self.peek().typ == TokenType::LeftBrace {
                    let local_params = self.parse_function_params()?;
                    local_params.into_iter().for_each(|param| params.push(param));
                    self.consume(TokenType::RightBrace, "Expected right parenthesis after function params declaration")?;
                }
            } else {
                return Err(ParseError{
                    message: "Cannot have chained params list in the function declaration".to_string(),
                    line: 1
                })
            }
        }

        Ok(GlobalStatement::FunctionDeclaration(FunctionDeclarationStatement{
            name: Identifier{name: id_parts.join(FUNCTION_NAME_SEPARATOR), id: 1.0},
            params,
            body: self.parse_block_stmt(true)?,
        }))
    }

    pub fn parse_function_params(&mut self) -> ParserResult<Vec<Identifier>> {
        self.advance();
        let mut ids = vec![];
        if let TokenType::Identifier(ident) = &self.peek().typ {
            ids.push(Identifier{name: ident.clone(), id: 1.0});
            self.advance();
        } else {
            return Err(ParseError{
                message: "Expected formal function parameter definition".to_string(),
                line: 1
            });
        }
        while self.peek().typ != TokenType::RightBrace {
            self.consume(TokenType::Comma, "Expected comma to separate function parameter")?;
            if let TokenType::Identifier(ident) = &self.peek().typ {
                ids.push(Identifier{name: ident.clone(), id: 1.0});
                self.advance();
            } else {
                return Err(ParseError{
                    message: "Expected formal function parameter definition".to_string(),
                    line: 1
                });
            }
        }
        Ok(ids)
    }

    pub fn parse_decl_statement(&mut self) -> ParserResult<Statement> {
        let token = self.advance().unwrap();
        let declaration_kind =
            if token.typ == TokenType::Var {
                DeclarationKind::Var
            }else {
                DeclarationKind::Const
            };
        let mut declarations = vec![];
        loop {
            let TokenType::Identifier(assignee) = self.advance().unwrap().clone().typ else {panic!("Assignee has to be a variable")};
            if self.peek().typ == TokenType::Equal {
                self.advance();
                let value = self.parse_expression()?;
                declarations.push(DeclarationStatement {
                    kind: declaration_kind.clone(),
                    variable: assignee,
                    value: Some(value),
                })
            } else {
                declarations.push(DeclarationStatement {
                    kind: declaration_kind.clone(),
                    variable: assignee,
                    value: None,
                })
            }
            if self.peek().typ == TokenType::Semicolon {
                self.advance();
                break;
            }

            self.consume(TokenType::Comma, "Expected , in between declaring multiple variables")?;
        }
        if declarations.len() == 1 {
            let declaration = declarations.remove(0);
            return Ok(Statement::Declaration(declaration));
        }
        Ok(Statement::MultiDeclaration(MultiDeclarationStatement{declarations}))
    }

    pub fn parse_loop_stmt(&mut self) -> ParserResult<Statement> {
        self.advance();
        let mut alias = "it".to_string();
        if self.peek().typ == TokenType::LeftParen {
            let body = self.parse_statement()?;
            Ok(Statement::Loop(Box::from(LoopStatement {
                body,
                alias
            })))
        } else {
            let mut iterate_reverse = false;
            if self.peek().typ == TokenType::Reverse {
                iterate_reverse = true;
                self.advance();
            }
            let range = self.parse_range()?;
            if self.peek().typ == TokenType::As {
                self.advance();
                let identifier = self.parse_expression()?;
                if let Some(id) = identifier.get_identifier() {
                    alias = id.clone();
                } else {
                    return Err(ParseError {
                        line: 1,
                        message: "Expected identifier after as keyword".to_string()
                    });
                }

            }
            if self.peek().typ == TokenType::LeftParen {
                let body = self.parse_statement()?;
                let Statement::Block(stmts) = body else { panic!("Not a block statement"); };

                let [start, stop]: [Expression;2] = if iterate_reverse {[range.stop.unwrap(), range.start]} else {[range.start, range.stop.unwrap()]};

                let mut statements = vec![];
                let init_clause = Statement::Declaration(
                    DeclarationStatement{
                        kind: DeclarationKind::Var,
                        variable: alias.clone(),
                        value: Some(start),
                    }
                );
                let mut loop_stmts = vec![];
                stmts.statements.into_iter().for_each(|st|loop_stmts.push(st));

                statements.push(Statement::Block(Box::from(BlockStatement {
                    statements: loop_stmts,
                })));

                statements.push(Statement::Expression(Box::from(ExpressionStatement {
                    expr: Expression::Assigment(Box::from(AssignmentExpression {
                        assignee: Expression::Identifier(alias.clone()),
                        value: Expression::Binary(Box::from(BinaryExpression {
                            left: Expression::Identifier(alias.clone()),
                            operator: if iterate_reverse {Operator::Minus} else {Operator::Plus} ,
                            right: Expression::Number(1f32),
                        })),
                    }))
                })));

                statements.push(Statement::If(Box::from(IfStatement {
                    condition: Expression::Binary(Box::from(BinaryExpression {
                        left: Expression::Identifier(alias.clone()),
                        operator: if iterate_reverse {Operator::Le} else {Operator::Ge},
                        right: stop,
                    })),
                    body: Statement::Break,
                    alternative: None,
                })));

                Ok(Statement::Block(Box::from(BlockStatement {
                    statements: vec![init_clause, Statement::Loop(Box::from(LoopStatement {
                        body: Statement::Block(Box::from(BlockStatement{
                            statements
                        })),
                        alias
                    }))],
                })))
            } else {
                Err(ParseError {
                    line: 1,
                    message: "Expected { after loop declaration".to_string()
                })
            }
        }
    }

    pub fn parse_range(&mut self) -> ParserResult<Range> {
        let start = self.parse_expression()?;
        if self.peek().typ == TokenType::DoubleDot {
            self.advance();
            let stop = self.parse_expression()?;
            Ok(Range {
                start,
                stop: Some(stop)
            })
        } else {
            Ok(Range{
                start,
                stop: None
            })
        }
    }

    pub fn parse_if_stmt(&mut self) -> ParserResult<Statement> {
        self.advance();
        let expr = self.parse_expression()?;
        if self.peek().typ == TokenType::LeftParen {
            let body = self.parse_block_stmt(false)?;
            if self.peek().typ == TokenType::Else {
                self.advance();
                let alternative_stmt = self.parse_statement()?;
                Ok(Statement::If(Box::from(IfStatement{
                    condition: expr,
                    body,
                    alternative: Some(alternative_stmt)
                })))
            } else {
                Ok(Statement::If(Box::from(IfStatement{
                    condition: expr,
                    body,
                    alternative: None
                })))
            }
        } else {
            Err(ParseError {
                message: "Expected { after if statement condition".to_string(),
                line: 1
            })
        }
    }

    pub fn parse_block_stmt(&mut self, is_function: bool) -> ParserResult<Statement> {
        self.advance();
        let mut stmts = vec![];
        loop {
            if self.at_end() {
                return Err(ParseError {
                    message: "Expected } at the end of a block statement".to_string(),
                    line: 1
                });
            }
            let token = self.peek();
            if token.typ == TokenType::RightParen {
                self.advance();
                break;
            }
            stmts.push(self.parse_statement()?);
        };
        if is_function {
            Ok(Statement::FunctionBody(Box::from(BlockStatement {
                statements: stmts
            })))
        }
        else {
            Ok(Statement::Block(Box::from(BlockStatement {
                statements: stmts
            })))
        }
    }

    pub fn parse_return_stmt(&mut self) -> ParserResult<Statement> {
        self.advance();
        if self.peek().typ == TokenType::Semicolon {
            self.advance();
            return Ok(Statement::Return(Box::from(ReturnStatement {
                value: None
            })));
        }
        let expr = self.parse_expression()?;
        self.consume(TokenType::Semicolon, "Expected ; at the end of the statement")?;
        Ok(Statement::Return(Box::from(ReturnStatement {
            value: Some(expr)
        })))
    }

    pub fn parse_print_stmt(&mut self) -> ParserResult<Statement> {
        self.advance();
        let expr = self.parse_expression()?;
        self.consume(TokenType::Semicolon, "Expected ; at the end of the statement")?;
        Ok(Statement::Print(Box::from(PrintStatement {
            expr
        })))
    }

    pub fn parse_expression_statement(&mut self) -> ParserResult<Statement> {
        let expr = self.parse_expression()?;
        self.consume(TokenType::Semicolon, "Expected ; at the end of the statement")?;
        Ok(Statement::Expression(Box::from(ExpressionStatement{
            expr
        })))
    }

    pub fn parse_expression(&mut self) -> ParserResult<Expression> {
        self.parse_assignment()
    }

    pub fn parse_assignment(&mut self) -> ParserResult<Expression> {
        let left = self.parse_or()?;
        if self.peek().typ == TokenType::Equal {
            self.advance();
            let right = self.parse_expression()?;
            return match left {
                Expression::Identifier(_) | Expression::Member(_) => {
                    Ok(Expression::Assigment(Box::new(AssignmentExpression{
                        assignee: left,
                        value: right,
                    })))
                },
                _ => {
                    Err(ParseError{
                        message: "Incorrect lvalue".to_string(),
                        line: 1,
                    })
                }
            };
        }

        Ok(left)
    }

    pub fn parse_or(&mut self) -> ParserResult<Expression> {
        let mut rez = self.parse_and()?;
        while TokenType::Or == self.peek().typ {
            self.advance();
            rez = {
                let right_hand = self.parse_and()?;
                Expression::Binary(Box::new(BinaryExpression {
                    left: rez,
                    operator: Operator::Or,
                    right: right_hand,
                }))
            }
        }
        Ok(rez)
    }

    pub fn parse_and(&mut self) -> ParserResult<Expression> {
        let mut rez = self.parse_eq()?;
        while self.peek().typ == TokenType::And {
            self.advance();
            rez = {
                let right_hand = self.parse_eq()?;
                Expression::Binary(Box::new(BinaryExpression {
                    left: rez,
                    operator: Operator::And,
                    right: right_hand,
                }))
            }
        }

        Ok(rez)
    }

    pub fn parse_eq(&mut self) -> ParserResult<Expression> {
        let left = self.parse_rel()?;
        if self.peek().typ == TokenType::Is {
            self.advance();
            let mut operator = Operator::Eq;
            if self.peek().typ == TokenType::Not {
                operator = Operator::NE;
                self.advance();
            }
            let right = self.parse_rel()?;
            return Ok(Expression::Binary(Box::new(BinaryExpression {
                left,
                right,
                operator
            })));
        }

        Ok(left)
    }

    pub fn parse_rel(&mut self) -> ParserResult<Expression> {
        let left = self.parse_term()?;
        let operator = match self.peek().typ {
            TokenType::Grater => Some(Operator::Greater),
            TokenType::Less => Some(Operator::Less),
            TokenType::GraterEqual => Some(Operator::Ge),
            TokenType::LessEqual => Some(Operator::Le),
            _ => None
        };
        if self.peek().typ == TokenType::GraterEqual || self.peek().typ == TokenType::Grater || self.peek().typ == TokenType::Less || self.peek().typ == TokenType::LessEqual {
            self.advance();
            let right = self.parse_term()?;
            return Ok(Expression::Binary(Box::new(BinaryExpression {
                left,
                operator: operator.unwrap(),
                right,
            })));
        }

        Ok(left)
    }

    pub fn parse_term(&mut self) -> ParserResult<Expression> {
        let mut rez = self.parse_factor()?;
        while self.peek().typ == TokenType::Plus || self.peek().typ == TokenType::Minus {
            let op = self.advance().unwrap();
            let operator = if op.typ == TokenType::Plus {Operator::Plus} else {Operator::Minus};
            rez = {
                let right = self.parse_factor()?;
                Expression::Binary(Box::new(BinaryExpression {
                    left: rez,
                    operator,
                    right,
                }))
            }
        }

        Ok(rez)
    }

    pub fn parse_factor(&mut self) -> ParserResult<Expression> {
        let mut rez = self.parse_unary()?;
        while self.peek().typ == TokenType::Star || self.peek().typ == TokenType::Slash {
            let op = self.advance().unwrap();
            let operator = if op.typ == TokenType::Slash {Operator::Div} else {Operator::Mul};
            rez = {
                let right = self.parse_unary()?;
                Expression::Binary(Box::new(BinaryExpression {
                    left: rez,
                    operator,
                    right,
                }))
            }
        }

        Ok(rez)
    }

    pub fn parse_unary(&mut self) -> ParserResult<Expression> {
        if self.peek().typ == TokenType::Minus || self.peek().typ == TokenType::Not {
            let op = self.advance().unwrap();
            let operator = if op.typ == TokenType::Minus {Operator::Minus} else {Operator::NE};
            let expr = self.parse_pow()?;
            return Ok(Expression::Unary(Box::new(UnaryExpression {
                operator,
                expr
            })));
        }
        let expr = self.parse_pow()?;
        Ok(expr)
    }

    pub fn parse_pow(&mut self) -> ParserResult<Expression> {
        let mut rez = self.parse_call()?;
        while self.peek().typ == TokenType::Pow {
            self.advance();
            rez = {
                let right = self.parse_call()?;
                Expression::Binary(Box::new(BinaryExpression{
                    left: rez,
                    operator: Operator::Pow,
                    right
                }))
            };
        }

        Ok(rez)
    }

    pub fn parse_call(&mut self) -> ParserResult<Expression> {
        let mut rez = self.parse_primary()?;
        loop {
            if self.peek().typ == TokenType::LeftBrace {
                if matches!(rez, Expression::Identifier(_)) {
                    let (callee, args) = self.parse_function_rest(&rez)?;

                    rez = Expression::Call(Box::new(CallExpression {
                        callee: Expression::Identifier(callee),
                        args,
                    }))
                } else {
                    self.advance();
                    let args = self.parse_args()?;
                    rez = Expression::Call(Box::new(CallExpression {
                        callee: rez,
                        args
                    }))
                }

            } else if self.peek().typ == TokenType::Possesive {
                self.advance();
                let member = self.parse_primary()?;
                if matches!(member, Expression::Identifier(_)) {
                    if self.peek().typ == TokenType::LeftBrace {
                        let (callee, args) = self.parse_function_rest(&member)?;

                        rez = Expression::Call(Box::new(CallExpression {
                            callee: Expression::Member(Box::new(MemberExpression {
                                callee: rez,
                                property: Expression::String(callee),
                            })),
                            args,
                        }))
                    } else {
                        rez = Expression::Member(Box::new(MemberExpression {
                            callee: rez,
                            property: Expression::String(member.get_identifier().unwrap().to_string()),
                        }));
                    }
                } else {
                    rez = Expression::Member(Box::new(MemberExpression {
                        callee: rez,
                        property: member,
                    }));
                }
            } else{
                break;
            }
        }
        Ok(rez)
    }

    pub fn parse_function_rest(&mut self, beginning: &Expression) -> ParserResult<(String, Vec<Expression>)> {
        let function_beg = beginning.get_identifier();
        if function_beg.is_none() {
            return Err(ParseError {
                message: "Expected an identifier for function call".to_string(),
                line: 1
            })
        }
        let mut id_parts = vec![function_beg.unwrap().clone()];
        let mut func_args = vec![];
        loop {
            if self.peek().typ == TokenType::LeftBrace {
                self.advance();
                self.parse_args()?.into_iter().for_each(|arg| func_args.push(arg));
            } else {
                if let TokenType::Identifier(val) = &self.peek().typ {
                    id_parts.push(val.clone());
                    self.advance();
                } else {
                    break;
                }
            }
        }

        Ok((id_parts.join(FUNCTION_NAME_SEPARATOR), func_args))
    }

    pub fn parse_args(&mut self) -> ParserResult<Vec<Expression>> {
        let mut args = vec![];
        if self.peek().typ == TokenType::RightBrace {
            self.advance();
            return Ok(vec![]);
        }
        args.push(self.parse_expression()?);
        while self.peek().typ == TokenType::Comma {
            self.advance();
            args.push(self.parse_expression()?);
        }
        self.consume(TokenType::RightBrace, "Expected ')' after arguments definitions")?;
        Ok(args)
    }

    pub fn parse_object(&mut self) -> ParserResult<Expression> {
        let mut keys = vec![];
        let mut values = vec![];
        while self.peek().typ != TokenType::RightParen {
            let expr = self.parse_primary()?;
            match &expr {
                Expression::Identifier(ident) => {
                    keys.push(Expression::String(ident.to_string()));

                    self.consume(TokenType::Colon, "Key value pairs must be separated by :")?;
                    let val = self.parse_expression()?;
                    values.push(val);
                }
                Expression::String(str) => {
                    keys.push(expr);

                    self.consume(TokenType::Colon, "Key value pairs must be separated by :")?;
                    let val = self.parse_expression()?;
                    values.push(val);
                }
                Expression::Number(num) => {
                    keys.push(Expression::String(num.to_string()));

                    self.consume(TokenType::Colon, "Key value pairs must be separated by :")?;
                    let val = self.parse_expression()?;
                    values.push(val);
                },
                _ => {
                    return Err(ParseError {
                        message: "Object key must be an identifier, string or number".to_string(),
                        line: 1
                    });
                }
            };
            if self.peek().typ == TokenType::Comma {
                self.consume(TokenType::Comma, "This error shouldn't be displayed ever")?;
            }
        }
        self.advance();
        Ok(Expression::Object(Box::new(ObjectExpression{
            properties: keys,
            values
        })))
    }

    pub fn parse_array(&mut self) -> ParserResult<Expression> {
        let mut keys = vec![];
        let mut values = vec![];
        while self.peek().typ != TokenType::RightSquareBracket {
            let expr = self.parse_expression()?;

            values.push(expr);
            keys.push(Expression::Number(keys.len() as f32));

            if self.peek().typ == TokenType::Comma {
                self.consume(TokenType::Comma, "This error shouldn't be displayed ever")?;
            }
        }
        self.advance();
        Ok(Expression::Array(Box::new(ArrayExpression{
            values
        })))
    }

    pub fn parse_primary(&mut self) -> ParserResult<Expression> {
        let tok = self.advance().unwrap();
        match &tok.typ {
            TokenType::True => Ok(Expression::Bool(true)),
            TokenType::False => Ok(Expression::Bool(false)),
            TokenType::Nil => Ok(Expression::Nil),
            TokenType::String(str) => Ok(Expression::String(str.to_string())),
            TokenType::Number(num) => Ok(Expression::Number(num.clone())),
            TokenType::Identifier(ident) => Ok(Expression::Identifier(ident.to_string())),
            TokenType::LeftParen => self.parse_object(),
            TokenType::LeftSquareBracket => self.parse_array(),
            TokenType::LeftBrace => {
                let expr = self.parse_expression()?;
                self.consume(TokenType::RightBrace, "Expected ')' at the end of expression")?;
                Ok(Expression::Group(Box::new(GroupExpression { expr })))
            }
            _ => Err(ParseError {
                message: "Expression expected".to_string(),
                line: 1,
            })
        }
    }

    pub fn advance(&mut self) -> Option<&Token> {
        self.current += 1;
        self.source.get((self.current - 1) as usize)
    }

    pub fn consume(&mut self, token_type: TokenType, error: &str) -> ParserResult<&Token> {
        if self.peek().typ == token_type {
            Ok(self.advance().unwrap())
        } else {
            Err(ParseError{
                message: error.to_string(),
                line: 1
            })
        }
    }

    pub fn peek(&self) -> &Token {
        self.source.get(self.current as usize).unwrap()
    }

    pub fn peek_nth(&self, index: i32) -> &Token {
        self.source.get((self.current + index) as usize).unwrap()
    }

    pub fn at_end(&self) -> bool {
        self.peek().typ == TokenType::Eof
    }
    pub fn new(source: Vec<Token>) -> Self {
        Self {
            source,
            start: 0,
            current: 0
        }
    }
}

#[cfg(test)]
mod tests {
    use pantera_ast::expression::{Expression, Operator};
    use pantera_ast::statement::{DeclarationKind, GlobalStatement, Statement};
    use crate::lexer::Lexer;
    use crate::parser::{Parser, FUNCTION_NAME_SEPARATOR};
    
    fn get_new_parser(input: &str) -> Vec<GlobalStatement> {
        let rez = Parser::new(Lexer::new(input).scan_tokens().unwrap()).parse_program();
        if rez.is_err() {
            let Err(ref mess) = rez else {panic!("Not possible")};
            assert!(false);
        }
        rez.unwrap()
    }

    #[test]
    pub fn test_parse_identifier() {
        let result  = get_new_parser("x;");
        assert_eq!(result.len(), 1);
        let stmt = result.get(0).unwrap();
        if let GlobalStatement::Statement(Statement::Expression(expr)) = stmt {
            assert_eq!(expr.expr.get_identifier().unwrap(), "x");
            return;
        }
        assert!(false);
    }

    #[test]
    pub fn test_parse_declaration() {
        let result  = get_new_parser("var x = 3;");
        assert_eq!(result.len(), 1);

        let stmt = result.get(0).unwrap();
        if let GlobalStatement::Statement(Statement::Declaration(ref stmt)) = stmt {
            if let Some(Expression::Number(x)) = stmt.value {
                assert_eq!(x, 3f32);
            }
            assert_eq!(stmt.variable, "x");
            assert!(matches!(stmt.kind, DeclarationKind::Var));
            return;
        }
        assert!(false);
    }
    #[test]
    pub fn test_if_statement() {
        let result = get_new_parser("if true {x;} s;");
        assert_eq!(result.len(), 2);
        let stmt = result.get(0).unwrap();
        if let GlobalStatement::Statement(Statement::If(expr)) = stmt {
            assert!(expr.alternative.is_none());
            assert!(matches!(expr.condition, Expression::Bool(true)));
            assert!(matches!(expr.body, Statement::Block(_)));
            if let Statement::Block(block) = &expr.body {
                assert_eq!(block.statements.len(), 1);
            } else {
                assert!(false);
            }
            return;
        }
        assert!(false);
    }

    #[test]
    pub fn test_loop_statement() {
        let result = get_new_parser("loop 1..3 {print it;}");
        assert_eq!(result.len(), 1);
        let stmt = result.get(0).unwrap();
        if let GlobalStatement::Statement(Statement::Block(loop_stmt)) = stmt {
            assert_eq!(loop_stmt.statements.len(), 2);

            assert!(loop_stmt.statements.get(0).is_some_and(|x| matches!(*x, Statement::Declaration(_))));
            assert!(loop_stmt.statements.get(1).is_some_and(|x| matches!(*x, Statement::Loop(_))));
            return;
        }
        assert!(false);
    }

    #[test]
    pub fn test_function_declaration() {
        let result = get_new_parser("fun check(a)greater_than(b) {return a > b;}");
        assert_eq!(result.len(), 1);

        if let GlobalStatement::FunctionDeclaration(ref func_dec) = result.get(0).unwrap() {
            // function params
            assert_eq!(func_dec.params.len(), 2);
            assert!(func_dec.params.get(0).is_some_and(|x|x.name == "a"));
            assert!(func_dec.params.get(1).is_some_and(|x|x.name == "b"));

            // function name
            assert_eq!(func_dec.name.name, "check_greater_than".to_string());

            // function body
            assert!(matches!(func_dec.body, Statement::Block(_)));
        } else {
            assert!(false);
        }
    }

    #[test]
    pub fn test_function_declaration_multiple_params() {
        let result = get_new_parser("fun check(a,b) {return a > b;}");
        assert_eq!(result.len(), 1);

        if let GlobalStatement::FunctionDeclaration(ref func_dec) = result.get(0).unwrap() {
            // function params
            assert_eq!(func_dec.params.len(), 2);
            assert!(func_dec.params.get(0).is_some_and(|x|x.name == "a"));
            assert!(func_dec.params.get(1).is_some_and(|x|x.name == "b"));

            // function name
            assert_eq!(func_dec.name.name, "check".to_string());

            // function body
            assert!(matches!(func_dec.body, Statement::Block(_)));
        } else {
            assert!(false);
        }
    }

    #[test]
    pub fn test_procedure() {
        let result = get_new_parser("fun check {return a > b;}");
        assert_eq!(result.len(), 1);

        if let GlobalStatement::FunctionDeclaration(ref func_dec) = result.get(0).unwrap() {
            // function params
            assert_eq!(func_dec.params.len(), 0);

            // function name
            assert_eq!(func_dec.name.name, "check".to_string());

            // function body
            assert!(matches!(func_dec.body, Statement::Block(_)));
        } else {
            assert!(false);
        }
    }

    #[test]
    pub fn test_or_expression() {
        let result = get_new_parser("x^2 or true;");
        assert_eq!(result.len(), 1);

        if let GlobalStatement::Statement(Statement::Expression(ref wrapper)) = result.get(0).unwrap() {
            if let Expression::Binary(ref expr) = wrapper.expr {
                assert!(matches!(expr.right, Expression::Bool(true)));
                assert!(matches!(expr.operator, Operator::Or));

                if let Expression::Binary(ref left) = expr.left {
                    assert!(matches!(left.left, Expression::Identifier(_)));
                    assert!(matches!(left.operator, Operator::Pow));
                    assert!(matches!(left.right, Expression::Number(2.0)))
                } else {
                    assert!(false);
                }
            }
            else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    pub fn test_and_expression() {
        let result = get_new_parser("3 and 4 + 7^2;");
        assert_eq!(result.len(), 1);

        if let GlobalStatement::Statement(Statement::Expression(ref wrapper)) = result.get(0).unwrap() {
            if let Expression::Binary(ref expr) = wrapper.expr {
                assert!(matches!(expr.left, Expression::Number(3.0)));
                assert!(matches!(expr.operator, Operator::And));

                if let Expression::Binary(ref right) = expr.right {
                    assert!(matches!(right.left, Expression::Number(4.0)));
                    assert!(matches!(right.operator, Operator::Plus));
                    if let Expression::Binary(ref last) = right.right {
                        assert!(matches!(last.left, Expression::Number(7.0)));
                        assert!(matches!(last.operator, Operator::Pow));
                        assert!(matches!(last.right, Expression::Number(2.0)));
                    } else {
                        assert!(false);
                    }
                } else {
                    assert!(false);
                }
            }
            else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    pub fn test_pow_expression() {
        let result = get_new_parser("-7^x;");
        assert_eq!(result.len(), 1);


        if let GlobalStatement::Statement(Statement::Expression(ref wrapper)) = result.get(0).unwrap() {
            if let Expression::Unary(ref expr) = wrapper.expr {
                assert!(matches!(expr.operator, Operator::Minus));

                if let Expression::Binary(ref right) = expr.expr {
                    assert!(matches!(right.left, Expression::Number(7.0)));
                    assert!(matches!(right.operator, Operator::Pow));
                    assert!(matches!(right.right, Expression::Identifier(_)));
                } else {
                    assert!(false);
                }
            }
            else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    pub fn test_function_call() {
        let result = get_new_parser("check(a)greater_than(b);");
        assert_eq!(result.len(), 1);

        if let GlobalStatement::Statement(Statement::Expression(ref func_call)) = result.get(0).unwrap() {
            if let Expression::Call(ref call) = func_call.expr {
                // function args
                assert_eq!(call.args.len(), 2);

                assert!(matches!(call.args.get(0).unwrap(), Expression::Identifier(_)));
                assert!(matches!(call.args.get(1).unwrap(), Expression::Identifier(_)));

                // function name
                assert!(matches!(call.callee, Expression::Identifier(_)));
            }
            else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    pub fn test_procedure_call() {
        let result = get_new_parser("check_greater_than();");
        assert_eq!(result.len(), 1);

        if let GlobalStatement::Statement(Statement::Expression(ref func_call)) = result.get(0).unwrap() {
            if let Expression::Call(ref call) = func_call.expr {
                // function args
                assert_eq!(call.args.len(), 0);

                // function name
                assert!(matches!(call.callee, Expression::Identifier(_)));
            }
            else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    pub fn test_member() {
        let result = get_new_parser("a's b's c;");
        assert_eq!(result.len(), 1);

        if let GlobalStatement::Statement(Statement::Expression(ref func_call)) = result.get(0).unwrap() {
            if let Expression::Member(ref member) = func_call.expr {
                // function args
                assert!(matches!(member.callee, Expression::Member(_)));

                assert!(matches!(member.property, Expression::String(_)));
            }
            else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    pub fn test_member_call() {
        let result = get_new_parser("a's b();");
        assert_eq!(result.len(), 1);

        if let GlobalStatement::Statement(Statement::Expression(ref func_call)) = result.get(0).unwrap() {
            if let Expression::Call(ref call) = func_call.expr {
                if let Expression::Member(ref mem) = call.callee {
                    assert_eq!(mem.callee.get_identifier().unwrap(), "a");
                    // assert_eq!(mem.property, "b");
                } else {
                    assert!(false);
                }

                assert_eq!(call.args.len(), 0);
            }
            else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    pub fn test_member_call_intertwined() {
        let result = get_new_parser("a's b(x)c;");
        assert_eq!(result.len(), 1);

        if let GlobalStatement::Statement(Statement::Expression(ref func_call)) = result.get(0).unwrap() {
            if let Expression::Call(ref call) = func_call.expr {

                if let Expression::Member(ref mem) = call.callee {
                    assert_eq!(mem.callee.get_identifier().unwrap(), "a");
                    // assert_eq!(mem.property, format!("b{}c", FUNCTION_NAME_SEPARATOR));
                } else {
                    assert!(false);
                }

                assert_eq!(call.args.len(), 1);
            }
            else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    pub fn test_call_member() {
        let result = get_new_parser("b(x)c's a;");
        assert_eq!(result.len(), 1);

        if let GlobalStatement::Statement(Statement::Expression(ref func_call)) = result.get(0).unwrap() {
            if let Expression::Member(ref member) = func_call.expr {

                if let Expression::Call(ref call) = member.callee {
                    assert_eq!(call.callee.get_identifier().unwrap(), &format!("b{}c", FUNCTION_NAME_SEPARATOR));
                    assert_eq!(call.args.len(), 1);
                } else {
                    assert!(false);
                }

                //assert_eq!(member.property, "a");
            }
            else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    pub fn test_call_call() {
        let result = get_new_parser("(a())(v, c);");
        assert_eq!(result.len(), 1);

        if let GlobalStatement::Statement(Statement::Expression(ref func_call)) = result.get(0).unwrap() {
            if let Expression::Call(ref call) = func_call.expr {

                if let Expression::Group(ref call_call) = call.callee {
                    assert!(matches!(call_call.expr, Expression::Call(_)));
                } else {
                    assert!(false);
                }

                assert_eq!(call.args.len(), 2);
            }
            else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    pub fn test_object() {
        let result = get_new_parser("{a: 3, \"b\": a + 4, 23.2: a^2};");
        assert_eq!(result.len(), 1);

        if let GlobalStatement::Statement(Statement::Expression(ref obj)) = result.get(0).unwrap() {
            if let Expression::Object(ref object) = obj.expr {
                assert_eq!(object.properties.len(), 3);
                assert_eq!(object.values.len(), 3);
            }
            else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }
}