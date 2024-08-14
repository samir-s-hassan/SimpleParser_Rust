// Here is where the various combinators are imported. You can find all the combinators here:
// https://docs.rs/nom/7.1.3/nom/
// Samir Hassan

use nom::*;
use nom::{
    IResult,
    branch::alt,
    combinator::opt,
    combinator::map,
    multi::{ many1, many0 },
    bytes::complete::{ tag, take_until, take_while },
    character::complete::{ alphanumeric1, digit1, line_ending, not_line_ending },
    sequence::{ preceded, tuple, delimited, pair },
};
use crate::lexer::*;

// Here are the different node types. You will use these to make your parser.
// You may add other nodes as you see fit.

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Program {
        children: Vec<Node>,
    },
    Statement {
        children: Vec<Node>,
    },
    FunctionDefine {
        name: Vec<u8>,
        children: Vec<Node>,
    },
    FunctionArguments {
        children: Vec<Node>,
    },
    FunctionStatements {
        children: Vec<Node>,
    },
    Expression {
        children: Vec<Node>,
    },
    MathExpression {
        name: Vec<u8>,
        children: Vec<Node>,
    },
    FunctionCall {
        name: Vec<u8>,
        children: Vec<Node>,
    },
    VariableDefine {
        children: Vec<Node>,
    },
    FunctionReturn {
        children: Vec<Node>,
    },
    Number {
        value: Vec<u8>,
    },
    Bool {
        value: bool,
    },
    Identifier {
        value: Vec<u8>,
    },
    String {
        value: Vec<u8>,
    },
    Null,
}

// <----------------- START OF HELPERS ----------------->
// Some helper functions to use Tokens instead of a &str with Nom.
// You'll probably have to create more of these as needed.

pub fn t_alpha(input: Tokens) -> IResult<Tokens, Token> {
    let fxn = check_token(
        &(|tk| {
            match tk.kind {
                TokenKind::Alpha => true,
                _ => false,
            }
        })
    );
    fxn(input.clone())
}

pub fn t_digit(input: Tokens) -> IResult<Tokens, Token> {
    let fxn = check_token(
        &(|tk| {
            match tk.kind {
                TokenKind::Digit => true,
                _ => false,
            }
        })
    );
    fxn(input.clone())
}

pub fn t_alpha1(input: Tokens) -> IResult<Tokens, Vec<Token>> {
    many1(t_alpha)(input)
}

pub fn t_alpha0(input: Tokens) -> IResult<Tokens, Vec<Token>> {
    many0(t_alpha)(input)
}

pub fn t_alphanumeric1(input: Tokens) -> IResult<Tokens, Vec<Token>> {
    many1(alt((t_alpha, t_digit)))(input)
}

pub fn t_alphanumeric0(input: Tokens) -> IResult<Tokens, Vec<Token>> {
    many0(alt((t_alpha, t_digit)))(input)
}

pub fn t_alphanumeric(input: Tokens) -> IResult<Tokens, Token> {
    let fxn = check_token(&(|tk| matches!(tk.kind, TokenKind::Alpha | TokenKind::Digit)));
    fxn(input.clone())
}

pub fn t_space(input: Tokens) -> IResult<Tokens, Token> {
    let fxn = check_token(&(|tk| matches!(tk.kind, TokenKind::WhiteSpace)));
    fxn(input.clone())
}

pub fn t_quote(input: Tokens) -> IResult<Tokens, Token> {
    let fxn = check_token(&(|tk| matches!(tk.kind, TokenKind::Quote)));
    fxn(input.clone())
}

//created these helper functions for my math expression
pub fn t_plus(input: Tokens) -> IResult<Tokens, Token> {
    let fxn = check_token(
        &(|tk| {
            match tk.kind {
                TokenKind::Plus => true,
                _ => false,
            }
        })
    );
    fxn(input.clone())
}

pub fn t_minus(input: Tokens) -> IResult<Tokens, Token> {
    let fxn = check_token(
        &(|tk| {
            match tk.kind {
                TokenKind::Dash => true,
                _ => false,
            }
        })
    );
    fxn(input.clone())
}

pub fn t_equal(input: Tokens) -> IResult<Tokens, Token> {
    let fxn = check_token(
        &(|tk| {
            match tk.kind {
                TokenKind::Equal => true,
                _ => false,
            }
        })
    );
    fxn(input.clone())
}

pub fn t_semicolon(input: Tokens) -> IResult<Tokens, Token> {
    let fxn = check_token(
        &(|tk| {
            match tk.kind {
                TokenKind::Semicolon => true,
                _ => false,
            }
        })
    );
    fxn(input.clone())
}

pub fn t_rightparen(input: Tokens) -> IResult<Tokens, Token> {
    let fxn = check_token(
        &(|tk| {
            match tk.kind {
                TokenKind::RightParen => true,
                _ => false,
            }
        })
    );
    fxn(input.clone())
}

pub fn t_rightbrace(input: Tokens) -> IResult<Tokens, Token> {
    let fxn = check_token(
        &(|tk| {
            match tk.kind {
                TokenKind::RightCurly => true,
                _ => false,
            }
        })
    );
    fxn(input.clone())
}

pub fn t_leftbrace(input: Tokens) -> IResult<Tokens, Token> {
    let fxn = check_token(
        &(|tk| {
            match tk.kind {
                TokenKind::LeftCurly => true,
                _ => false,
            }
        })
    );
    fxn(input.clone())
}

pub fn t_leftparen(input: Tokens) -> IResult<Tokens, Token> {
    let fxn = check_token(
        &(|tk| {
            match tk.kind {
                TokenKind::LeftParen => true,
                _ => false,
            }
        })
    );
    fxn(input.clone())
}

// keywords

pub fn t_fn(input: Tokens) -> IResult<Tokens, Token> {
    let fxn = check_token(
        &(|tk| {
            match tk.kind {
                TokenKind::Fn => true,
                _ => false,
            }
        })
    );
    fxn(input.clone())
}

pub fn t_let(input: Tokens) -> IResult<Tokens, Token> {
    let fxn = check_token(
        &(|tk| {
            match tk.kind {
                TokenKind::Let => true,
                _ => false,
            }
        })
    );
    fxn(input.clone())
}

pub fn t_return(input: Tokens) -> IResult<Tokens, Token> {
    let fxn = check_token(
        &(|tk| {
            match tk.kind {
                TokenKind::Return => true,
                _ => false,
            }
        })
    );
    fxn(input.clone())
}

//created a t_true and t_false helper functions
pub fn t_true(input: Tokens) -> IResult<Tokens, Token> {
    let fxn = check_token(
        &(|tk| {
            match tk.kind {
                TokenKind::True => true,
                _ => false,
            }
        })
    );
    fxn(input.clone())
}

pub fn t_false(input: Tokens) -> IResult<Tokens, Token> {
    let fxn = check_token(
        &(|tk| {
            match tk.kind {
                TokenKind::False => true,
                _ => false,
            }
        })
    );
    fxn(input.clone())
}

// <----------------- START OF PARSERS ----------------->

// identifier = alpha , <alnum> ;
pub fn identifier(input: Tokens) -> IResult<Tokens, Node> {
    let (input, first) = t_alpha(input)?;
    let (input, rest) = t_alphanumeric0(input)?;
    let mut identifier = first.lexeme;
    for mut tk in rest {
        identifier.append(&mut tk.lexeme);
    }
    Ok((input, Node::Identifier { value: identifier }))
}

// number = {digit} ;
pub fn number(input: Tokens) -> IResult<Tokens, Node> {
    let (input, num_tokens) = many1(t_digit)(input)?; //parse one or more digit tokens
    let value: Vec<u8> = num_tokens
        .iter()
        .flat_map(|token| token.lexeme.iter().copied())
        .collect(); //iterate over the bytes of the lexeme and copy them into a collection
    Ok((input, Node::Number { value }))
}

// boolean = "true" | "false" ;
pub fn boolean(input: Tokens) -> IResult<Tokens, Node> {
    alt((
        map(t_true, |_| Node::Bool { value: true }),
        map(t_false, |_| Node::Bool { value: false }),
    ))(input)
}

// string = "\"" , {alnum | " "} , "\"" ;
pub fn string(input: Tokens) -> IResult<Tokens, Node> {
    let (input, _) = t_quote(input)?;
    let (input, content_tokens) = many0(alt((t_alphanumeric, t_space)))(input)?;
    let (input, _) = t_quote(input)?;

    let value: Vec<u8> = content_tokens
        .iter()
        .flat_map(|token| token.lexeme.iter().copied())
        .collect();

    Ok((input, Node::String { value }))
}

// function_call = identifier , "(" , [arguments] , ")" ;
pub fn function_call(input: Tokens) -> IResult<Tokens, Node> {
    let (input, ident) = identifier(input)?;
    let (input, _) = t_leftparen(input)?;
    let (input, arg) = opt(arguments)(input)?;
    let (input, _) = t_rightparen(input)?;

    // collect the arguments into a Vec<Node>
    let children = arg.map_or_else(
        || vec![Node::FunctionArguments { children: vec![] }],
        |node| vec![node]
    );

    // extract the identifier name
    let name = match ident {
        Node::Identifier { value } => value,
        _ => panic!("Unexpected Node Type"),
    };

    Ok((input, Node::FunctionCall { name, children }))
}

// value = number | identifier | boolean ;
pub fn value(input: Tokens) -> IResult<Tokens, Node> {
    alt((identifier, number, boolean))(input)
}

// math_expression = value , { ("+" | "-") , value } ;
pub fn math_expression(input: Tokens) -> IResult<Tokens, Node> {
    // parse the first value
    let (input, first_value) = value(input)?;

    // parse zero or more pairs of the operator + or - and its value
    let (input, remaining_values) = many0(
        pair(
            alt((map(t_plus, |_| '+'), map(t_minus, |_| '-'))), // get the operator in as a character
            value
        )
    )(input)?;

    // check if there's only one value and no operations
    if remaining_values.is_empty() {
        return Ok((input, first_value));
    }

    // create the children vector
    let mut children = vec![first_value];

    // add the remaining values to the children vector
    for (_, next_value) in remaining_values {
        children.push(next_value);
    }

    // return the MathExpression node with the children vector
    Ok((input, Node::MathExpression { name: vec![97, 100, 100], children }))
}

// expression = boolean | math_expression | function_call | number | string | identifier ;
pub fn expression(input: Tokens) -> IResult<Tokens, Node> {
    alt((boolean, math_expression, function_call, number, string, identifier))(input)
}

// statement = variable_define , ";" | function_return , ";" ;
pub fn statement(input: Tokens) -> IResult<Tokens, Node> {
    let (input, statements) = many1(alt((variable_define, function_return)))(input)?;
    let (input, _) = t_semicolon(input)?;

    Ok((input, Node::FunctionStatements { children: statements }))
}

// function_return = "return" , (function_call | expression | identifier) ;
pub fn function_return(input: Tokens) -> IResult<Tokens, Node> {
    let (input, _) = t_return(input)?; //parse the "return" keyword
    let (input, value) = alt((function_call, expression, identifier))(input)?; // parse either a function call, an expression, or an identifier using the alt
    Ok((input, Node::FunctionReturn { children: vec![value] }))
}

// variable_define = "let" , identifier , "=" , expression ;
pub fn variable_define(input: Tokens) -> IResult<Tokens, Node> {
    let (input, _) = t_let(input)?; // parse the "let" keyword
    let (input, name) = identifier(input)?; // parse the variable name (identifier)
    let (input, _) = t_equal(input)?; // parse the "=" sign
    let (input, value) = expression(input)?; // parse the initial value expression

    // make the VariableDefine node with the variable name and its initial value
    let node = Node::VariableDefine {
        children: vec![name, value],
    };

    Ok((input, node))
}

// arguments = expression , { "," , expression } ;
pub fn arguments(input: Tokens) -> IResult<Tokens, Node> {
    let (input, expressions) = many0(
        delimited(
            t_space, // there might be spaces before each expression
            expression, // parse an expression
            t_space // there might be spaces after each expression
        )
    )(input)?;

    // make the FunctionArguments node with the parsed expressions as children
    Ok((input, Node::FunctionArguments { children: expressions }))
}

// function_define = "fn" , identifier , "(" , [arguments] , ")" , "{" , <statement> , "}" ;
pub fn function_define(input: Tokens) -> IResult<Tokens, Node> {
    let (input, _) = t_fn(input)?;
    let (input, first_token) = t_alpha(input)?;
    let (input, remaining_tokens) = t_alphanumeric0(input)?;
    let (input, _) = t_leftparen(input)?;
    let (input, arguments_node) = arguments(input)?;
    let (input, _) = t_rightparen(input)?;
    let (input, _) = t_leftbrace(input)?;
    let (input, statements_nodes) = many1(statement)(input)?;
    let (input, _) = t_rightbrace(input)?;

    let mut token_identifier = first_token.lexeme;
    for mut token in remaining_tokens {
        token_identifier.append(&mut token.lexeme);
    }

    let mut children = vec![arguments_node];
    children.extend(statements_nodes);

    Ok((input, Node::FunctionDefine { name: token_identifier, children }))
}

// comment = "//" , (?any-character? - newline);
pub fn comment(input: Tokens) -> IResult<Tokens, Node> {
    unimplemented!();
}

// program = {function_definition} ;
pub fn program(input: Tokens) -> IResult<Tokens, Node> {
    let (input, function_nodes) = many0(function_define)(input)?;
    Ok((input, Node::Program { children: function_nodes }))
}
