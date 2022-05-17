#![allow(nonstandard_style)]
// Generated from .\lif.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::lifparser::*;

pub trait lifListener<'input> : ParseTreeListener<'input,lifParserContextType>{

/**
 * Enter a parse tree produced by {@link lifParser#root}.
 * @param ctx the parse tree
 */
fn enter_root(&mut self, _ctx: &RootContext<'input>) { }
/**
 * Exit a parse tree produced by {@link lifParser#root}.
 * @param ctx the parse tree
 */
fn exit_root(&mut self, _ctx: &RootContext<'input>) { }

/**
 * Enter a parse tree produced by {@link lifParser#instruction}.
 * @param ctx the parse tree
 */
fn enter_instruction(&mut self, _ctx: &InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link lifParser#instruction}.
 * @param ctx the parse tree
 */
fn exit_instruction(&mut self, _ctx: &InstructionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link lifParser#connect}.
 * @param ctx the parse tree
 */
fn enter_connect(&mut self, _ctx: &ConnectContext<'input>) { }
/**
 * Exit a parse tree produced by {@link lifParser#connect}.
 * @param ctx the parse tree
 */
fn exit_connect(&mut self, _ctx: &ConnectContext<'input>) { }

/**
 * Enter a parse tree produced by {@link lifParser#create}.
 * @param ctx the parse tree
 */
fn enter_create(&mut self, _ctx: &CreateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link lifParser#create}.
 * @param ctx the parse tree
 */
fn exit_create(&mut self, _ctx: &CreateContext<'input>) { }

/**
 * Enter a parse tree produced by {@link lifParser#delete}.
 * @param ctx the parse tree
 */
fn enter_delete(&mut self, _ctx: &DeleteContext<'input>) { }
/**
 * Exit a parse tree produced by {@link lifParser#delete}.
 * @param ctx the parse tree
 */
fn exit_delete(&mut self, _ctx: &DeleteContext<'input>) { }

/**
 * Enter a parse tree produced by {@link lifParser#attach}.
 * @param ctx the parse tree
 */
fn enter_attach(&mut self, _ctx: &AttachContext<'input>) { }
/**
 * Exit a parse tree produced by {@link lifParser#attach}.
 * @param ctx the parse tree
 */
fn exit_attach(&mut self, _ctx: &AttachContext<'input>) { }

/**
 * Enter a parse tree produced by {@link lifParser#out}.
 * @param ctx the parse tree
 */
fn enter_out(&mut self, _ctx: &OutContext<'input>) { }
/**
 * Exit a parse tree produced by {@link lifParser#out}.
 * @param ctx the parse tree
 */
fn exit_out(&mut self, _ctx: &OutContext<'input>) { }

/**
 * Enter a parse tree produced by {@link lifParser#for_instr}.
 * @param ctx the parse tree
 */
fn enter_for_instr(&mut self, _ctx: &For_instrContext<'input>) { }
/**
 * Exit a parse tree produced by {@link lifParser#for_instr}.
 * @param ctx the parse tree
 */
fn exit_for_instr(&mut self, _ctx: &For_instrContext<'input>) { }

/**
 * Enter a parse tree produced by {@link lifParser#operation}.
 * @param ctx the parse tree
 */
fn enter_operation(&mut self, _ctx: &OperationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link lifParser#operation}.
 * @param ctx the parse tree
 */
fn exit_operation(&mut self, _ctx: &OperationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link lifParser#get_function}.
 * @param ctx the parse tree
 */
fn enter_get_function(&mut self, _ctx: &Get_functionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link lifParser#get_function}.
 * @param ctx the parse tree
 */
fn exit_get_function(&mut self, _ctx: &Get_functionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link lifParser#len_function}.
 * @param ctx the parse tree
 */
fn enter_len_function(&mut self, _ctx: &Len_functionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link lifParser#len_function}.
 * @param ctx the parse tree
 */
fn exit_len_function(&mut self, _ctx: &Len_functionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link lifParser#right_expr}.
 * @param ctx the parse tree
 */
fn enter_right_expr(&mut self, _ctx: &Right_exprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link lifParser#right_expr}.
 * @param ctx the parse tree
 */
fn exit_right_expr(&mut self, _ctx: &Right_exprContext<'input>) { }

/**
 * Enter a parse tree produced by {@link lifParser#assignation}.
 * @param ctx the parse tree
 */
fn enter_assignation(&mut self, _ctx: &AssignationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link lifParser#assignation}.
 * @param ctx the parse tree
 */
fn exit_assignation(&mut self, _ctx: &AssignationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link lifParser#read}.
 * @param ctx the parse tree
 */
fn enter_read(&mut self, _ctx: &ReadContext<'input>) { }
/**
 * Exit a parse tree produced by {@link lifParser#read}.
 * @param ctx the parse tree
 */
fn exit_read(&mut self, _ctx: &ReadContext<'input>) { }

/**
 * Enter a parse tree produced by {@link lifParser#in_instr}.
 * @param ctx the parse tree
 */
fn enter_in_instr(&mut self, _ctx: &In_instrContext<'input>) { }
/**
 * Exit a parse tree produced by {@link lifParser#in_instr}.
 * @param ctx the parse tree
 */
fn exit_in_instr(&mut self, _ctx: &In_instrContext<'input>) { }

/**
 * Enter a parse tree produced by {@link lifParser#attribut}.
 * @param ctx the parse tree
 */
fn enter_attribut(&mut self, _ctx: &AttributContext<'input>) { }
/**
 * Exit a parse tree produced by {@link lifParser#attribut}.
 * @param ctx the parse tree
 */
fn exit_attribut(&mut self, _ctx: &AttributContext<'input>) { }

/**
 * Enter a parse tree produced by {@link lifParser#tuple}.
 * @param ctx the parse tree
 */
fn enter_tuple(&mut self, _ctx: &TupleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link lifParser#tuple}.
 * @param ctx the parse tree
 */
fn exit_tuple(&mut self, _ctx: &TupleContext<'input>) { }

/**
 * Enter a parse tree produced by {@link lifParser#tuple_content}.
 * @param ctx the parse tree
 */
fn enter_tuple_content(&mut self, _ctx: &Tuple_contentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link lifParser#tuple_content}.
 * @param ctx the parse tree
 */
fn exit_tuple_content(&mut self, _ctx: &Tuple_contentContext<'input>) { }

/**
 * Enter a parse tree produced by {@link lifParser#tuple_space_name}.
 * @param ctx the parse tree
 */
fn enter_tuple_space_name(&mut self, _ctx: &Tuple_space_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link lifParser#tuple_space_name}.
 * @param ctx the parse tree
 */
fn exit_tuple_space_name(&mut self, _ctx: &Tuple_space_nameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link lifParser#server_name}.
 * @param ctx the parse tree
 */
fn enter_server_name(&mut self, _ctx: &Server_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link lifParser#server_name}.
 * @param ctx the parse tree
 */
fn exit_server_name(&mut self, _ctx: &Server_nameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link lifParser#init_var}.
 * @param ctx the parse tree
 */
fn enter_init_var(&mut self, _ctx: &Init_varContext<'input>) { }
/**
 * Exit a parse tree produced by {@link lifParser#init_var}.
 * @param ctx the parse tree
 */
fn exit_init_var(&mut self, _ctx: &Init_varContext<'input>) { }

/**
 * Enter a parse tree produced by {@link lifParser#protocol}.
 * @param ctx the parse tree
 */
fn enter_protocol(&mut self, _ctx: &ProtocolContext<'input>) { }
/**
 * Exit a parse tree produced by {@link lifParser#protocol}.
 * @param ctx the parse tree
 */
fn exit_protocol(&mut self, _ctx: &ProtocolContext<'input>) { }

/**
 * Enter a parse tree produced by {@link lifParser#ip_address}.
 * @param ctx the parse tree
 */
fn enter_ip_address(&mut self, _ctx: &Ip_addressContext<'input>) { }
/**
 * Exit a parse tree produced by {@link lifParser#ip_address}.
 * @param ctx the parse tree
 */
fn exit_ip_address(&mut self, _ctx: &Ip_addressContext<'input>) { }

/**
 * Enter a parse tree produced by {@link lifParser#port}.
 * @param ctx the parse tree
 */
fn enter_port(&mut self, _ctx: &PortContext<'input>) { }
/**
 * Exit a parse tree produced by {@link lifParser#port}.
 * @param ctx the parse tree
 */
fn exit_port(&mut self, _ctx: &PortContext<'input>) { }

}
