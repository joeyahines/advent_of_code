use aoc_helper::{PuzzleInput, PuzzlePart};
use std::iter::Iterator;
use std::str::Chars;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Operation(Operation),
    Number(i64),
    SubExpresion(Vec<Token>),
}

fn parse_tokens(expr: &mut Chars) -> Vec<Token> {
    let mut tokens = Vec::new();
    while let Some(c) = expr.next() {
        let token = match c {
            '+' => Token::Operation(Operation::Add),
            '*' => Token::Operation(Operation::Multiply),
            '(' => Token::SubExpresion(parse_tokens(expr)),
            ')' => return tokens,
            c => {
                if c.is_numeric() {
                    Token::Number(c.to_digit(10).unwrap() as i64)
                }
                else {
                    continue;
                }
            }
        };
        tokens.push(token)
    }

    tokens
}

fn operation(op: &Operation, val1: i64, val2: i64) -> i64 {
    match op {
        Operation::Add => val1 + val2,
        Operation::Multiply => val1 * val2,
    }
}


fn evaluate_expression_p1(expression: Vec<Token>) -> i64 {
    let mut acc = 0;
    let mut last_operation = Operation::Add;

    for token in expression {
        match token {
            Token::Operation(op) => last_operation = op,
            Token::Number(val) => acc = operation(&last_operation, acc, val),
            Token::SubExpresion(sub_expr) => acc =  operation(&last_operation, acc,evaluate_expression_p1(sub_expr)),
        }
    }

    acc
}


fn evaluate_expression_p2(expression: Vec<Token>) -> i64 {
    let split = expression.split(|token| { *token == Token::Operation(Operation::Multiply) });

    let sub_values: Vec<i64> = split.map(|sub_expr| {
        let mut acc = 0;
        let mut last_operation = Operation::Add;
        for token in sub_expr {
            match token {
                Token::Operation(op) => last_operation = *op,
                Token::Number(val) => acc = operation(&last_operation, acc, *val),
                Token::SubExpresion(sub_expr) => acc =  operation(&last_operation, acc,evaluate_expression_p2(sub_expr.clone())),
            }
        }
        acc
    }).collect();

    let mut acc = 1;
    for value in sub_values {
        acc *= value;
    }

    acc
}

fn main() {
    let puzzle_input = PuzzleInput::new();

    let sum: i64 =puzzle_input.input.iter().map(|expr| {
        let mut expr = expr.chars();
        let tokens = parse_tokens(&mut expr);

        if puzzle_input.part == PuzzlePart::FIRST {
            evaluate_expression_p1(tokens)
        }
        else {
            evaluate_expression_p2(tokens)
        }
    }).sum();

    println!("The sum of all expressions is {}", sum)
}
