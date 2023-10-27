use std::{cell::RefCell, rc::Rc};

use crate::{
    label::StringLabel,
    node::{Node, NodeType},
};
use memchr::memchr2_iter;

pub trait Parser<T> {
    type TokenType;
    const TOKEN_START: T;
    const TOKEN_END: T;
    const ESCAPE_CHAR: T;

    type LabelType;
    fn parse(&self, tree_string: &str) -> Result<NodeType<Self::LabelType>, String>;
    fn new() -> Self
    where
        Self: Sized;
}

struct BracketNotationParser {}

impl BracketNotationParser {}

impl Parser<u8> for BracketNotationParser {
    type TokenType = String;
    type LabelType = String;
    const TOKEN_START: u8 = b'{';
    const TOKEN_END: u8 = b'}';
    const ESCAPE_CHAR: u8 = b'\\';

    fn parse(&self, tree_string: &str) -> Result<NodeType<Self::LabelType>, String> {
        #[inline(always)]
        fn is_escaped_char(byte_string: &[u8], offset: usize) -> bool {
            offset > 0 && byte_string[offset - 1] == BracketNotationParser::ESCAPE_CHAR
        }
        let original_tree_string = tree_string.clone();
        let tree_string = tree_string.as_bytes();

        let mut node_stack = Vec::<Rc<RefCell<Node<Self::TokenType>>>>::new();

        let token_positions: Vec<usize> =
            memchr2_iter(Self::TOKEN_START, Self::TOKEN_END, tree_string)
                .filter(|bracket_position| !is_escaped_char(tree_string, *bracket_position))
                .collect();
        // trivial check for number of brackets
        if token_positions.len() < 2 {
            return Err("Bad tree string!".to_owned());
        }

        let mut token_position_iter = token_positions.iter().peekable();
        let root_start = *token_position_iter.next().unwrap();
        let root_end = **token_position_iter.peek().unwrap();

        let root_label = StringLabel::from(&tree_string[(root_start + 1)..root_end]);
        let root_node = Node::new(Box::new(root_label));

        node_stack.push(Rc::clone(&root_node));

        while let Some(token_start_position) = token_position_iter.next() {
            match tree_string[*token_start_position] {
                Self::TOKEN_START => {
                    let Some(token_end_position) = token_position_iter.peek() else {
                        return Err("Would not find label ending token!".to_owned());
                    };
                    let label = StringLabel::from(
                        &tree_string[(*token_start_position + 1)..**token_end_position],
                    );
                    let n = Node::new(Box::new(label));
                    node_stack
                        .last()
                        .unwrap()
                        .borrow_mut()
                        .get_children_mut()
                        .push(Rc::clone(&n));
                    node_stack.push(n);
                }
                Self::TOKEN_END => {
                    node_stack
                        .pop()
                        .expect("Unexpected reach of top of node_stack");
                }
                _ => panic!("Unexpected token position!"),
            }
        }

        return Ok(root_node);
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_simple() {
        let simple_tree = r" \{ {koren{levy}{pravy}}".to_owned();
        let parser = BracketNotationParser::new();
        let parse_result = parser.parse(&simple_tree);
        assert!(parse_result.is_ok());
        let Ok(parsed_tree) = parse_result else {
            panic!("unable to parse tree");
        };

        assert!(parsed_tree.borrow().get_children().len() == 2);
        assert_eq!(
            parsed_tree.borrow().get_children()[0].borrow().is_leaf(),
            true
        );
        assert_eq!(
            parsed_tree.borrow().get_children()[1].borrow().is_leaf(),
            true
        );
    }

    #[test]
    fn it_parses_longer_trees() {
        let tree_string = r"{glaninger weg{1{}{A}}{2{{1}{2}{3}{4}{5}{6}}}{3{{3}{6}{8}}{A}}{4{{1}{2}{3}{4}}}{5{{3}{4}{5}}}{6{}{A}}{7{{1}{2}}{A{1}{2}{3}{5}}}{8{{1}{2}{3}{4}{6}{7}{8}}{A}{B{1}}}{9}{10}{11}{12{{1}}}{13{}{A}{B}}{14}{15{}{A}}{17}{18}{19{{1}}}{20}{21}}".to_owned();
        let parser = BracketNotationParser::new();
        let parse_result = parser.parse(&tree_string);
        assert!(parse_result.is_ok());
    }
}
