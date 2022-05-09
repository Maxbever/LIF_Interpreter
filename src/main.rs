#![feature(try_blocks)]
#![feature(receiver_trait)]
extern crate core;
extern crate lazy_static;

use crate::constant::{ATTACH, CREATE, DELETE, IN, OUT, READ, SPACE};
use crate::generated_file_antlr::liflexer::lifLexer;
use crate::generated_file_antlr::liflistener::lifListener;
use crate::generated_file_antlr::lifparser;
use crate::generated_file_antlr::lifparser::lifParserContextType;
use crate::generated_file_antlr::lifparser::{lifParser, lifParserContext};
use crate::lifparser::{lifTreeWalker, AssignationContext, AttachContext, AttachContextAttrs, AttributContext, ConnectContext, ConnectContextAttrs, CreateContext, CreateContextAttrs, DeleteContext, DeleteContextAttrs, In_instrContext, In_instrContextAttrs, Init_varContext, Init_varContextAttrs, InstructionContext, Ip_addressContext, OutContext, OutContextAttrs, PortContext, ProtocolContext, ReadContext, ReadContextAttrs, RootContext, TupleContext, Tuple_contentContext, Tuple_space_nameContext, AssignationContextAttrs, Init_varContextAll};
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::{ParseTree, ParseTreeListener};
use antlr_rust::InputStream;
use server::Server;
use std::collections::HashMap;
use std::{env, fs};
use std::rc::Rc;

mod constant;
mod generated_file_antlr;
mod server;

struct Listener {
    server_list: HashMap<String, Server>,
    symbol_table: HashMap<String, Value>
}

enum Value {
    String(String),
    Number(u32),
    Tuple(Vec<Value>),
    Char(char),
    ID(Box<Value>),
    Error(String)
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Value::String(val) => {
                Value::String(val.clone())
            }
            Value::Number(val) => {
                Value::Number(val.clone())
            }
            Value::Tuple(val) => {
                Value::Tuple(val.clone())
            }
            Value::Char(val) => {
                Value::Char(val.clone())
            }
            Value::ID(val) => {
                Value::ID(val.clone())
            }
            Value::Error(val) => {
                Value::Error(val.clone())
            }
        }
    }
}

enum TupleOperationContext<'input, 'a> {
    OutContext(&'a OutContext<'input>),
    InContext(&'a In_instrContext<'input>),
    ReadContext(&'a ReadContext<'input>),
}

impl Listener {
    pub fn new() -> Listener {
        Listener {
            server_list: HashMap::new(),
            symbol_table: HashMap::new(),
        }
    }

    fn manage_tuple_operation(&mut self, _ctx: TupleOperationContext, operation: &str) {
        match _ctx {
            TupleOperationContext::OutContext(context) => {
                if let Some(_) = context.tuple(0) {
                    let server = self.server_list.get(&*String::from("test"));
                    match server {
                        None => {}
                        Some(server) => {
                            let mut tuple_list: String = String::new();
                            for tuple in context.tuple_all() {
                                tuple_list += &tuple.get_text();
                            }
                            println!(
                                "{}",
                                server.send_message(String::from(operation) + SPACE + &*tuple_list)
                            );
                        }
                    }
                }
            }
            TupleOperationContext::InContext(context) => {
                if let Some(_) = context.tuple(0) {
                    let server = self.server_list.get(&*String::from("test"));
                    match server {
                        None => {}
                        Some(server) => {
                            let mut tuple_list: String = String::new();
                            for tuple in context.tuple_all() {
                                tuple_list += &tuple.get_text();
                            }
                            println!(
                                "{}",
                                server.send_message(String::from(operation) + SPACE + &*tuple_list)
                            );
                        }
                    }
                }
            }
            TupleOperationContext::ReadContext(context) => {
                if let Some(_) = context.tuple(0) {
                    let server = self.server_list.get(&*String::from("test"));
                    match server {
                        None => {}
                        Some(server) => {
                            let mut tuple_list: String = String::new();
                            for tuple in context.tuple_all() {
                                tuple_list += &tuple.get_text();
                            }
                            println!(
                                "{}",
                                server.send_message(String::from(operation) + SPACE + &*tuple_list)
                            );
                        }
                    }
                }
            }
        };
    }

    fn enter_init_var(&mut self, _ctx: Rc<Init_varContextAll>) -> Value {
        let mut error_message = String::from("Error in vairable assignation");
        if let Some(string_context) = _ctx.STRING() {
            return Value::String(string_context.get_text());
        } else if let Some(number_context) = _ctx.NUMBER() {
            return Value::Number(number_context.get_text().parse::<u32>().unwrap());
        } else if let Some(tuple_context) = _ctx.tuple() {
            return Value::Tuple(Vec::new()); //TODO
        } else if let Some(id_context) = _ctx.ID() {
            let id_variable: String = id_context.get_text();
            match self.symbol_table.remove(&id_variable) {
                None => {
                    error_message = format!("Variable {} doesn't exist", id_context.get_text());
                }
                Some(variable) => {
                    self.symbol_table.insert(id_variable, variable.clone());
                    return Value::ID(Box::from(variable));
                }
            }
        } else if let Some(char_context) = _ctx.CHARACTER() {
            return Value::Char(char_context.get_text().chars().nth(0).unwrap());
        }
        return Value::Error(error_message)
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
                                attribute_list += &*(" ".to_string() + &attribute.get_text());
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

    fn enter_delete(&mut self, _ctx: &DeleteContext<'_>) {
        if let Some(tuple_space_name) = _ctx.tuple_space_name() {
            let server = self.server_list.get(&*String::from("test"));
            match server {
                None => {}
                Some(server) => {
                    if let Some(attribute) = _ctx.attribut() {
                        println!(
                            "{}",
                            server.send_message(
                                String::from(DELETE)
                                    + SPACE
                                    + &*attribute.get_text()
                                    + SPACE
                                    + &*tuple_space_name.get_text()
                            )
                        );
                    } else {
                        println!(
                            "{}",
                            server.send_message(
                                String::from(DELETE) + SPACE + &*tuple_space_name.get_text()
                            )
                        );
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
                            attribute_list += &*(" ".to_string() + &attribute.get_text());
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

    fn enter_read(&mut self, _ctx: &ReadContext<'_>) {
        self.manage_tuple_operation(TupleOperationContext::ReadContext(_ctx), READ);
    }

    fn enter_in_instr(&mut self, _ctx: &In_instrContext<'_>) {
        self.manage_tuple_operation(TupleOperationContext::InContext(_ctx), IN);
    }

    fn enter_out(&mut self, _ctx: &OutContext<'_>) {
        self.manage_tuple_operation(TupleOperationContext::OutContext(_ctx), OUT);
    }

    fn enter_assignation(&mut self, _ctx: &AssignationContext<'_>) {
        if let Some(id_context) = _ctx.ID(){
            if let Some(init_var_context) = _ctx.init_var(){
                let value:Value = self.enter_init_var(init_var_context);
                &self.symbol_table.insert(id_context.get_text(),value);
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
