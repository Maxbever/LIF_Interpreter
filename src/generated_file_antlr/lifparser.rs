// Generated from lif.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::liflistener::*;
use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const CONNECT:isize=1; 
		pub const ATTACH:isize=2; 
		pub const CREATE:isize=3; 
		pub const DELETE:isize=4; 
		pub const OUT:isize=5; 
		pub const READ:isize=6; 
		pub const IN:isize=7; 
		pub const TCP:isize=8; 
		pub const UDP:isize=9; 
		pub const LPAR:isize=10; 
		pub const RPAR:isize=11; 
		pub const COMMA:isize=12; 
		pub const DOUBLEQUOTE:isize=13; 
		pub const QUOTE:isize=14; 
		pub const SLASH:isize=15; 
		pub const BACKSLASH:isize=16; 
		pub const LBRACKET:isize=17; 
		pub const RBRACKET:isize=18; 
		pub const DOT:isize=19; 
		pub const DOUBLEDOT:isize=20; 
		pub const SEMICOLON:isize=21; 
		pub const KLEENE:isize=22; 
		pub const WILDCARD:isize=23; 
		pub const ID:isize=24; 
		pub const NUMBER:isize=25; 
		pub const STRING:isize=26; 
		pub const CHARACTER:isize=27; 
		pub const LINECOMMENT:isize=28; 
		pub const COMMENT:isize=29; 
		pub const NEWLINE:isize=30; 
		pub const WS:isize=31;
	pub const RULE_root:usize = 0; 
	pub const RULE_instruction:usize = 1; 
	pub const RULE_attribut:usize = 2; 
	pub const RULE_tuple:usize = 3; 
	pub const RULE_tuple_content:usize = 4; 
	pub const RULE_tupleSpaceName:usize = 5; 
	pub const RULE_initvar:usize = 6; 
	pub const RULE_protocol:usize = 7; 
	pub const RULE_ipAddress:usize = 8; 
	pub const RULE_port:usize = 9;
	pub const ruleNames: [&'static str; 10] =  [
		"root", "instruction", "attribut", "tuple", "tuple_content", "tupleSpaceName", 
		"initvar", "protocol", "ipAddress", "port"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;24] = [
		None, Some("'connect'"), Some("'attach'"), Some("'create'"), Some("'delete'"), 
		Some("'out'"), Some("'read'"), Some("'in'"), Some("'tcp'"), Some("'udp'"), 
		Some("'('"), Some("')'"), Some("','"), Some("'\"'"), Some("'''"), Some("'/'"), 
		Some("'\\'"), Some("'['"), Some("']'"), Some("'.'"), Some("':'"), Some("';'"), 
		Some("'*'"), Some("'_'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;32]  = [
		None, Some("CONNECT"), Some("ATTACH"), Some("CREATE"), Some("DELETE"), 
		Some("OUT"), Some("READ"), Some("IN"), Some("TCP"), Some("UDP"), Some("LPAR"), 
		Some("RPAR"), Some("COMMA"), Some("DOUBLEQUOTE"), Some("QUOTE"), Some("SLASH"), 
		Some("BACKSLASH"), Some("LBRACKET"), Some("RBRACKET"), Some("DOT"), Some("DOUBLEDOT"), 
		Some("SEMICOLON"), Some("KLEENE"), Some("WILDCARD"), Some("ID"), Some("NUMBER"), 
		Some("STRING"), Some("CHARACTER"), Some("LINECOMMENT"), Some("COMMENT"), 
		Some("NEWLINE"), Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,lifParserExt, I, lifParserContextType , dyn lifListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type lifTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, lifParserContextType , dyn lifListener<'input> + 'a>;

/// Parser for lif grammar
pub struct lifParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","2");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				lifParserExt{
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> lifParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> lifParser<'input, I, DefaultErrorStrategy<'input,lifParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for lifParser
pub trait lifParserContext<'input>:
	for<'x> Listenable<dyn lifListener<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=lifParserContextType>
{}

impl<'input> lifParserContext<'input> for TerminalNode<'input,lifParserContextType> {}
impl<'input> lifParserContext<'input> for ErrorNode<'input,lifParserContextType> {}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn lifParserContext<'input> + 'input{}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn lifListener<'input> + 'input{}

pub struct lifParserContextType;
antlr_rust::type_id!{lifParserContextType}

impl<'input> ParserNodeType<'input> for lifParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn lifParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct lifParserExt{
}

impl lifParserExt{
}


impl<'input> TokenAware<'input> for lifParserExt{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for lifParserExt{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for lifParserExt{
	fn get_grammar_file_name(&self) -> & str{ "lif.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- root ----------------
pub type RootContextAll<'input> = RootContext<'input>;


pub type RootContext<'input> = BaseParserRuleContext<'input,RootContextExt<'input>>;

#[derive(Clone)]
pub struct RootContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> lifParserContext<'input> for RootContext<'input>{}

impl<'input,'a> Listenable<dyn lifListener<'input> + 'a> for RootContext<'input>{
	fn enter(&self,listener: &mut (dyn lifListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_root(self);
	}
}

impl<'input> CustomRuleContext<'input> for RootContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = lifParserContextType;
	fn get_rule_index(&self) -> usize { RULE_root }
	//fn type_rule_index() -> usize where Self: Sized { RULE_root }
}
antlr_rust::type_id!{RootContextExt<'a>}

impl<'input> RootContextExt<'input>{
	fn new(parent: Option<Rc<dyn lifParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RootContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RootContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RootContextAttrs<'input>: lifParserContext<'input> + BorrowMut<RootContextExt<'input>>{

fn instruction_all(&self) ->  Vec<Rc<InstructionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn instruction(&self, i: usize) -> Option<Rc<InstructionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> RootContextAttrs<'input> for RootContext<'input>{}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn root(&mut self,)
	-> Result<Rc<RootContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RootContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_root);
        let mut _localctx: Rc<RootContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(23);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << CONNECT) | (1usize << ATTACH) | (1usize << CREATE) | (1usize << DELETE) | (1usize << OUT) | (1usize << READ) | (1usize << IN))) != 0) {
				{
				{
				/*InvokeRule instruction*/
				recog.base.set_state(20);
				recog.instruction()?;

				}
				}
				recog.base.set_state(25);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- instruction ----------------
pub type InstructionContextAll<'input> = InstructionContext<'input>;


pub type InstructionContext<'input> = BaseParserRuleContext<'input,InstructionContextExt<'input>>;

#[derive(Clone)]
pub struct InstructionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> lifParserContext<'input> for InstructionContext<'input>{}

impl<'input,'a> Listenable<dyn lifListener<'input> + 'a> for InstructionContext<'input>{
	fn enter(&self,listener: &mut (dyn lifListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_instruction(self);
	}
}

impl<'input> CustomRuleContext<'input> for InstructionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = lifParserContextType;
	fn get_rule_index(&self) -> usize { RULE_instruction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_instruction }
}
antlr_rust::type_id!{InstructionContextExt<'a>}

impl<'input> InstructionContextExt<'input>{
	fn new(parent: Option<Rc<dyn lifParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InstructionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InstructionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InstructionContextAttrs<'input>: lifParserContext<'input> + BorrowMut<InstructionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CONNECT
/// Returns `None` if there is no child corresponding to token CONNECT
fn CONNECT(&self) -> Option<Rc<TerminalNode<'input,lifParserContextType>>> where Self:Sized{
	self.get_token(CONNECT, 0)
}
fn protocol(&self) -> Option<Rc<ProtocolContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token DOUBLEDOT in current rule
fn DOUBLEDOT_all(&self) -> Vec<Rc<TerminalNode<'input,lifParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DOUBLEDOT, starting from 0.
/// Returns `None` if number of children corresponding to token DOUBLEDOT is less or equal than `i`.
fn DOUBLEDOT(&self, i: usize) -> Option<Rc<TerminalNode<'input,lifParserContextType>>> where Self:Sized{
	self.get_token(DOUBLEDOT, i)
}
fn ipAddress(&self) -> Option<Rc<IpAddressContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn port(&self) -> Option<Rc<PortContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CREATE
/// Returns `None` if there is no child corresponding to token CREATE
fn CREATE(&self) -> Option<Rc<TerminalNode<'input,lifParserContextType>>> where Self:Sized{
	self.get_token(CREATE, 0)
}
fn attribut_all(&self) ->  Vec<Rc<AttributContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn attribut(&self, i: usize) -> Option<Rc<AttributContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn tupleSpaceName(&self) -> Option<Rc<TupleSpaceNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DELETE
/// Returns `None` if there is no child corresponding to token DELETE
fn DELETE(&self) -> Option<Rc<TerminalNode<'input,lifParserContextType>>> where Self:Sized{
	self.get_token(DELETE, 0)
}
/// Retrieves first TerminalNode corresponding to token ATTACH
/// Returns `None` if there is no child corresponding to token ATTACH
fn ATTACH(&self) -> Option<Rc<TerminalNode<'input,lifParserContextType>>> where Self:Sized{
	self.get_token(ATTACH, 0)
}
/// Retrieves first TerminalNode corresponding to token OUT
/// Returns `None` if there is no child corresponding to token OUT
fn OUT(&self) -> Option<Rc<TerminalNode<'input,lifParserContextType>>> where Self:Sized{
	self.get_token(OUT, 0)
}
fn tuple(&self) -> Option<Rc<TupleContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token READ
/// Returns `None` if there is no child corresponding to token READ
fn READ(&self) -> Option<Rc<TerminalNode<'input,lifParserContextType>>> where Self:Sized{
	self.get_token(READ, 0)
}
/// Retrieves first TerminalNode corresponding to token IN
/// Returns `None` if there is no child corresponding to token IN
fn IN(&self) -> Option<Rc<TerminalNode<'input,lifParserContextType>>> where Self:Sized{
	self.get_token(IN, 0)
}

}

impl<'input> InstructionContextAttrs<'input> for InstructionContext<'input>{}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn instruction(&mut self,)
	-> Result<Rc<InstructionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InstructionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_instruction);
        let mut _localctx: Rc<InstructionContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(60);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 CONNECT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(26);
					recog.base.match_token(CONNECT,&mut recog.err_handler)?;

					/*InvokeRule protocol*/
					recog.base.set_state(27);
					recog.protocol()?;

					recog.base.set_state(28);
					recog.base.match_token(DOUBLEDOT,&mut recog.err_handler)?;

					/*InvokeRule ipAddress*/
					recog.base.set_state(29);
					recog.ipAddress()?;

					recog.base.set_state(30);
					recog.base.match_token(DOUBLEDOT,&mut recog.err_handler)?;

					/*InvokeRule port*/
					recog.base.set_state(31);
					recog.port()?;

					}
				}

			 CREATE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(33);
					recog.base.match_token(CREATE,&mut recog.err_handler)?;

					/*InvokeRule attribut*/
					recog.base.set_state(34);
					recog.attribut()?;

					/*InvokeRule tupleSpaceName*/
					recog.base.set_state(35);
					recog.tupleSpaceName()?;

					recog.base.set_state(39);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << ID) | (1usize << NUMBER) | (1usize << STRING) | (1usize << CHARACTER))) != 0) {
						{
						{
						/*InvokeRule attribut*/
						recog.base.set_state(36);
						recog.attribut()?;

						}
						}
						recog.base.set_state(41);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}

			 DELETE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(42);
					recog.base.match_token(DELETE,&mut recog.err_handler)?;

					/*InvokeRule attribut*/
					recog.base.set_state(43);
					recog.attribut()?;

					/*InvokeRule tupleSpaceName*/
					recog.base.set_state(44);
					recog.tupleSpaceName()?;

					}
				}

			 ATTACH 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(46);
					recog.base.match_token(ATTACH,&mut recog.err_handler)?;

					/*InvokeRule tupleSpaceName*/
					recog.base.set_state(47);
					recog.tupleSpaceName()?;

					recog.base.set_state(51);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << ID) | (1usize << NUMBER) | (1usize << STRING) | (1usize << CHARACTER))) != 0) {
						{
						{
						/*InvokeRule attribut*/
						recog.base.set_state(48);
						recog.attribut()?;

						}
						}
						recog.base.set_state(53);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}

			 OUT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(54);
					recog.base.match_token(OUT,&mut recog.err_handler)?;

					/*InvokeRule tuple*/
					recog.base.set_state(55);
					recog.tuple()?;

					}
				}

			 READ 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					recog.base.set_state(56);
					recog.base.match_token(READ,&mut recog.err_handler)?;

					/*InvokeRule tuple*/
					recog.base.set_state(57);
					recog.tuple()?;

					}
				}

			 IN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					recog.base.set_state(58);
					recog.base.match_token(IN,&mut recog.err_handler)?;

					/*InvokeRule tuple*/
					recog.base.set_state(59);
					recog.tuple()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- attribut ----------------
pub type AttributContextAll<'input> = AttributContext<'input>;


pub type AttributContext<'input> = BaseParserRuleContext<'input,AttributContextExt<'input>>;

#[derive(Clone)]
pub struct AttributContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> lifParserContext<'input> for AttributContext<'input>{}

impl<'input,'a> Listenable<dyn lifListener<'input> + 'a> for AttributContext<'input>{
	fn enter(&self,listener: &mut (dyn lifListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_attribut(self);
	}
}

impl<'input> CustomRuleContext<'input> for AttributContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = lifParserContextType;
	fn get_rule_index(&self) -> usize { RULE_attribut }
	//fn type_rule_index() -> usize where Self: Sized { RULE_attribut }
}
antlr_rust::type_id!{AttributContextExt<'a>}

impl<'input> AttributContextExt<'input>{
	fn new(parent: Option<Rc<dyn lifParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AttributContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AttributContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AttributContextAttrs<'input>: lifParserContext<'input> + BorrowMut<AttributContextExt<'input>>{

fn initvar(&self) -> Option<Rc<InitvarContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AttributContextAttrs<'input> for AttributContext<'input>{}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn attribut(&mut self,)
	-> Result<Rc<AttributContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AttributContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_attribut);
        let mut _localctx: Rc<AttributContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule initvar*/
			recog.base.set_state(62);
			recog.initvar()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tuple ----------------
pub type TupleContextAll<'input> = TupleContext<'input>;


pub type TupleContext<'input> = BaseParserRuleContext<'input,TupleContextExt<'input>>;

#[derive(Clone)]
pub struct TupleContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> lifParserContext<'input> for TupleContext<'input>{}

impl<'input,'a> Listenable<dyn lifListener<'input> + 'a> for TupleContext<'input>{
	fn enter(&self,listener: &mut (dyn lifListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_tuple(self);
	}
}

impl<'input> CustomRuleContext<'input> for TupleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = lifParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tuple }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tuple }
}
antlr_rust::type_id!{TupleContextExt<'a>}

impl<'input> TupleContextExt<'input>{
	fn new(parent: Option<Rc<dyn lifParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TupleContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TupleContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TupleContextAttrs<'input>: lifParserContext<'input> + BorrowMut<TupleContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LPAR
/// Returns `None` if there is no child corresponding to token LPAR
fn LPAR(&self) -> Option<Rc<TerminalNode<'input,lifParserContextType>>> where Self:Sized{
	self.get_token(LPAR, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAR
/// Returns `None` if there is no child corresponding to token RPAR
fn RPAR(&self) -> Option<Rc<TerminalNode<'input,lifParserContextType>>> where Self:Sized{
	self.get_token(RPAR, 0)
}
fn tuple_content_all(&self) ->  Vec<Rc<Tuple_contentContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn tuple_content(&self, i: usize) -> Option<Rc<Tuple_contentContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,lifParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,lifParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> TupleContextAttrs<'input> for TupleContext<'input>{}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tuple(&mut self,)
	-> Result<Rc<TupleContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TupleContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_tuple);
        let mut _localctx: Rc<TupleContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(64);
			recog.base.match_token(LPAR,&mut recog.err_handler)?;

			{
			/*InvokeRule tuple_content*/
			recog.base.set_state(65);
			recog.tuple_content()?;

			recog.base.set_state(70);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(66);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule tuple_content*/
				recog.base.set_state(67);
				recog.tuple_content()?;

				}
				}
				recog.base.set_state(72);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			recog.base.set_state(73);
			recog.base.match_token(RPAR,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tuple_content ----------------
pub type Tuple_contentContextAll<'input> = Tuple_contentContext<'input>;


pub type Tuple_contentContext<'input> = BaseParserRuleContext<'input,Tuple_contentContextExt<'input>>;

#[derive(Clone)]
pub struct Tuple_contentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> lifParserContext<'input> for Tuple_contentContext<'input>{}

impl<'input,'a> Listenable<dyn lifListener<'input> + 'a> for Tuple_contentContext<'input>{
	fn enter(&self,listener: &mut (dyn lifListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_tuple_content(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tuple_contentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = lifParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tuple_content }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tuple_content }
}
antlr_rust::type_id!{Tuple_contentContextExt<'a>}

impl<'input> Tuple_contentContextExt<'input>{
	fn new(parent: Option<Rc<dyn lifParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tuple_contentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tuple_contentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Tuple_contentContextAttrs<'input>: lifParserContext<'input> + BorrowMut<Tuple_contentContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,lifParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
/// Retrieves first TerminalNode corresponding to token WILDCARD
/// Returns `None` if there is no child corresponding to token WILDCARD
fn WILDCARD(&self) -> Option<Rc<TerminalNode<'input,lifParserContextType>>> where Self:Sized{
	self.get_token(WILDCARD, 0)
}
/// Retrieves first TerminalNode corresponding to token NUMBER
/// Returns `None` if there is no child corresponding to token NUMBER
fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,lifParserContextType>>> where Self:Sized{
	self.get_token(NUMBER, 0)
}
/// Retrieves first TerminalNode corresponding to token CHARACTER
/// Returns `None` if there is no child corresponding to token CHARACTER
fn CHARACTER(&self) -> Option<Rc<TerminalNode<'input,lifParserContextType>>> where Self:Sized{
	self.get_token(CHARACTER, 0)
}
fn tuple(&self) -> Option<Rc<TupleContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ID
/// Returns `None` if there is no child corresponding to token ID
fn ID(&self) -> Option<Rc<TerminalNode<'input,lifParserContextType>>> where Self:Sized{
	self.get_token(ID, 0)
}

}

impl<'input> Tuple_contentContextAttrs<'input> for Tuple_contentContext<'input>{}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tuple_content(&mut self,)
	-> Result<Rc<Tuple_contentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tuple_contentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_tuple_content);
        let mut _localctx: Rc<Tuple_contentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(81);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 STRING 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(75);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
				}

			 WILDCARD 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(76);
					recog.base.match_token(WILDCARD,&mut recog.err_handler)?;

					}
				}

			 NUMBER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(77);
					recog.base.match_token(NUMBER,&mut recog.err_handler)?;

					}
				}

			 CHARACTER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(78);
					recog.base.match_token(CHARACTER,&mut recog.err_handler)?;

					}
				}

			 LPAR 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule tuple*/
					recog.base.set_state(79);
					recog.tuple()?;

					}
				}

			 ID 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					recog.base.set_state(80);
					recog.base.match_token(ID,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tupleSpaceName ----------------
pub type TupleSpaceNameContextAll<'input> = TupleSpaceNameContext<'input>;


pub type TupleSpaceNameContext<'input> = BaseParserRuleContext<'input,TupleSpaceNameContextExt<'input>>;

#[derive(Clone)]
pub struct TupleSpaceNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> lifParserContext<'input> for TupleSpaceNameContext<'input>{}

impl<'input,'a> Listenable<dyn lifListener<'input> + 'a> for TupleSpaceNameContext<'input>{
	fn enter(&self,listener: &mut (dyn lifListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_tupleSpaceName(self);
	}
}

impl<'input> CustomRuleContext<'input> for TupleSpaceNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = lifParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tupleSpaceName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tupleSpaceName }
}
antlr_rust::type_id!{TupleSpaceNameContextExt<'a>}

impl<'input> TupleSpaceNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn lifParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TupleSpaceNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TupleSpaceNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TupleSpaceNameContextAttrs<'input>: lifParserContext<'input> + BorrowMut<TupleSpaceNameContextExt<'input>>{

fn initvar(&self) -> Option<Rc<InitvarContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TupleSpaceNameContextAttrs<'input> for TupleSpaceNameContext<'input>{}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tupleSpaceName(&mut self,)
	-> Result<Rc<TupleSpaceNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TupleSpaceNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_tupleSpaceName);
        let mut _localctx: Rc<TupleSpaceNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule initvar*/
			recog.base.set_state(83);
			recog.initvar()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- initvar ----------------
pub type InitvarContextAll<'input> = InitvarContext<'input>;


pub type InitvarContext<'input> = BaseParserRuleContext<'input,InitvarContextExt<'input>>;

#[derive(Clone)]
pub struct InitvarContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> lifParserContext<'input> for InitvarContext<'input>{}

impl<'input,'a> Listenable<dyn lifListener<'input> + 'a> for InitvarContext<'input>{
	fn enter(&self,listener: &mut (dyn lifListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_initvar(self);
	}
}

impl<'input> CustomRuleContext<'input> for InitvarContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = lifParserContextType;
	fn get_rule_index(&self) -> usize { RULE_initvar }
	//fn type_rule_index() -> usize where Self: Sized { RULE_initvar }
}
antlr_rust::type_id!{InitvarContextExt<'a>}

impl<'input> InitvarContextExt<'input>{
	fn new(parent: Option<Rc<dyn lifParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InitvarContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InitvarContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InitvarContextAttrs<'input>: lifParserContext<'input> + BorrowMut<InitvarContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token NUMBER
/// Returns `None` if there is no child corresponding to token NUMBER
fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,lifParserContextType>>> where Self:Sized{
	self.get_token(NUMBER, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,lifParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
/// Retrieves first TerminalNode corresponding to token CHARACTER
/// Returns `None` if there is no child corresponding to token CHARACTER
fn CHARACTER(&self) -> Option<Rc<TerminalNode<'input,lifParserContextType>>> where Self:Sized{
	self.get_token(CHARACTER, 0)
}
/// Retrieves first TerminalNode corresponding to token ID
/// Returns `None` if there is no child corresponding to token ID
fn ID(&self) -> Option<Rc<TerminalNode<'input,lifParserContextType>>> where Self:Sized{
	self.get_token(ID, 0)
}

}

impl<'input> InitvarContextAttrs<'input> for InitvarContext<'input>{}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn initvar(&mut self,)
	-> Result<Rc<InitvarContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InitvarContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_initvar);
        let mut _localctx: Rc<InitvarContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(85);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << ID) | (1usize << NUMBER) | (1usize << STRING) | (1usize << CHARACTER))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- protocol ----------------
pub type ProtocolContextAll<'input> = ProtocolContext<'input>;


pub type ProtocolContext<'input> = BaseParserRuleContext<'input,ProtocolContextExt<'input>>;

#[derive(Clone)]
pub struct ProtocolContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> lifParserContext<'input> for ProtocolContext<'input>{}

impl<'input,'a> Listenable<dyn lifListener<'input> + 'a> for ProtocolContext<'input>{
	fn enter(&self,listener: &mut (dyn lifListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_protocol(self);
	}
}

impl<'input> CustomRuleContext<'input> for ProtocolContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = lifParserContextType;
	fn get_rule_index(&self) -> usize { RULE_protocol }
	//fn type_rule_index() -> usize where Self: Sized { RULE_protocol }
}
antlr_rust::type_id!{ProtocolContextExt<'a>}

impl<'input> ProtocolContextExt<'input>{
	fn new(parent: Option<Rc<dyn lifParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ProtocolContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ProtocolContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ProtocolContextAttrs<'input>: lifParserContext<'input> + BorrowMut<ProtocolContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token UDP
/// Returns `None` if there is no child corresponding to token UDP
fn UDP(&self) -> Option<Rc<TerminalNode<'input,lifParserContextType>>> where Self:Sized{
	self.get_token(UDP, 0)
}
/// Retrieves first TerminalNode corresponding to token TCP
/// Returns `None` if there is no child corresponding to token TCP
fn TCP(&self) -> Option<Rc<TerminalNode<'input,lifParserContextType>>> where Self:Sized{
	self.get_token(TCP, 0)
}

}

impl<'input> ProtocolContextAttrs<'input> for ProtocolContext<'input>{}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn protocol(&mut self,)
	-> Result<Rc<ProtocolContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ProtocolContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_protocol);
        let mut _localctx: Rc<ProtocolContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(87);
			_la = recog.base.input.la(1);
			if { !(_la==TCP || _la==UDP) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- ipAddress ----------------
pub type IpAddressContextAll<'input> = IpAddressContext<'input>;


pub type IpAddressContext<'input> = BaseParserRuleContext<'input,IpAddressContextExt<'input>>;

#[derive(Clone)]
pub struct IpAddressContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> lifParserContext<'input> for IpAddressContext<'input>{}

impl<'input,'a> Listenable<dyn lifListener<'input> + 'a> for IpAddressContext<'input>{
	fn enter(&self,listener: &mut (dyn lifListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ipAddress(self);
	}
}

impl<'input> CustomRuleContext<'input> for IpAddressContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = lifParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ipAddress }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ipAddress }
}
antlr_rust::type_id!{IpAddressContextExt<'a>}

impl<'input> IpAddressContextExt<'input>{
	fn new(parent: Option<Rc<dyn lifParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IpAddressContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IpAddressContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IpAddressContextAttrs<'input>: lifParserContext<'input> + BorrowMut<IpAddressContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token NUMBER in current rule
fn NUMBER_all(&self) -> Vec<Rc<TerminalNode<'input,lifParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NUMBER, starting from 0.
/// Returns `None` if number of children corresponding to token NUMBER is less or equal than `i`.
fn NUMBER(&self, i: usize) -> Option<Rc<TerminalNode<'input,lifParserContextType>>> where Self:Sized{
	self.get_token(NUMBER, i)
}
/// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input,lifParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
/// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input,lifParserContextType>>> where Self:Sized{
	self.get_token(DOT, i)
}

}

impl<'input> IpAddressContextAttrs<'input> for IpAddressContext<'input>{}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn ipAddress(&mut self,)
	-> Result<Rc<IpAddressContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IpAddressContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_ipAddress);
        let mut _localctx: Rc<IpAddressContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(89);
			recog.base.match_token(NUMBER,&mut recog.err_handler)?;

			recog.base.set_state(90);
			recog.base.match_token(DOT,&mut recog.err_handler)?;

			recog.base.set_state(91);
			recog.base.match_token(NUMBER,&mut recog.err_handler)?;

			recog.base.set_state(92);
			recog.base.match_token(DOT,&mut recog.err_handler)?;

			recog.base.set_state(93);
			recog.base.match_token(NUMBER,&mut recog.err_handler)?;

			recog.base.set_state(94);
			recog.base.match_token(DOT,&mut recog.err_handler)?;

			recog.base.set_state(95);
			recog.base.match_token(NUMBER,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- port ----------------
pub type PortContextAll<'input> = PortContext<'input>;


pub type PortContext<'input> = BaseParserRuleContext<'input,PortContextExt<'input>>;

#[derive(Clone)]
pub struct PortContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> lifParserContext<'input> for PortContext<'input>{}

impl<'input,'a> Listenable<dyn lifListener<'input> + 'a> for PortContext<'input>{
	fn enter(&self,listener: &mut (dyn lifListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_port(self);
	}
}

impl<'input> CustomRuleContext<'input> for PortContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = lifParserContextType;
	fn get_rule_index(&self) -> usize { RULE_port }
	//fn type_rule_index() -> usize where Self: Sized { RULE_port }
}
antlr_rust::type_id!{PortContextExt<'a>}

impl<'input> PortContextExt<'input>{
	fn new(parent: Option<Rc<dyn lifParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PortContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PortContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PortContextAttrs<'input>: lifParserContext<'input> + BorrowMut<PortContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token NUMBER
/// Returns `None` if there is no child corresponding to token NUMBER
fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,lifParserContextType>>> where Self:Sized{
	self.get_token(NUMBER, 0)
}

}

impl<'input> PortContextAttrs<'input> for PortContext<'input>{}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn port(&mut self,)
	-> Result<Rc<PortContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PortContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_port);
        let mut _localctx: Rc<PortContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(97);
			recog.base.match_token(NUMBER,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x21\x66\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\x0a\
	\x09\x0a\x04\x0b\x09\x0b\x03\x02\x07\x02\x18\x0a\x02\x0c\x02\x0e\x02\x1b\
	\x0b\x02\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\
	\x03\x03\x03\x03\x03\x03\x07\x03\x28\x0a\x03\x0c\x03\x0e\x03\x2b\x0b\x03\
	\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x07\x03\x34\x0a\
	\x03\x0c\x03\x0e\x03\x37\x0b\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\
	\x03\x03\x05\x03\x3f\x0a\x03\x03\x04\x03\x04\x03\x05\x03\x05\x03\x05\x03\
	\x05\x07\x05\x47\x0a\x05\x0c\x05\x0e\x05\x4a\x0b\x05\x03\x05\x03\x05\x03\
	\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x05\x06\x54\x0a\x06\x03\x07\
	\x03\x07\x03\x08\x03\x08\x03\x09\x03\x09\x03\x0a\x03\x0a\x03\x0a\x03\x0a\
	\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0b\x03\x0b\x03\x0b\x02\x02\x0c\x02\
	\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\x02\x04\x03\x02\x1a\x1d\x03\x02\x0a\
	\x0b\x02\x6a\x02\x19\x03\x02\x02\x02\x04\x3e\x03\x02\x02\x02\x06\x40\x03\
	\x02\x02\x02\x08\x42\x03\x02\x02\x02\x0a\x53\x03\x02\x02\x02\x0c\x55\x03\
	\x02\x02\x02\x0e\x57\x03\x02\x02\x02\x10\x59\x03\x02\x02\x02\x12\x5b\x03\
	\x02\x02\x02\x14\x63\x03\x02\x02\x02\x16\x18\x05\x04\x03\x02\x17\x16\x03\
	\x02\x02\x02\x18\x1b\x03\x02\x02\x02\x19\x17\x03\x02\x02\x02\x19\x1a\x03\
	\x02\x02\x02\x1a\x03\x03\x02\x02\x02\x1b\x19\x03\x02\x02\x02\x1c\x1d\x07\
	\x03\x02\x02\x1d\x1e\x05\x10\x09\x02\x1e\x1f\x07\x16\x02\x02\x1f\x20\x05\
	\x12\x0a\x02\x20\x21\x07\x16\x02\x02\x21\x22\x05\x14\x0b\x02\x22\x3f\x03\
	\x02\x02\x02\x23\x24\x07\x05\x02\x02\x24\x25\x05\x06\x04\x02\x25\x29\x05\
	\x0c\x07\x02\x26\x28\x05\x06\x04\x02\x27\x26\x03\x02\x02\x02\x28\x2b\x03\
	\x02\x02\x02\x29\x27\x03\x02\x02\x02\x29\x2a\x03\x02\x02\x02\x2a\x3f\x03\
	\x02\x02\x02\x2b\x29\x03\x02\x02\x02\x2c\x2d\x07\x06\x02\x02\x2d\x2e\x05\
	\x06\x04\x02\x2e\x2f\x05\x0c\x07\x02\x2f\x3f\x03\x02\x02\x02\x30\x31\x07\
	\x04\x02\x02\x31\x35\x05\x0c\x07\x02\x32\x34\x05\x06\x04\x02\x33\x32\x03\
	\x02\x02\x02\x34\x37\x03\x02\x02\x02\x35\x33\x03\x02\x02\x02\x35\x36\x03\
	\x02\x02\x02\x36\x3f\x03\x02\x02\x02\x37\x35\x03\x02\x02\x02\x38\x39\x07\
	\x07\x02\x02\x39\x3f\x05\x08\x05\x02\x3a\x3b\x07\x08\x02\x02\x3b\x3f\x05\
	\x08\x05\x02\x3c\x3d\x07\x09\x02\x02\x3d\x3f\x05\x08\x05\x02\x3e\x1c\x03\
	\x02\x02\x02\x3e\x23\x03\x02\x02\x02\x3e\x2c\x03\x02\x02\x02\x3e\x30\x03\
	\x02\x02\x02\x3e\x38\x03\x02\x02\x02\x3e\x3a\x03\x02\x02\x02\x3e\x3c\x03\
	\x02\x02\x02\x3f\x05\x03\x02\x02\x02\x40\x41\x05\x0e\x08\x02\x41\x07\x03\
	\x02\x02\x02\x42\x43\x07\x0c\x02\x02\x43\x48\x05\x0a\x06\x02\x44\x45\x07\
	\x0e\x02\x02\x45\x47\x05\x0a\x06\x02\x46\x44\x03\x02\x02\x02\x47\x4a\x03\
	\x02\x02\x02\x48\x46\x03\x02\x02\x02\x48\x49\x03\x02\x02\x02\x49\x4b\x03\
	\x02\x02\x02\x4a\x48\x03\x02\x02\x02\x4b\x4c\x07\x0d\x02\x02\x4c\x09\x03\
	\x02\x02\x02\x4d\x54\x07\x1c\x02\x02\x4e\x54\x07\x19\x02\x02\x4f\x54\x07\
	\x1b\x02\x02\x50\x54\x07\x1d\x02\x02\x51\x54\x05\x08\x05\x02\x52\x54\x07\
	\x1a\x02\x02\x53\x4d\x03\x02\x02\x02\x53\x4e\x03\x02\x02\x02\x53\x4f\x03\
	\x02\x02\x02\x53\x50\x03\x02\x02\x02\x53\x51\x03\x02\x02\x02\x53\x52\x03\
	\x02\x02\x02\x54\x0b\x03\x02\x02\x02\x55\x56\x05\x0e\x08\x02\x56\x0d\x03\
	\x02\x02\x02\x57\x58\x09\x02\x02\x02\x58\x0f\x03\x02\x02\x02\x59\x5a\x09\
	\x03\x02\x02\x5a\x11\x03\x02\x02\x02\x5b\x5c\x07\x1b\x02\x02\x5c\x5d\x07\
	\x15\x02\x02\x5d\x5e\x07\x1b\x02\x02\x5e\x5f\x07\x15\x02\x02\x5f\x60\x07\
	\x1b\x02\x02\x60\x61\x07\x15\x02\x02\x61\x62\x07\x1b\x02\x02\x62\x13\x03\
	\x02\x02\x02\x63\x64\x07\x1b\x02\x02\x64\x15\x03\x02\x02\x02\x08\x19\x29\
	\x35\x3e\x48\x53";

