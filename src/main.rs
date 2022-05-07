#![feature(try_blocks)]
#![feature(receiver_trait)]
extern crate lazy_static;

use crate::constant::{ATTACH, CREATE, OUT, SPACE};
use crate::generated_file_antlr::liflexer::lifLexer;
use crate::generated_file_antlr::liflistener::lifListener;
use crate::generated_file_antlr::lifparser;
use crate::generated_file_antlr::lifparser::{lifParser, lifParserContext};
use crate::generated_file_antlr::lifparser::{
    lifParserContextType, AttributContext, InstructionContext, TupleContext, Tuple_contentContext,
};
use crate::lifparser::{
    lifTreeWalker, AttachContext, AttachContextAttrs, ConnectContext, ConnectContextAttrs,
    CreateContext, CreateContextAttrs, DeleteContext, In_instrContext, Init_varContext,
    Ip_addressContext, OutContext, OutContextAttrs, PortContext, ProtocolContext, ReadContext,
    RootContext, Tuple_space_nameContext,
};
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::{ParseTree, ParseTreeListener};
use antlr_rust::InputStream;
use server::Server;
use std::collections::HashMap;
use std::{env, fs};

mod constant;
mod generated_file_antlr;
mod server;

struct Listener {
    server_list: HashMap<String, Server>,
}

impl Listener {
    pub fn new<'a>() -> Listener {
        Listener {
            server_list: HashMap::with_capacity(64),
        }
    }
}

impl<'input> ParseTreeListener<'input, lifParserContextType> for Listener {
    // fn visit_terminal(&mut self, _node: &TerminalNode<'_, lifParserContextType>) {
    //     todo!()
    // }
    // fn visit_error_node(&mut self, _node: &ErrorNode<'_, lifParserContextType>) {
    //     todo!()
    // }
    // fn enter_every_rule(&mut self, _ctx: &dyn lifParserContext<'input>) {
    //     println!(
    //         "rule entered {}",
    //         lifparser::ruleNames
    //             .get(_ctx.get_rule_index())
    //             .unwrap_or(&"error")
    //     )
    // }
    fn exit_every_rule(&mut self, _ctx: &dyn lifParserContext<'input>) {
        todo!()
    }
}

impl lifListener<'_> for Listener {
    fn enter_connect(&mut self, _ctx: &ConnectContext<'_>) {
        if let Some(protocol) = _ctx.protocol() {
            if let Some(ip_address) = _ctx.ip_address() {
                if let Some(port) = _ctx.port() {
                    let server =
                        Server::new(ip_address.get_text(), port.get_text(), protocol.get_text());
                    self.server_list.insert("test".parse().unwrap(), server); //TODO Manage server list
                }
            }
        }
    }
    fn enter_create(&mut self, _ctx: &CreateContext<'_>) {
        if let Some(attribute) = _ctx.attribut(0) {
            if let Some(tuple_space_name) = _ctx.tuple_space_name() {
                let server = self.server_list.get(&*String::from("test"));
                match server {
                    None => {}
                    Some(server) => {
                        if let Some(_) = _ctx.attribut(1) {
                            let mut attribute_list: String = String::new();
                            let mut list = _ctx.attribut_all();
                            list.remove(0);
                            for attribute in list {
                                attribute_list += &attribute.get_text();
                            }
                            println!(
                                "{}",
                                server.send_message(
                                    String::from(CREATE)
                                        + SPACE
                                        + &*attribute.get_text()
                                        + SPACE
                                        + &*tuple_space_name.get_text()
                                        + SPACE
                                        + &*attribute_list
                                )
                            );
                        } else {
                            println!(
                                "{}",
                                server.send_message(
                                    String::from(CREATE)
                                        + SPACE
                                        + &*attribute.get_text()
                                        + SPACE
                                        + &*tuple_space_name.get_text()
                                )
                            );
                        }
                    }
                }
            }
        }
    }

    fn enter_attach(&mut self, _ctx: &AttachContext<'_>) {
        if let Some(tuple_space_name) = _ctx.tuple_space_name() {
            let server = self.server_list.get(&*String::from("test"));
            match server {
                None => {}
                Some(server) => {
                    if let Some(_) = _ctx.attribut(0) {
                        let mut attribute_list: String = String::new();
                        for attribute in _ctx.attribut_all() {
                            attribute_list += &attribute.get_text();
                        }
                        println!(
                            "{}",
                            server.send_message(
                                String::from(ATTACH)
                                    + SPACE
                                    + &*tuple_space_name.get_text()
                                    + SPACE
                                    + &*attribute_list
                            )
                        );
                    } else {
                        println!(
                            "{}",
                            server.send_message(
                                String::from(ATTACH) + SPACE + &*tuple_space_name.get_text()
                            )
                        );
                    }
                }
            }
        }
    }

    fn enter_out(&mut self, _ctx: &OutContext<'_>) {
        if let Some(tuple) = _ctx.tuple() {
            let server = self.server_list.get(&*String::from("test"));
            match server {
                None => {}
                Some(server) => {
                    println!(
                        "{}",
                        server.send_message(String::from(OUT) + SPACE + &*tuple.get_text())
                    );
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    match fs::read_to_string(filename) {
        Ok(content) => {
            let lexer = lifLexer::new(InputStream::new(content.as_str()));
            let token_source = CommonTokenStream::new(lexer);
            let mut parser = lifParser::new(token_source);
            println!("Start parsing lif");
            lifTreeWalker::walk(Box::new(Listener::new()), &*parser.root().unwrap());
        }
        Err(error) => {
            println!("{}", error)
        }
    };
}
