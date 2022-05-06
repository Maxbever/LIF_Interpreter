#![feature(try_blocks)]
extern crate lazy_static;

use crate::generated_file_antlr::liflexer::lifLexer;
use crate::generated_file_antlr::liflistener::lifListener;
use crate::generated_file_antlr::lifparser;
use crate::generated_file_antlr::lifparser::{lifParser, lifParserContext};
use crate::generated_file_antlr::lifparser::{
    lifParserContextType, AttributContext, InstructionContext, TupleContext, Tuple_contentContext,
};
use crate::lifparser::{
    lifTreeWalker, ConnectContext, ConnectContextAttrs, Tuple_space_nameContext,
};
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::{ErrorNode, ParseTree, ParseTreeListener, TerminalNode};
use antlr_rust::InputStream;

mod generated_file_antlr;

struct Listener;

impl<'input> ParseTreeListener<'input, lifParserContextType> for Listener {
    fn visit_terminal(&mut self, _node: &TerminalNode<'_, lifParserContextType>) {
        todo!()
    }
    fn visit_error_node(&mut self, _node: &ErrorNode<'_, lifParserContextType>) {
        todo!()
    }
    fn enter_every_rule(&mut self, _ctx: &dyn lifParserContext<'input>) {
        println!(
            "rule entered {}",
            lifparser::ruleNames
                .get(_ctx.get_rule_index())
                .unwrap_or(&"error")
        )
    }
    fn exit_every_rule(&mut self, _ctx: &dyn lifParserContext<'input>) {
        todo!()
    }
}

impl lifListener<'_> for Listener {
    fn exit_instruction<'input>(&mut self, _ctx: &InstructionContext<'input>) {
        todo!()
    }

    fn enter_connect(&mut self, _ctx: &ConnectContext<'_>) {
        if let Some(protocol) = _ctx.protocol() {
            if let Some(ip_address) = _ctx.ip_address() {
                if let Some(port) = _ctx.port() {
                    println!(
                        "connect entered {}:{}:{}",
                        protocol.get_text(),
                        ip_address.get_text(),
                        port.get_text()
                    )
                }
            }
        }
    }
    fn enter_attribut<'input>(&mut self, _ctx: &AttributContext<'input>) {
        todo!()
    }
    fn exit_attribut<'input>(&mut self, _ctx: &AttributContext<'input>) {
        todo!()
    }
    fn enter_tuple<'input>(&mut self, _ctx: &TupleContext<'input>) {
        todo!()
    }
    fn exit_tuple<'input>(&mut self, _ctx: &TupleContext<'input>) {
        todo!()
    }
    fn enter_tuple_content<'input>(&mut self, _ctx: &Tuple_contentContext<'input>) {
        todo!()
    }
    fn exit_tuple_content<'input>(&mut self, _ctx: &Tuple_contentContext<'input>) {
        todo!()
    }
    fn enter_tuple_space_name(&mut self, _ctx: &Tuple_space_nameContext<'_>) {
        todo!()
    }
}

fn main() {
    let lexer = lifLexer::new(InputStream::new("connect tcp:127.0.0.1:9500"));
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = lifParser::new(token_source);
    println!("\nstart parsing lif");
    lifTreeWalker::walk(Box::new(Listener {}), &*parser.root().unwrap());
}
