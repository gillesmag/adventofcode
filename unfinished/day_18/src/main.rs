use std::iter::Peekable;

#[derive(Debug)]
struct ParseNode {
    children: Vec<ParseNode>,
}

#[derive(Debug)]
enum LexItem {
    Paren,
    Comma,
    Num(u32),
}

fn lex(input: &str) -> Vec<LexItem> {
    let mut result = vec![];

    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' => {
                it.next();
                let n = get_number(c, &mut it);
                result.push(LexItem::Num(n));
            }
            '[' | ']' => {
                result.push(LexItem::Paren);
                it.next();
            }
            ',' => {
                result.push(LexItem::Comma);
                it.next();
            }
            _ => unreachable!("Invalid characters in input string"),
        }
    }
    result
}

fn parse_expr(tokens: &Vec<LexItem>, pos: usize) -> (ParseNode, usize) {
    let c = tokens.get(pos);
    match c {
        &LexItem::Num(n) => {
            let mut node = ParseNode::new();
            node.entry = GrammarItem::Number(n);
            (node, pos + 1)
        }
        &LexItem::Paren => {
            let child = parse_expr(tokens, pos+1)
}

fn parse(input: &str) -> ParseNode {
    let tokens = lex(input);
    println!("{:?}", tokens);
    ParseNode { children: vec![] }
}

fn get_number<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> u32 {
    let mut number = c.to_string().parse::<u32>().unwrap();
    while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<u32>()) {
        number = number * 10 + digit;
        iter.next();
    }
    number
}

fn main() {
    let input = "[1332,2]";
    println!("{:?}", parse(input));
}
