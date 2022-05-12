#![feature(try_blocks)]
#![feature(receiver_trait)]
extern crate core;
extern crate lazy_static;

use crate::constant::{ATTACH, CREATE, DELETE, GET_FUNCTION, IN, LEN_FUNCTION, OUT, READ, SPACE};
use crate::generated_file_antlr::liflexer::lifLexer;
use crate::generated_file_antlr::liflistener::lifListener;
use crate::generated_file_antlr::lifparser;
use crate::generated_file_antlr::lifparser::lifParser;
use crate::generated_file_antlr::lifparser::lifParserContextType;
use crate::lifparser::{lifTreeWalker, AssignationContext, AssignationContextAttrs, AttachContext, AttachContextAttrs, AttributContextAll, AttributContextAttrs, ConnectContext, ConnectContextAttrs, CreateContext, CreateContextAttrs, DeleteContext, DeleteContextAttrs, In_instrContext, In_instrContextAttrs, Init_varContextAll, Init_varContextAttrs, OutContext, OutContextAttrs, ReadContext, ReadContextAttrs, TupleContextAll, TupleContextAttrs, Tuple_contentContextAll, Tuple_contentContextAttrs, Tuple_space_nameContextAll, Tuple_space_nameContextAttrs, Get_functionContextAttrs, Len_functionContextAttrs, OperationContextAttrs, OperationContextAll, Right_exprContextAll, Right_exprContextAttrs};
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::{ParseTree, ParseTreeListener, TerminalNode};
use antlr_rust::InputStream;
use server::Server;
use std::collections::HashMap;
use std::rc::Rc;
use std::{env, fs};
use antlr_rust::rule_context::CustomRuleContext;

mod constant;
mod generated_file_antlr;
mod server;

struct Listener {
    server_list: HashMap<String, Server>,
    symbol_table: HashMap<String, Value>,
}

enum Value {
    String(String),
    Number(i64),
    Tuple(Vec<Value>),
    Char(char),
    ID(Box<Value>),
}

impl Value {
    fn get_value(&self) -> String {
        match self {
            Value::String(value) => "\"".to_owned() + &value.clone() + &*"\"",
            Value::Number(value) => value.to_string(),
            Value::Tuple(value) => {
                let mut tuple_string: String = String::from('(');
                for i in 0..value.len() {
                    let value_tuple = value.get(i).unwrap();
                    tuple_string += &*value_tuple.get_value();
                    if i != value.len() - 1 {
                        tuple_string.push(',')
                    }
                }
                tuple_string += ")";
                tuple_string
            }
            Value::Char(value) => value.to_string(),
            Value::ID(value) => value.get_value(),
        }
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Value::String(val) => Value::String(val.clone()),
            Value::Number(val) => Value::Number(val.clone()),
            Value::Tuple(val) => Value::Tuple(val.clone()),
            Value::Char(val) => Value::Char(val.clone()),
            Value::ID(val) => Value::ID(val.clone()),
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

    fn manage_tuple_operation(&mut self, _ctx: TupleOperationContext, operation: &str) -> String {
        match _ctx {
            TupleOperationContext::OutContext(context) => {
                if let Some(_) = context.tuple(0) {
                    let server_name = String::from("test");
                    let server = self.server_list.remove(&*server_name);
                    match server {
                        None => {}
                        Some(server) => {
                            let mut tuple_list: String = String::new();
                            for tuple in context.tuple_all() {
                                let tuple_value = self.validate_tuple(tuple).clone();
                                tuple_list.push_str(&*tuple_value);
                            }
                            let response =
                                server.send_message(String::from(operation) + SPACE + &*tuple_list);
                            println!("{}", response);
                            &self.server_list.insert(server_name, server);
                            return response;
                        }
                    }
                }
            }
            TupleOperationContext::InContext(context) => {
                if let Some(_) = context.tuple(0) {
                    let server_name = String::from("test");
                    let server = self.server_list.remove(&*server_name);
                    match server {
                        None => {}
                        Some(server) => {
                            let mut tuple_list: String = String::new();
                            for tuple in context.tuple_all() {
                                tuple_list += &*self.validate_tuple(tuple);
                            }
                            let response =
                                server.send_message(String::from(operation) + SPACE + &*tuple_list);
                            println!("{}", response);
                            &self.server_list.insert(server_name, server);
                            return response;
                        }
                    }
                }
            }
            TupleOperationContext::ReadContext(context) => {
                if let Some(_) = context.tuple(0) {
                    let server_name = String::from("test");
                    let server = self.server_list.remove(&*server_name);
                    match server {
                        None => {}
                        Some(server) => {
                            let mut tuple_list: String = String::new();
                            for tuple in context.tuple_all() {
                                tuple_list += &*self.validate_tuple(tuple);
                            }
                            let response =
                                server.send_message(String::from(operation) + SPACE + &*tuple_list);
                            println!("{}", response);
                            &self.server_list.insert(server_name, server);
                            return response;
                        }
                    }
                }
            }
        }
        panic!("Error in the tuple operation");
    }

    fn enter_init_var(&mut self, _ctx: Rc<Init_varContextAll>) -> Value {
        if let Some(string_context) = _ctx.STRING() {
            let mut string = string_context.get_text();
            string.pop();
            string.remove(0);
            return Value::String(string);
        } else if let Some(number_context) = _ctx.NUMBER() {
            return Value::Number(number_context.get_text().parse::<i64>().unwrap());
        } else if let Some(tuple_context) = _ctx.tuple() {
            return if let Some(id_context) = tuple_context.ID() {
                Value::Tuple(vec![self.add_variable_in_variable(id_context)])
            } else {
                let mut tuple_list: Vec<Value> = Vec::new();
                for tuple in tuple_context.tuple_content_all() {
                    let mut tuple_value: Value;
                    if let Some(_) = tuple.WILDCARD() {
                        tuple_value = Value::Char('_');
                    } else {
                        tuple_value = self.enter_init_var(tuple.init_var().unwrap())
                    }
                    tuple_list.push(tuple_value);
                }
                Value::Tuple(tuple_list)
            };
        } else if let Some(id_context) = _ctx.ID() {
            return self.add_variable_in_variable(id_context);
        } else if let Some(char_context) = _ctx.CHARACTER() {
            return Value::Char(char_context.get_text().chars().nth(0).unwrap());
        }
        panic!("Variable not well defined")
    }

    fn add_variable_in_variable(
        &mut self,
        id_context: Rc<TerminalNode<lifParserContextType>>,
    ) -> Value {
        let id_variable: String = id_context.get_text();
        match self.symbol_table.remove(&id_variable) {
            None => {
                panic!("Variable {} doesn't exist", id_context.get_text());
            }
            Some(variable) => {
                self.symbol_table.insert(id_variable, variable.clone());
                return Value::ID(Box::from(variable));
            }
        }
    }

    fn validate_attribute(&self, attribute: Rc<AttributContextAll>) -> String {
        return if let Some(id_attribute) = attribute.ID() {
            self.validate_variable(id_attribute)
        } else {
            attribute.get_text()
        };
    }

    fn validate_tuple_name(&self, attribute: Rc<Tuple_space_nameContextAll>) -> String {
        return if let Some(id_attribute) = attribute.ID() {
            self.validate_variable(id_attribute)
        } else {
            attribute.get_text()
        };
    }

    fn validate_variable(&self, id_attribute: Rc<TerminalNode<lifParserContextType>>) -> String {
        let id_variable = id_attribute.get_text();
        match self.symbol_table.get(&id_variable) {
            None => {
                panic!("Variable {} not found", &id_variable)
            }
            Some(variable) => match variable {
                Value::String(_) | Value::ID(_) => {
                    return variable.get_value();
                }
                Value::Number(_) | Value::Tuple(_) | Value::Char(_) => {
                    panic!("Value not available for an attribute")
                }
            },
        }
    }

    fn validate_tuple(&mut self, all_tuple: Rc<TupleContextAll>) -> String {
        if let Some(id_tuple) = all_tuple.ID() {
            let id_variable = id_tuple.get_text();
            match self.symbol_table.get(&id_variable) {
                None => {
                    panic!("Variable {} not found", &id_variable)
                }
                Some(variable) => match variable {
                    Value::Tuple(_) | Value::ID(_) => {
                        return variable.get_value();
                    }
                    Value::Number(_) | Value::String(_) | Value::Char(_) => {
                        panic!("Value not available for an attribute")
                    }
                },
            }
        } else {
            let mut tuple_list: String = String::from("(");
            for i in 0..all_tuple.tuple_content_all().len() {
                if let Some(tuple) = all_tuple.tuple_content(i) {
                    let mut tuple_value: String = String::new();
                    if let Some(_) = tuple.WILDCARD() {
                        tuple_value += "_";
                    } else {
                        tuple_value += &*self.enter_init_var(tuple.init_var().unwrap()).get_value();
                    }
                    if i != all_tuple.tuple_content_all().len() - 1 {
                        tuple_value.push_str(",");
                    }
                    tuple_list += &*tuple_value;
                }
            }
            return tuple_list + ")";
        }
    }

    fn parse_tuple(&mut self, response: String) -> Value {
        let lexer = lifLexer::new(InputStream::new(response.as_str()));
        let token_source = CommonTokenStream::new(lexer);
        let mut parser = lifParser::new(token_source);
        return self.enter_init_var(parser.init_var().unwrap());
    }

    fn function_on_tuple(&mut self, tuple_context: Rc<TupleContextAll>, function : &str, index: Option<usize>, id_context: String){
            let value;
            if let Some(variable) = tuple_context.ID() {
                value = self.symbol_table.remove(&*variable.get_text()).unwrap();
                self.symbol_table.insert(variable.get_text(),value.clone());
            } else {
                value = self.parse_tuple(tuple_context.get_text());
            }
            match value {
                Value::Tuple(tuple_value) => {
                    let mut vec_temp = tuple_value.clone();
                    if function == LEN_FUNCTION{
                        let _ = &self.symbol_table.insert(
                            id_context,
                            Value::Number(vec_temp.len() as i64),
                        );
                    }else {
                        let _ = &self.symbol_table.insert(
                            id_context,
                            vec_temp.remove(index.unwrap()),
                        );
                    }
                }
                _ => {
                    panic!("Get must be on tuple")
                }
            }
        }

    fn manage_operation(&mut self, operation_context: Rc<OperationContextAll>, id_context: String){
        if let Some(get_context) = operation_context.get_function() {
            let index = get_context.NUMBER().unwrap().get_text().parse::<usize>().unwrap();
            if let Some(tuple_context) = get_context.tuple() {
                self.function_on_tuple(tuple_context, GET_FUNCTION, Some(index), id_context);
            }
        }else{
            if let Some(len_context) = operation_context.len_function(){
                if let Some(tuple_context) = len_context.tuple() {
                    self.function_on_tuple(tuple_context, LEN_FUNCTION, None, id_context);
                }
            }else if let Some(_) = operation_context.PLUS() {
                let right_expr = &self.manage_right_expr(operation_context.right_expr().unwrap());
                let left_expr = &self.manage_left_expr(operation_context);
                self.symbol_table.insert(id_context , Value::Number(left_expr + right_expr));
            }else if let Some(_) = operation_context.MINUS() {
                let right_expr = &self.manage_right_expr(operation_context.right_expr().unwrap());
                let left_expr = &self.manage_left_expr(operation_context);
                self.symbol_table.insert(id_context , Value::Number(left_expr - right_expr));
            }else if let Some(_) = operation_context.KLEENE() {
                let right_expr = &self.manage_right_expr(operation_context.right_expr().unwrap());
                let left_expr = &self.manage_left_expr(operation_context);
                self.symbol_table.insert(id_context , Value::Number(left_expr * right_expr));
            }else if let Some(_) = operation_context.SLASH() {
                let right_expr = &self.manage_right_expr(operation_context.right_expr().unwrap());
                let left_expr = &self.manage_left_expr(operation_context);
                self.symbol_table.insert(id_context , Value::Number(left_expr / right_expr));
            }
        }
    }

    fn manage_right_expr(&self, right_context: Rc<Right_exprContextAll>) -> i64 {
        if let Some(id_context) = right_context.ID() {
            let val =  &self.manage_var_expr_content(id_context);
            return val.clone();
        }else if let Some(id_context) = right_context.NUMBER() {
            return id_context.get_text().parse::<i64>().unwrap();
        }
        panic!("Right Expression invalid")
    }

    fn manage_left_expr(&self, operation_context: Rc<OperationContextAll>) -> i64 {
        if let Some(id_context) = operation_context.ID() {
            let val =  &self.manage_var_expr_content(id_context);
            return val.clone();
        }else if let Some(id_context) = operation_context.NUMBER() {
            return id_context.get_text().parse::<i64>().unwrap();
        }
        panic!("Right Expression invalid")
    }

    fn manage_var_expr_content(&self, id_context: Rc<TerminalNode<lifParserContextType>>) -> i64 {
        let id_variable = id_context.get_text();
        match &self.symbol_table.get(&id_variable) {
            None => {
                panic!("Variable {} not found", &id_variable)
            }
            Some(variable) => match variable {
                Value::Number(number) => {
                    return number.clone();
                }
                Value::Tuple(_) | Value::String(_) | Value::Char(_) => {
                    panic!("Value not available for an attribute")
                }
                Value::ID(id_value)  => {
                    return id_value.get_value().parse::<i64>().unwrap()
                }
            },
        }
    }
}

impl ParseTreeListener<'_, lifParserContextType> for Listener {}

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
                                attribute_list +=
                                    &*(self.validate_attribute(attribute) + &*" ".to_string());
                            }
                            println!(
                                "{}",
                                server.send_message(
                                    String::from(CREATE)
                                        + SPACE
                                        + &*self.validate_attribute(attribute)
                                        + SPACE
                                        + &*self.validate_tuple_name(tuple_space_name)
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
                                        + &*self.validate_attribute(attribute)
                                        + SPACE
                                        + &*self.validate_tuple_name(tuple_space_name)
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
                                    + &*self.validate_attribute(attribute)
                                    + SPACE
                                    + &*self.validate_tuple_name(tuple_space_name)
                            )
                        );
                    } else {
                        println!(
                            "{}",
                            server.send_message(
                                String::from(DELETE)
                                    + SPACE
                                    + &*self.validate_tuple_name(tuple_space_name)
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
                            attribute_list +=
                                &*(self.validate_attribute(attribute) + &*" ".to_string());
                        }
                        println!(
                            "{}",
                            server.send_message(
                                String::from(ATTACH)
                                    + SPACE
                                    + &*self.validate_tuple_name(tuple_space_name)
                                    + SPACE
                                    + &*attribute_list
                            )
                        );
                    } else {
                        println!(
                            "{}",
                            server.send_message(
                                String::from(ATTACH)
                                    + SPACE
                                    + &*self.validate_tuple_name(tuple_space_name)
                            )
                        );
                    }
                }
            }
        }
    }

    fn enter_out(&mut self, _ctx: &OutContext<'_>) {
        self.manage_tuple_operation(TupleOperationContext::OutContext(_ctx), OUT);
    }

    fn enter_assignation(&mut self, _ctx: &AssignationContext<'_>) {
        if let Some(id_context) = _ctx.ID() {
            if let Some(init_var_context) = _ctx.init_var() {
                let value: Value = self.enter_init_var(init_var_context);
                let _ = &self.symbol_table.insert(id_context.get_text(), value);
            } else {
                let mut response: String = String::new();
                if let Some(read_context) = _ctx.read() {
                    response = self.manage_tuple_operation(
                        TupleOperationContext::ReadContext(&read_context),
                        READ,
                    );
                } else {
                    if let Some(in_context) = _ctx.in_instr() {
                        response = self.manage_tuple_operation(
                            TupleOperationContext::InContext(&in_context),
                            IN,
                        );
                    } else {
                        if let Some(operation_context)= _ctx.operation(){
                            self.manage_operation(operation_context, id_context.get_text());
                        }
                    }
                }
                if !response.contains("ERROR") && !response.is_empty() {
                    let value = self.parse_tuple(response);
                    let _ = self
                        .symbol_table
                        .insert(id_context.get_text(),value);
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
