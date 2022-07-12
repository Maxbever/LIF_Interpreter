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
use crate::lifparser::{
    AssignationContext, AssignationContextAttrs, AttachContext, AttachContextAttrs,
    AttributContextAll, AttributContextAttrs, Basic_boolean_operationContextAll,
    Basic_boolean_operationContextAttrs, Boolean_operationContextAll,
    Boolean_operationContextAttrs, ConnectContext, ConnectContextAttrs, CreateContext,
    CreateContextAttrs, DeleteContext, DeleteContextAttrs, Encryption_keyContextAll,
    Encryption_keyContextAttrs, For_instrContext, For_instrContextAttrs, Get_functionContextAttrs,
    In_instrContext, In_instrContextAttrs, Init_varContextAll, Init_varContextAttrs,
    InstructionContext, InstructionContextAttrs, Len_functionContextAttrs, OperationContextAll,
    OperationContextAttrs, OutContext, OutContextAttrs, ReadContext, ReadContextAttrs,
    Right_exprContextAll, Right_exprContextAttrs, RootContextAttrs, Server_nameContextAll,
    Server_nameContextAttrs, TupleContextAll, TupleContextAttrs, Tuple_contentContextAttrs,
    Tuple_space_nameContextAll, Tuple_space_nameContextAttrs, While_instrContext,
    While_instrContextAll, While_instrContextAttrs,
};
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::{ParseTree, ParseTreeListener, TerminalNode};
use antlr_rust::InputStream;
use server::Server;
use std::collections::HashMap;
use std::rc::Rc;
use std::{env, fs};

mod constant;
mod generated_file_antlr;
mod server;

struct Listener {
    server_list: HashMap<String, Server>,
    symbol_table: HashMap<String, Value>,
    server_attached: String,
}

enum Value {
    String(String),
    Number(f32),
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
            server_attached: String::new(),
        }
    }

    fn manage_tuple_operation(
        &mut self,
        _ctx: TupleOperationContext,
        operation: &str,
        server_name: &String,
    ) -> String {
        match _ctx {
            TupleOperationContext::OutContext(context) => {
                if let Some(_) = context.tuple(0) {
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
                            let _ = &self.server_list.insert(server_name.clone(), server);
                            return response;
                        }
                    }
                }
            }
            TupleOperationContext::InContext(context) => {
                if let Some(_) = context.tuple(0) {
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
                            println!("In result: {}", response);
                            let _ = &self.server_list.insert(server_name.clone(), server);
                            return response;
                        }
                    }
                }
            }
            TupleOperationContext::ReadContext(context) => {
                if let Some(_) = context.tuple(0) {
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
                            let _ = &self.server_list.insert(server_name.clone(), server);
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
            return Value::Number(number_context.get_text().parse::<f32>().unwrap());
        } else if let Some(tuple_context) = _ctx.tuple() {
            return if let Some(id_context) = tuple_context.ID() {
                Value::Tuple(vec![self.add_variable_in_variable(id_context)])
            } else {
                let mut tuple_list: Vec<Value> = Vec::new();
                for tuple in tuple_context.tuple_content_all() {
                    let tuple_value: Value;
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

    fn validate_encryption_key(&self, key: Rc<Encryption_keyContextAll>) -> String {
        let mut res = (if let Some(key) = key.ID() {
            self.validate_variable(key)
        } else {
            key.get_text()
        });
        res.pop();
        res.remove(0);
        return res;
    }

    fn validate_tuple_name(&self, tuple_space_ctx: Rc<Tuple_space_nameContextAll>) -> String {
        return if let Some(id_attribute) = tuple_space_ctx.ID() {
            self.validate_variable(id_attribute)
        } else {
            tuple_space_ctx.get_text()
        };
    }

    fn validate_server_name(&self, server_name_ctx: Rc<Server_nameContextAll>) -> String {
        return if let Some(id_attribute) = server_name_ctx.ID() {
            self.validate_variable(id_attribute)
        } else {
            server_name_ctx.get_text()
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

    fn function_on_tuple(
        &mut self,
        tuple_context: Rc<TupleContextAll>,
        function: &str,
        index: Option<usize>,
    ) -> Value {
        let value;
        if let Some(variable) = tuple_context.ID() {
            value = self.symbol_table.remove(&*variable.get_text()).unwrap();
            self.symbol_table.insert(variable.get_text(), value.clone());
        } else {
            value = self.parse_tuple(tuple_context.get_text());
        }
        match value {
            Value::Tuple(tuple_value) => {
                let mut vec_temp = tuple_value.clone();
                return if function == LEN_FUNCTION {
                    Value::Number(vec_temp.len() as f32)
                } else {
                    vec_temp.remove(index.unwrap())
                };
            }
            _ => {
                panic!("Get or len must be on tuple")
            }
        }
    }

    fn manage_operation(&mut self, operation_context: Rc<OperationContextAll>) -> Value {
        if let Some(get_context) = operation_context.get_function() {
            if let Some(right_exp) = get_context.right_expr() {
                let index = self.manage_right_expr(right_exp);
                if let Some(tuple_context) = get_context.tuple() {
                    return self.function_on_tuple(
                        tuple_context,
                        GET_FUNCTION,
                        Some(index as usize),
                    );
                }
            }
        } else {
            if let Some(len_context) = operation_context.len_function() {
                if let Some(tuple_context) = len_context.tuple() {
                    return self.function_on_tuple(tuple_context, LEN_FUNCTION, None);
                }
            } else if let Some(_) = operation_context.PLUS() {
                let right_expr = self.manage_operation(operation_context.operation(1).unwrap());
                let left_expr = self.manage_operation(operation_context.operation(0).unwrap());
                return Value::Number(
                    (Listener::check_value_number(left_expr)
                        + Listener::check_value_number(right_expr)) as f32,
                );
            } else if let Some(_) = operation_context.MINUS() {
                let right_expr = self.manage_operation(operation_context.operation(1).unwrap());
                let left_expr = self.manage_operation(operation_context.operation(0).unwrap());
                return Value::Number(
                    Listener::check_value_number(left_expr) as f32
                        - Listener::check_value_number(right_expr) as f32,
                );
            } else if let Some(_) = operation_context.KLEENE() {
                let right_expr = self.manage_operation(operation_context.operation(1).unwrap());
                let left_expr = self.manage_operation(operation_context.operation(0).unwrap());
                return Value::Number(
                    (Listener::check_value_number(left_expr)
                        * Listener::check_value_number(right_expr)) as f32,
                );
            } else if let Some(_) = operation_context.SLASH() {
                let right_expr = self.manage_operation(operation_context.operation(1).unwrap());
                let left_expr = self.manage_operation(operation_context.operation(0).unwrap());
                return Value::Number(
                    Listener::check_value_number(left_expr) as f32
                        / Listener::check_value_number(right_expr) as f32,
                );
            } else if let Some(right_expr) = operation_context.right_expr() {
                return Value::Number(*&self.manage_right_expr(right_expr));
            }
        }
        panic!("Operation not succeed")
    }

    fn manage_right_expr(&self, right_context: Rc<Right_exprContextAll>) -> f32 {
        if let Some(id_context) = right_context.ID() {
            let val = &self.manage_var_expr_content(id_context);
            return val.clone();
        } else if let Some(id_context) = right_context.NUMBER() {
            return id_context.get_text().parse::<f32>().unwrap();
        }
        panic!("Right Expression invalid")
    }

    fn manage_var_expr_content(&self, id_context: Rc<TerminalNode<lifParserContextType>>) -> f32 {
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
                    panic!("Value not available for an number operation")
                }
                Value::ID(id_value) => return id_value.get_value().parse::<f32>().unwrap(),
            },
        }
    }

    fn check_value_number(value: Value) -> i32 {
        match value {
            Value::Number(number) => return number as i32,
            Value::ID(value) => {
                return Listener::check_value_number(*value);
            }
            Value::String(_) | Value::Tuple(_) | Value::Char(_) => {
                panic!("Value must be a number")
            }
        }
    }

    fn validate_boolean_operation(&self, boolean_expr: &Rc<Boolean_operationContextAll>) -> bool {
        return if let Some(_) = boolean_expr.AND() {
            self.validate_boolean_basic_operation(boolean_expr.basic_boolean_operation(0).unwrap())
                && self.validate_boolean_basic_operation(
                    boolean_expr.basic_boolean_operation(1).unwrap(),
                )
        } else if let Some(_) = boolean_expr.OR() {
            self.validate_boolean_basic_operation(boolean_expr.basic_boolean_operation(0).unwrap())
                || self.validate_boolean_basic_operation(
                    boolean_expr.basic_boolean_operation(1).unwrap(),
                )
        } else {
            self.validate_boolean_basic_operation(boolean_expr.basic_boolean_operation(0).unwrap())
        };
    }

    fn validate_boolean_basic_operation(
        &self,
        boolean_expr: Rc<Basic_boolean_operationContextAll>,
    ) -> bool {
        return if let Some(_) = boolean_expr.RCHEVRON() {
            self.manage_right_expr(boolean_expr.right_expr(0).unwrap())
                >= self.manage_right_expr(boolean_expr.right_expr(1).unwrap())
        } else if let Some(_) = boolean_expr.LCHEVRON() {
            self.manage_right_expr(boolean_expr.right_expr(0).unwrap())
                <= self.manage_right_expr(boolean_expr.right_expr(1).unwrap())
        } else if let Some(_) = boolean_expr.EXCLAMATION() {
            self.manage_right_expr(boolean_expr.right_expr(0).unwrap())
                != self.manage_right_expr(boolean_expr.right_expr(1).unwrap())
        } else {
            self.manage_right_expr(boolean_expr.right_expr(0).unwrap())
                == self.manage_right_expr(boolean_expr.right_expr(1).unwrap())
        };
    }
}

impl ParseTreeListener<'_, lifParserContextType> for Listener {}

impl lifListener<'_> for Listener {
    fn enter_instruction(&mut self, _ctx: &InstructionContext<'_>) {
        if let Some(connect_ctx) = _ctx.connect() {
            self.enter_connect(&connect_ctx)
        } else if let Some(connect_ctx) = _ctx.connect() {
            self.enter_connect(&connect_ctx)
        } else if let Some(create_ctx) = _ctx.create() {
            self.enter_create(&create_ctx)
        } else if let Some(delete_ctx) = _ctx.delete() {
            self.enter_delete(&delete_ctx)
        } else if let Some(attach_ctx) = _ctx.attach() {
            self.enter_attach(&attach_ctx)
        } else if let Some(out_ctx) = _ctx.out() {
            self.enter_out(&out_ctx)
        } else if let Some(for_context) = _ctx.for_instr() {
            self.enter_for_instr(&for_context)
        } else if let Some(assignation_ctx) = _ctx.assignation() {
            self.enter_assignation(&assignation_ctx)
        } else if let Some(while_ctx) = _ctx.while_instr() {
            self.enter_while_instr(&while_ctx)
        }
    }

    fn enter_while_instr(&mut self, _ctx: &While_instrContextAll<'_>) {
        if let Some(boolean_expr) = _ctx.boolean_operation() {
            while self.validate_boolean_operation(&boolean_expr) {
                for instr in _ctx.instruction_all() {
                    self.enter_instruction(&instr);
                }
            }
        }
    }

    fn enter_connect(&mut self, _ctx: &ConnectContext<'_>) {
        if let Some(server_name) = _ctx.server_name() {
            if let Some(protocol) = _ctx.protocol() {
                if let Some(ip_address) = _ctx.ip_address() {
                    if let Some(port) = _ctx.port() {
                        if let Some(key) = _ctx.encryption_key() {
                            let server_name = self.validate_server_name(server_name);
                            let server = Server::new(
                                ip_address.get_text(),
                                port.get_text(),
                                protocol.get_text(),
                                server_name.clone(),
                                self.validate_encryption_key(key),
                            );
                            self.server_list.insert(server_name, server);
                        }
                    }
                }
            }
        }
    }

    fn enter_create(&mut self, _ctx: &CreateContext<'_>) {
        if let Some(server_name) = _ctx.server_name() {
            if let Some(attribute) = _ctx.attribut(0) {
                if let Some(tuple_space_name) = _ctx.tuple_space_name() {
                    let server_name = self.validate_server_name(server_name);
                    let server = self.server_list.get(&*server_name);
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
    }

    fn enter_delete(&mut self, _ctx: &DeleteContext<'_>) {
        if let Some(server_name) = _ctx.server_name() {
            if let Some(tuple_space_name) = _ctx.tuple_space_name() {
                let server_name = self.validate_server_name(server_name);
                let server = self.server_list.get(&*server_name);
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
    }

    fn enter_attach(&mut self, _ctx: &AttachContext<'_>) {
        if let Some(server_name) = _ctx.server_name() {
            self.server_attached = self.validate_server_name(server_name);
            if let Some(tuple_space_name) = _ctx.tuple_space_name() {
                let server = self.server_list.get(&*self.server_attached);
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
    }

    fn enter_out(&mut self, _ctx: &OutContext<'_>) {
        self.manage_tuple_operation(
            TupleOperationContext::OutContext(_ctx),
            OUT,
            &self.server_attached.clone(),
        );
    }

    fn enter_for_instr(&mut self, _ctx: &For_instrContext<'_>) {
        let iterator_value = self.manage_operation(_ctx.operation(0).unwrap());
        let max = Listener::check_value_number(self.manage_operation(_ctx.operation(1).unwrap()));
        self.symbol_table
            .insert(_ctx.ID().unwrap().get_text(), iterator_value.clone());

        let mut iterator = Listener::check_value_number(iterator_value);

        while iterator < max {
            for instr in _ctx.instruction_all() {
                self.enter_instruction(&instr);
            }
            iterator += 1;
            self.symbol_table.insert(
                _ctx.ID().unwrap().get_text(),
                Value::Number(iterator as f32),
            );
        }
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
                        &self.server_attached.clone(),
                    );
                } else {
                    if let Some(in_context) = _ctx.in_instr() {
                        response = self.manage_tuple_operation(
                            TupleOperationContext::InContext(&in_context),
                            IN,
                            &self.server_attached.clone(),
                        );
                    } else {
                        if let Some(operation_context) = _ctx.operation() {
                            let value = self.manage_operation(operation_context);
                            self.symbol_table.insert(id_context.get_text(), value);
                        }
                    }
                }
                if !response.contains("ERROR") && !response.is_empty() {
                    let value = self.parse_tuple(response);
                    let _ = self.symbol_table.insert(id_context.get_text(), value);
                }
                // else {
                //     let value = self.parse_tuple(String::from("()"));
                //     let _ = self.symbol_table.insert(id_context.get_text(), value);
                // }
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
            let mut listener = Listener::new();
            for instr in parser.root().unwrap().instruction_all() {
                listener.enter_instruction(&instr);
            }
        }
        Err(error) => {
            println!("{}", error)
        }
    };
}
