#![allow(nonstandard_style)]
// Generated from lif.g4 by ANTLR 4.8
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
 * Enter a parse tree produced by {@link lifParser#tupleSpaceName}.
 * @param ctx the parse tree
 */
fn enter_tupleSpaceName(&mut self, _ctx: &TupleSpaceNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link lifParser#tupleSpaceName}.
 * @param ctx the parse tree
 */
fn exit_tupleSpaceName(&mut self, _ctx: &TupleSpaceNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link lifParser#initvar}.
 * @param ctx the parse tree
 */
fn enter_initvar(&mut self, _ctx: &InitvarContext<'input>) { }
/**
 * Exit a parse tree produced by {@link lifParser#initvar}.
 * @param ctx the parse tree
 */
fn exit_initvar(&mut self, _ctx: &InitvarContext<'input>) { }

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
 * Enter a parse tree produced by {@link lifParser#ipAddress}.
 * @param ctx the parse tree
 */
fn enter_ipAddress(&mut self, _ctx: &IpAddressContext<'input>) { }
/**
 * Exit a parse tree produced by {@link lifParser#ipAddress}.
 * @param ctx the parse tree
 */
fn exit_ipAddress(&mut self, _ctx: &IpAddressContext<'input>) { }

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
