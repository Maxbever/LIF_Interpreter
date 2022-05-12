// Generated from .\lif.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use super::liflistener::*;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::error_strategy::{DefaultErrorStrategy, ErrorStrategy};
use antlr_rust::errors::*;
use antlr_rust::int_stream::EOF;
use antlr_rust::lazy_static;
use antlr_rust::parser::{BaseParser, Parser, ParserNodeType, ParserRecog};
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::parser_rule_context::{cast, cast_mut, BaseParserRuleContext, ParserRuleContext};
use antlr_rust::recognizer::{Actions, Recognizer};
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::token::{OwningToken, Token, TOKEN_EOF};
use antlr_rust::token_factory::{CommonTokenFactory, TokenAware, TokenFactory};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::tree::*;
use antlr_rust::vocabulary::{Vocabulary, VocabularyImpl};
use antlr_rust::PredictionContextCache;
use antlr_rust::TokenSource;
use antlr_rust::{TidAble, TidExt};

use std::any::{Any, TypeId};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

pub const CONNECT: isize = 1;
pub const ATTACH: isize = 2;
pub const CREATE: isize = 3;
pub const DELETE: isize = 4;
pub const OUT: isize = 5;
pub const READ: isize = 6;
pub const IN: isize = 7;
pub const TCP: isize = 8;
pub const UDP: isize = 9;
pub const VAR: isize = 10;
pub const GET: isize = 11;
pub const LEN: isize = 12;
pub const FOR: isize = 13;
pub const TO: isize = 14;
pub const LPAR: isize = 15;
pub const RPAR: isize = 16;
pub const COMMA: isize = 17;
pub const DOUBLEQUOTE: isize = 18;
pub const QUOTE: isize = 19;
pub const SLASH: isize = 20;
pub const BACKSLASH: isize = 21;
pub const LBRACKET: isize = 22;
pub const RBRACKET: isize = 23;
pub const DOT: isize = 24;
pub const DOUBLEDOT: isize = 25;
pub const SEMICOLON: isize = 26;
pub const KLEENE: isize = 27;
pub const WILDCARD: isize = 28;
pub const EQUAL: isize = 29;
pub const PLUS: isize = 30;
pub const MINUS: isize = 31;
pub const RIGHT_BRACE: isize = 32;
pub const LEFT_BRACE: isize = 33;
pub const ID: isize = 34;
pub const NUMBER: isize = 35;
pub const STRING: isize = 36;
pub const CHARACTER: isize = 37;
pub const LINECOMMENT: isize = 38;
pub const COMMENT: isize = 39;
pub const NEWLINE: isize = 40;
pub const WS: isize = 41;
pub const RULE_root: usize = 0;
pub const RULE_instruction: usize = 1;
pub const RULE_connect: usize = 2;
pub const RULE_create: usize = 3;
pub const RULE_delete: usize = 4;
pub const RULE_attach: usize = 5;
pub const RULE_read: usize = 6;
pub const RULE_in_instr: usize = 7;
pub const RULE_out: usize = 8;
pub const RULE_for_instr: usize = 9;
pub const RULE_operation: usize = 10;
pub const RULE_get_function: usize = 11;
pub const RULE_len_function: usize = 12;
pub const RULE_assignation: usize = 13;
pub const RULE_attribut: usize = 14;
pub const RULE_right_expr: usize = 15;
pub const RULE_tuple: usize = 16;
pub const RULE_tuple_content: usize = 17;
pub const RULE_tuple_space_name: usize = 18;
pub const RULE_init_var: usize = 19;
pub const RULE_protocol: usize = 20;
pub const RULE_ip_address: usize = 21;
pub const RULE_port: usize = 22;
pub const ruleNames: [&'static str; 23] = [
    "root",
    "instruction",
    "connect",
    "create",
    "delete",
    "attach",
    "read",
    "in_instr",
    "out",
    "for_instr",
    "operation",
    "get_function",
    "len_function",
    "assignation",
    "attribut",
    "right_expr",
    "tuple",
    "tuple_content",
    "tuple_space_name",
    "init_var",
    "protocol",
    "ip_address",
    "port",
];

pub const _LITERAL_NAMES: [Option<&'static str>; 34] = [
    None,
    Some("'connect'"),
    Some("'attach'"),
    Some("'create'"),
    Some("'delete'"),
    Some("'out'"),
    Some("'read'"),
    Some("'in'"),
    Some("'tcp'"),
    Some("'udp'"),
    Some("'var'"),
    Some("'get'"),
    Some("'len'"),
    Some("'for'"),
    Some("'to'"),
    Some("'('"),
    Some("')'"),
    Some("','"),
    Some("'\"'"),
    Some("'''"),
    Some("'/'"),
    Some("'\\'"),
    Some("'['"),
    Some("']'"),
    Some("'.'"),
    Some("':'"),
    Some("';'"),
    Some("'*'"),
    Some("'_'"),
    Some("'='"),
    Some("'+'"),
    Some("'-'"),
    Some("'}'"),
    Some("'{'"),
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 42] = [
    None,
    Some("CONNECT"),
    Some("ATTACH"),
    Some("CREATE"),
    Some("DELETE"),
    Some("OUT"),
    Some("READ"),
    Some("IN"),
    Some("TCP"),
    Some("UDP"),
    Some("VAR"),
    Some("GET"),
    Some("LEN"),
    Some("FOR"),
    Some("TO"),
    Some("LPAR"),
    Some("RPAR"),
    Some("COMMA"),
    Some("DOUBLEQUOTE"),
    Some("QUOTE"),
    Some("SLASH"),
    Some("BACKSLASH"),
    Some("LBRACKET"),
    Some("RBRACKET"),
    Some("DOT"),
    Some("DOUBLEDOT"),
    Some("SEMICOLON"),
    Some("KLEENE"),
    Some("WILDCARD"),
    Some("EQUAL"),
    Some("PLUS"),
    Some("MINUS"),
    Some("RIGHT_BRACE"),
    Some("LEFT_BRACE"),
    Some("ID"),
    Some("NUMBER"),
    Some("STRING"),
    Some("CHARACTER"),
    Some("LINECOMMENT"),
    Some("COMMENT"),
    Some("NEWLINE"),
    Some("WS"),
];
lazy_static! {
    static ref _shared_context_cache: Arc<PredictionContextCache> =
        Arc::new(PredictionContextCache::new());
    static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(
        _LITERAL_NAMES.iter(),
        _SYMBOLIC_NAMES.iter(),
        None
    ));
}

type BaseParserType<'input, I> =
    BaseParser<'input, lifParserExt, I, lifParserContextType, dyn lifListener<'input> + 'input>;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type lifTreeWalker<'input, 'a> =
    ParseTreeWalker<'input, 'a, lifParserContextType, dyn lifListener<'input> + 'a>;

/// Parser for lif grammar
pub struct lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    base: BaseParserType<'input, I>,
    interpreter: Arc<ParserATNSimulator>,
    _shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn get_serialized_atn() -> &'static str {
        _serializedATN
    }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
        antlr_rust::recognizer::check_version("0", "2");
        let interpreter = Arc::new(ParserATNSimulator::new(
            _ATN.clone(),
            _decision_to_DFA.clone(),
            _shared_context_cache.clone(),
        ));
        Self {
            base: BaseParser::new_base_parser(input, Arc::clone(&interpreter), lifParserExt {}),
            interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }
}

type DynStrategy<'input, I> = Box<dyn ErrorStrategy<'input, BaseParserType<'input, I>> + 'input>;

impl<'input, I> lifParser<'input, I, DynStrategy<'input, I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self {
        Self::with_strategy(input, Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> lifParser<'input, I, DefaultErrorStrategy<'input, lifParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn new(input: I) -> Self {
        Self::with_strategy(input, DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for lifParser
pub trait lifParserContext<'input>:
    for<'x> Listenable<dyn lifListener<'input> + 'x>
    + ParserRuleContext<'input, TF = LocalTokenFactory<'input>, Ctx = lifParserContextType>
{
}

impl<'input> lifParserContext<'input> for TerminalNode<'input, lifParserContextType> {}
impl<'input> lifParserContext<'input> for ErrorNode<'input, lifParserContextType> {}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn lifParserContext<'input> + 'input {}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn lifListener<'input> + 'input {}

pub struct lifParserContextType;
antlr_rust::type_id! {lifParserContextType}

impl<'input> ParserNodeType<'input> for lifParserContextType {
    type TF = LocalTokenFactory<'input>;
    type Type = dyn lifParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    type Target = BaseParserType<'input, I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct lifParserExt {}

impl lifParserExt {}

impl<'input> TokenAware<'input> for lifParserExt {
    type TF = LocalTokenFactory<'input>;
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    ParserRecog<'input, BaseParserType<'input, I>> for lifParserExt
{
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    Actions<'input, BaseParserType<'input, I>> for lifParserExt
{
    fn get_grammar_file_name(&self) -> &str {
        "lif.g4"
    }

    fn get_rule_names(&self) -> &[&str] {
        &ruleNames
    }

    fn get_vocabulary(&self) -> &dyn Vocabulary {
        &**VOCABULARY
    }
    fn sempred(
        _localctx: Option<&(dyn lifParserContext<'input> + 'input)>,
        rule_index: isize,
        pred_index: isize,
        recog: &mut BaseParserType<'input, I>,
    ) -> bool {
        match rule_index {
            10 => lifParser::<'input, I, _>::operation_sempred(
                _localctx.and_then(|x| x.downcast_ref()),
                pred_index,
                recog,
            ),
            _ => true,
        }
    }
}

impl<'input, I> lifParser<'input, I, DefaultErrorStrategy<'input, lifParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    fn operation_sempred(
        _localctx: Option<&OperationContext<'input>>,
        pred_index: isize,
        recog: &mut <Self as Deref>::Target,
    ) -> bool {
        match pred_index {
            0 => recog.precpred(None, 4),
            1 => recog.precpred(None, 3),
            2 => recog.precpred(None, 2),
            3 => recog.precpred(None, 1),
            _ => true,
        }
    }
}
//------------------- root ----------------
pub type RootContextAll<'input> = RootContext<'input>;

pub type RootContext<'input> = BaseParserRuleContext<'input, RootContextExt<'input>>;

#[derive(Clone)]
pub struct RootContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> lifParserContext<'input> for RootContext<'input> {}

impl<'input, 'a> Listenable<dyn lifListener<'input> + 'a> for RootContext<'input> {
    fn enter(&self, listener: &mut (dyn lifListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_root(self);
    }
}

impl<'input> CustomRuleContext<'input> for RootContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = lifParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_root
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_root }
}
antlr_rust::type_id! {RootContextExt<'a>}

impl<'input> RootContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn lifParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<RootContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            RootContextExt { ph: PhantomData },
        ))
    }
}

pub trait RootContextAttrs<'input>:
    lifParserContext<'input> + BorrowMut<RootContextExt<'input>>
{
    fn instruction_all(&self) -> Vec<Rc<InstructionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn instruction(&self, i: usize) -> Option<Rc<InstructionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> RootContextAttrs<'input> for RootContext<'input> {}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn root(&mut self) -> Result<Rc<RootContextAll<'input>>, ANTLRError> {
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
                recog.base.set_state(49);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while ((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << CONNECT)
                            | (1usize << ATTACH)
                            | (1usize << CREATE)
                            | (1usize << DELETE)
                            | (1usize << OUT)
                            | (1usize << VAR)
                            | (1usize << FOR)))
                        != 0
                {
                    {
                        {
                            /*InvokeRule instruction*/
                            recog.base.set_state(46);
                            recog.instruction()?;
                        }
                    }
                    recog.base.set_state(51);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
        };
        match result {
            Ok(_) => {}
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

pub type InstructionContext<'input> = BaseParserRuleContext<'input, InstructionContextExt<'input>>;

#[derive(Clone)]
pub struct InstructionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> lifParserContext<'input> for InstructionContext<'input> {}

impl<'input, 'a> Listenable<dyn lifListener<'input> + 'a> for InstructionContext<'input> {
    fn enter(&self, listener: &mut (dyn lifListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_instruction(self);
    }
}

impl<'input> CustomRuleContext<'input> for InstructionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = lifParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_instruction
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_instruction }
}
antlr_rust::type_id! {InstructionContextExt<'a>}

impl<'input> InstructionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn lifParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<InstructionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            InstructionContextExt { ph: PhantomData },
        ))
    }
}

pub trait InstructionContextAttrs<'input>:
    lifParserContext<'input> + BorrowMut<InstructionContextExt<'input>>
{
    fn connect(&self) -> Option<Rc<ConnectContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn create(&self) -> Option<Rc<CreateContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn delete(&self) -> Option<Rc<DeleteContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn attach(&self) -> Option<Rc<AttachContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn out(&self) -> Option<Rc<OutContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn for_instr(&self) -> Option<Rc<For_instrContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn assignation(&self) -> Option<Rc<AssignationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> InstructionContextAttrs<'input> for InstructionContext<'input> {}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn instruction(&mut self) -> Result<Rc<InstructionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = InstructionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 2, RULE_instruction);
        let mut _localctx: Rc<InstructionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(59);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                CONNECT => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule connect*/
                        recog.base.set_state(52);
                        recog.connect()?;
                    }
                }

                CREATE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule create*/
                        recog.base.set_state(53);
                        recog.create()?;
                    }
                }

                DELETE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        /*InvokeRule delete*/
                        recog.base.set_state(54);
                        recog.delete()?;
                    }
                }

                ATTACH => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        /*InvokeRule attach*/
                        recog.base.set_state(55);
                        recog.attach()?;
                    }
                }

                OUT => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        /*InvokeRule out*/
                        recog.base.set_state(56);
                        recog.out()?;
                    }
                }

                FOR => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6);
                    recog.base.enter_outer_alt(None, 6);
                    {
                        /*InvokeRule for_instr*/
                        recog.base.set_state(57);
                        recog.for_instr()?;
                    }
                }

                VAR => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 7);
                    recog.base.enter_outer_alt(None, 7);
                    {
                        /*InvokeRule assignation*/
                        recog.base.set_state(58);
                        recog.assignation()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
        };
        match result {
            Ok(_) => {}
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
//------------------- connect ----------------
pub type ConnectContextAll<'input> = ConnectContext<'input>;

pub type ConnectContext<'input> = BaseParserRuleContext<'input, ConnectContextExt<'input>>;

#[derive(Clone)]
pub struct ConnectContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> lifParserContext<'input> for ConnectContext<'input> {}

impl<'input, 'a> Listenable<dyn lifListener<'input> + 'a> for ConnectContext<'input> {
    fn enter(&self, listener: &mut (dyn lifListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_connect(self);
    }
}

impl<'input> CustomRuleContext<'input> for ConnectContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = lifParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_connect
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_connect }
}
antlr_rust::type_id! {ConnectContextExt<'a>}

impl<'input> ConnectContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn lifParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ConnectContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ConnectContextExt { ph: PhantomData },
        ))
    }
}

pub trait ConnectContextAttrs<'input>:
    lifParserContext<'input> + BorrowMut<ConnectContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CONNECT
    /// Returns `None` if there is no child corresponding to token CONNECT
    fn CONNECT(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CONNECT, 0)
    }
    fn protocol(&self) -> Option<Rc<ProtocolContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token DOUBLEDOT in current rule
    fn DOUBLEDOT_all(&self) -> Vec<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token DOUBLEDOT, starting from 0.
    /// Returns `None` if number of children corresponding to token DOUBLEDOT is less or equal than `i`.
    fn DOUBLEDOT(&self, i: usize) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOUBLEDOT, i)
    }
    fn ip_address(&self) -> Option<Rc<Ip_addressContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn port(&self) -> Option<Rc<PortContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> ConnectContextAttrs<'input> for ConnectContext<'input> {}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn connect(&mut self) -> Result<Rc<ConnectContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ConnectContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_connect);
        let mut _localctx: Rc<ConnectContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(61);
                recog.base.match_token(CONNECT, &mut recog.err_handler)?;

                /*InvokeRule protocol*/
                recog.base.set_state(62);
                recog.protocol()?;

                recog.base.set_state(63);
                recog.base.match_token(DOUBLEDOT, &mut recog.err_handler)?;

                /*InvokeRule ip_address*/
                recog.base.set_state(64);
                recog.ip_address()?;

                recog.base.set_state(65);
                recog.base.match_token(DOUBLEDOT, &mut recog.err_handler)?;

                /*InvokeRule port*/
                recog.base.set_state(66);
                recog.port()?;
            }
        };
        match result {
            Ok(_) => {}
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
//------------------- create ----------------
pub type CreateContextAll<'input> = CreateContext<'input>;

pub type CreateContext<'input> = BaseParserRuleContext<'input, CreateContextExt<'input>>;

#[derive(Clone)]
pub struct CreateContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> lifParserContext<'input> for CreateContext<'input> {}

impl<'input, 'a> Listenable<dyn lifListener<'input> + 'a> for CreateContext<'input> {
    fn enter(&self, listener: &mut (dyn lifListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_create(self);
    }
}

impl<'input> CustomRuleContext<'input> for CreateContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = lifParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_create
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_create }
}
antlr_rust::type_id! {CreateContextExt<'a>}

impl<'input> CreateContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn lifParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<CreateContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            CreateContextExt { ph: PhantomData },
        ))
    }
}

pub trait CreateContextAttrs<'input>:
    lifParserContext<'input> + BorrowMut<CreateContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CREATE
    /// Returns `None` if there is no child corresponding to token CREATE
    fn CREATE(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CREATE, 0)
    }
    fn attribut_all(&self) -> Vec<Rc<AttributContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn attribut(&self, i: usize) -> Option<Rc<AttributContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn tuple_space_name(&self) -> Option<Rc<Tuple_space_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> CreateContextAttrs<'input> for CreateContext<'input> {}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn create(&mut self) -> Result<Rc<CreateContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = CreateContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_create);
        let mut _localctx: Rc<CreateContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(68);
                recog.base.match_token(CREATE, &mut recog.err_handler)?;

                /*InvokeRule attribut*/
                recog.base.set_state(69);
                recog.attribut()?;

                /*InvokeRule tuple_space_name*/
                recog.base.set_state(70);
                recog.tuple_space_name()?;

                recog.base.set_state(74);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == ID || _la == STRING {
                    {
                        {
                            /*InvokeRule attribut*/
                            recog.base.set_state(71);
                            recog.attribut()?;
                        }
                    }
                    recog.base.set_state(76);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
        };
        match result {
            Ok(_) => {}
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
//------------------- delete ----------------
pub type DeleteContextAll<'input> = DeleteContext<'input>;

pub type DeleteContext<'input> = BaseParserRuleContext<'input, DeleteContextExt<'input>>;

#[derive(Clone)]
pub struct DeleteContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> lifParserContext<'input> for DeleteContext<'input> {}

impl<'input, 'a> Listenable<dyn lifListener<'input> + 'a> for DeleteContext<'input> {
    fn enter(&self, listener: &mut (dyn lifListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_delete(self);
    }
}

impl<'input> CustomRuleContext<'input> for DeleteContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = lifParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_delete
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_delete }
}
antlr_rust::type_id! {DeleteContextExt<'a>}

impl<'input> DeleteContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn lifParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<DeleteContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            DeleteContextExt { ph: PhantomData },
        ))
    }
}

pub trait DeleteContextAttrs<'input>:
    lifParserContext<'input> + BorrowMut<DeleteContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token DELETE
    /// Returns `None` if there is no child corresponding to token DELETE
    fn DELETE(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DELETE, 0)
    }
    fn attribut(&self) -> Option<Rc<AttributContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn tuple_space_name(&self) -> Option<Rc<Tuple_space_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> DeleteContextAttrs<'input> for DeleteContext<'input> {}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn delete(&mut self) -> Result<Rc<DeleteContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = DeleteContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_delete);
        let mut _localctx: Rc<DeleteContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(77);
                recog.base.match_token(DELETE, &mut recog.err_handler)?;

                /*InvokeRule attribut*/
                recog.base.set_state(78);
                recog.attribut()?;

                /*InvokeRule tuple_space_name*/
                recog.base.set_state(79);
                recog.tuple_space_name()?;
            }
        };
        match result {
            Ok(_) => {}
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
//------------------- attach ----------------
pub type AttachContextAll<'input> = AttachContext<'input>;

pub type AttachContext<'input> = BaseParserRuleContext<'input, AttachContextExt<'input>>;

#[derive(Clone)]
pub struct AttachContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> lifParserContext<'input> for AttachContext<'input> {}

impl<'input, 'a> Listenable<dyn lifListener<'input> + 'a> for AttachContext<'input> {
    fn enter(&self, listener: &mut (dyn lifListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_attach(self);
    }
}

impl<'input> CustomRuleContext<'input> for AttachContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = lifParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_attach
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_attach }
}
antlr_rust::type_id! {AttachContextExt<'a>}

impl<'input> AttachContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn lifParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<AttachContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            AttachContextExt { ph: PhantomData },
        ))
    }
}

pub trait AttachContextAttrs<'input>:
    lifParserContext<'input> + BorrowMut<AttachContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ATTACH
    /// Returns `None` if there is no child corresponding to token ATTACH
    fn ATTACH(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ATTACH, 0)
    }
    fn tuple_space_name(&self) -> Option<Rc<Tuple_space_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn attribut_all(&self) -> Vec<Rc<AttributContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn attribut(&self, i: usize) -> Option<Rc<AttributContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> AttachContextAttrs<'input> for AttachContext<'input> {}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn attach(&mut self) -> Result<Rc<AttachContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = AttachContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_attach);
        let mut _localctx: Rc<AttachContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(81);
                recog.base.match_token(ATTACH, &mut recog.err_handler)?;

                /*InvokeRule tuple_space_name*/
                recog.base.set_state(82);
                recog.tuple_space_name()?;

                recog.base.set_state(86);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == ID || _la == STRING {
                    {
                        {
                            /*InvokeRule attribut*/
                            recog.base.set_state(83);
                            recog.attribut()?;
                        }
                    }
                    recog.base.set_state(88);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
        };
        match result {
            Ok(_) => {}
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
//------------------- read ----------------
pub type ReadContextAll<'input> = ReadContext<'input>;

pub type ReadContext<'input> = BaseParserRuleContext<'input, ReadContextExt<'input>>;

#[derive(Clone)]
pub struct ReadContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> lifParserContext<'input> for ReadContext<'input> {}

impl<'input, 'a> Listenable<dyn lifListener<'input> + 'a> for ReadContext<'input> {
    fn enter(&self, listener: &mut (dyn lifListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_read(self);
    }
}

impl<'input> CustomRuleContext<'input> for ReadContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = lifParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_read
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_read }
}
antlr_rust::type_id! {ReadContextExt<'a>}

impl<'input> ReadContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn lifParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ReadContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ReadContextExt { ph: PhantomData },
        ))
    }
}

pub trait ReadContextAttrs<'input>:
    lifParserContext<'input> + BorrowMut<ReadContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token READ
    /// Returns `None` if there is no child corresponding to token READ
    fn READ(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(READ, 0)
    }
    fn tuple_all(&self) -> Vec<Rc<TupleContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn tuple(&self, i: usize) -> Option<Rc<TupleContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> ReadContextAttrs<'input> for ReadContext<'input> {}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn read(&mut self) -> Result<Rc<ReadContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ReadContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_read);
        let mut _localctx: Rc<ReadContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(89);
                recog.base.match_token(READ, &mut recog.err_handler)?;

                /*InvokeRule tuple*/
                recog.base.set_state(90);
                recog.tuple()?;

                recog.base.set_state(95);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == COMMA {
                    {
                        {
                            recog.base.set_state(91);
                            recog.base.match_token(COMMA, &mut recog.err_handler)?;

                            /*InvokeRule tuple*/
                            recog.base.set_state(92);
                            recog.tuple()?;
                        }
                    }
                    recog.base.set_state(97);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
        };
        match result {
            Ok(_) => {}
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
//------------------- in_instr ----------------
pub type In_instrContextAll<'input> = In_instrContext<'input>;

pub type In_instrContext<'input> = BaseParserRuleContext<'input, In_instrContextExt<'input>>;

#[derive(Clone)]
pub struct In_instrContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> lifParserContext<'input> for In_instrContext<'input> {}

impl<'input, 'a> Listenable<dyn lifListener<'input> + 'a> for In_instrContext<'input> {
    fn enter(&self, listener: &mut (dyn lifListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_in_instr(self);
    }
}

impl<'input> CustomRuleContext<'input> for In_instrContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = lifParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_in_instr
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_in_instr }
}
antlr_rust::type_id! {In_instrContextExt<'a>}

impl<'input> In_instrContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn lifParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<In_instrContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            In_instrContextExt { ph: PhantomData },
        ))
    }
}

pub trait In_instrContextAttrs<'input>:
    lifParserContext<'input> + BorrowMut<In_instrContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token IN
    /// Returns `None` if there is no child corresponding to token IN
    fn IN(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IN, 0)
    }
    fn tuple_all(&self) -> Vec<Rc<TupleContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn tuple(&self, i: usize) -> Option<Rc<TupleContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> In_instrContextAttrs<'input> for In_instrContext<'input> {}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn in_instr(&mut self) -> Result<Rc<In_instrContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = In_instrContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_in_instr);
        let mut _localctx: Rc<In_instrContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(98);
                recog.base.match_token(IN, &mut recog.err_handler)?;

                /*InvokeRule tuple*/
                recog.base.set_state(99);
                recog.tuple()?;

                recog.base.set_state(104);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == COMMA {
                    {
                        {
                            recog.base.set_state(100);
                            recog.base.match_token(COMMA, &mut recog.err_handler)?;

                            /*InvokeRule tuple*/
                            recog.base.set_state(101);
                            recog.tuple()?;
                        }
                    }
                    recog.base.set_state(106);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
        };
        match result {
            Ok(_) => {}
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
//------------------- out ----------------
pub type OutContextAll<'input> = OutContext<'input>;

pub type OutContext<'input> = BaseParserRuleContext<'input, OutContextExt<'input>>;

#[derive(Clone)]
pub struct OutContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> lifParserContext<'input> for OutContext<'input> {}

impl<'input, 'a> Listenable<dyn lifListener<'input> + 'a> for OutContext<'input> {
    fn enter(&self, listener: &mut (dyn lifListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_out(self);
    }
}

impl<'input> CustomRuleContext<'input> for OutContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = lifParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_out
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_out }
}
antlr_rust::type_id! {OutContextExt<'a>}

impl<'input> OutContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn lifParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OutContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OutContextExt { ph: PhantomData },
        ))
    }
}

pub trait OutContextAttrs<'input>:
    lifParserContext<'input> + BorrowMut<OutContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token OUT
    /// Returns `None` if there is no child corresponding to token OUT
    fn OUT(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OUT, 0)
    }
    fn tuple_all(&self) -> Vec<Rc<TupleContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn tuple(&self, i: usize) -> Option<Rc<TupleContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> OutContextAttrs<'input> for OutContext<'input> {}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn out(&mut self) -> Result<Rc<OutContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OutContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_out);
        let mut _localctx: Rc<OutContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(107);
                recog.base.match_token(OUT, &mut recog.err_handler)?;

                /*InvokeRule tuple*/
                recog.base.set_state(108);
                recog.tuple()?;

                recog.base.set_state(113);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == COMMA {
                    {
                        {
                            recog.base.set_state(109);
                            recog.base.match_token(COMMA, &mut recog.err_handler)?;

                            /*InvokeRule tuple*/
                            recog.base.set_state(110);
                            recog.tuple()?;
                        }
                    }
                    recog.base.set_state(115);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
        };
        match result {
            Ok(_) => {}
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
//------------------- for_instr ----------------
pub type For_instrContextAll<'input> = For_instrContext<'input>;

pub type For_instrContext<'input> = BaseParserRuleContext<'input, For_instrContextExt<'input>>;

#[derive(Clone)]
pub struct For_instrContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> lifParserContext<'input> for For_instrContext<'input> {}

impl<'input, 'a> Listenable<dyn lifListener<'input> + 'a> for For_instrContext<'input> {
    fn enter(&self, listener: &mut (dyn lifListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_for_instr(self);
    }
}

impl<'input> CustomRuleContext<'input> for For_instrContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = lifParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_for_instr
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_for_instr }
}
antlr_rust::type_id! {For_instrContextExt<'a>}

impl<'input> For_instrContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn lifParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<For_instrContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            For_instrContextExt { ph: PhantomData },
        ))
    }
}

pub trait For_instrContextAttrs<'input>:
    lifParserContext<'input> + BorrowMut<For_instrContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token FOR
    /// Returns `None` if there is no child corresponding to token FOR
    fn FOR(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(FOR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ID, 0)
    }
    /// Retrieves first TerminalNode corresponding to token EQUAL
    /// Returns `None` if there is no child corresponding to token EQUAL
    fn EQUAL(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EQUAL, 0)
    }
    fn operation_all(&self) -> Vec<Rc<OperationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn operation(&self, i: usize) -> Option<Rc<OperationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token TO
    /// Returns `None` if there is no child corresponding to token TO
    fn TO(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(TO, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LPAR
    /// Returns `None` if there is no child corresponding to token LPAR
    fn LPAR(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LPAR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RPAR
    /// Returns `None` if there is no child corresponding to token RPAR
    fn RPAR(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RPAR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LEFT_BRACE
    /// Returns `None` if there is no child corresponding to token LEFT_BRACE
    fn LEFT_BRACE(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LEFT_BRACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RIGHT_BRACE
    /// Returns `None` if there is no child corresponding to token RIGHT_BRACE
    fn RIGHT_BRACE(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RIGHT_BRACE, 0)
    }
    fn instruction_all(&self) -> Vec<Rc<InstructionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn instruction(&self, i: usize) -> Option<Rc<InstructionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> For_instrContextAttrs<'input> for For_instrContext<'input> {}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn for_instr(&mut self) -> Result<Rc<For_instrContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = For_instrContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_for_instr);
        let mut _localctx: Rc<For_instrContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(116);
                recog.base.match_token(FOR, &mut recog.err_handler)?;

                recog.base.set_state(117);
                recog.base.match_token(ID, &mut recog.err_handler)?;

                recog.base.set_state(118);
                recog.base.match_token(EQUAL, &mut recog.err_handler)?;

                /*InvokeRule operation*/
                recog.base.set_state(119);
                recog.operation_rec(0)?;

                recog.base.set_state(120);
                recog.base.match_token(TO, &mut recog.err_handler)?;

                recog.base.set_state(121);
                recog.base.match_token(LPAR, &mut recog.err_handler)?;

                /*InvokeRule operation*/
                recog.base.set_state(122);
                recog.operation_rec(0)?;

                recog.base.set_state(123);
                recog.base.match_token(RPAR, &mut recog.err_handler)?;

                recog.base.set_state(124);
                recog.base.match_token(LEFT_BRACE, &mut recog.err_handler)?;

                recog.base.set_state(126);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                loop {
                    {
                        {
                            /*InvokeRule instruction*/
                            recog.base.set_state(125);
                            recog.instruction()?;
                        }
                    }
                    recog.base.set_state(128);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                    if !(((_la) & !0x3f) == 0
                        && ((1usize << _la)
                            & ((1usize << CONNECT)
                                | (1usize << ATTACH)
                                | (1usize << CREATE)
                                | (1usize << DELETE)
                                | (1usize << OUT)
                                | (1usize << VAR)
                                | (1usize << FOR)))
                            != 0)
                    {
                        break;
                    }
                }
                recog.base.set_state(130);
                recog
                    .base
                    .match_token(RIGHT_BRACE, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
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
//------------------- operation ----------------
pub type OperationContextAll<'input> = OperationContext<'input>;

pub type OperationContext<'input> = BaseParserRuleContext<'input, OperationContextExt<'input>>;

#[derive(Clone)]
pub struct OperationContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> lifParserContext<'input> for OperationContext<'input> {}

impl<'input, 'a> Listenable<dyn lifListener<'input> + 'a> for OperationContext<'input> {
    fn enter(&self, listener: &mut (dyn lifListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_operation(self);
    }
}

impl<'input> CustomRuleContext<'input> for OperationContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = lifParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_operation
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_operation }
}
antlr_rust::type_id! {OperationContextExt<'a>}

impl<'input> OperationContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn lifParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OperationContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OperationContextExt { ph: PhantomData },
        ))
    }
}

pub trait OperationContextAttrs<'input>:
    lifParserContext<'input> + BorrowMut<OperationContextExt<'input>>
{
    fn get_function(&self) -> Option<Rc<Get_functionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn len_function(&self) -> Option<Rc<Len_functionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn right_expr(&self) -> Option<Rc<Right_exprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn operation_all(&self) -> Vec<Rc<OperationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn operation(&self, i: usize) -> Option<Rc<OperationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token PLUS
    /// Returns `None` if there is no child corresponding to token PLUS
    fn PLUS(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PLUS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token MINUS
    /// Returns `None` if there is no child corresponding to token MINUS
    fn MINUS(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MINUS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token KLEENE
    /// Returns `None` if there is no child corresponding to token KLEENE
    fn KLEENE(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(KLEENE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SLASH
    /// Returns `None` if there is no child corresponding to token SLASH
    fn SLASH(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SLASH, 0)
    }
}

impl<'input> OperationContextAttrs<'input> for OperationContext<'input> {}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn operation(&mut self) -> Result<Rc<OperationContextAll<'input>>, ANTLRError> {
        self.operation_rec(0)
    }

    fn operation_rec(&mut self, _p: isize) -> Result<Rc<OperationContextAll<'input>>, ANTLRError> {
        let recog = self;
        let _parentctx = recog.ctx.take();
        let _parentState = recog.base.get_state();
        let mut _localctx = OperationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_recursion_rule(_localctx.clone(), 20, RULE_operation, _p);
        let mut _localctx: Rc<OperationContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
        let _startState = 20;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(136);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(8, &mut recog.base)? {
                    1 => {
                        {
                            /*InvokeRule get_function*/
                            recog.base.set_state(133);
                            recog.get_function()?;
                        }
                    }
                    2 => {
                        {
                            /*InvokeRule len_function*/
                            recog.base.set_state(134);
                            recog.len_function()?;
                        }
                    }
                    3 => {
                        {
                            /*InvokeRule right_expr*/
                            recog.base.set_state(135);
                            recog.right_expr()?;
                        }
                    }

                    _ => {}
                }

                let tmp = recog.input.lt(-1).cloned();
                recog.ctx.as_ref().unwrap().set_stop(tmp);
                recog.base.set_state(152);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(10, &mut recog.base)?;
                while  _alt != 2 && _alt != INVALID_ALT {
                    if _alt == 1 {
                        recog.trigger_exit_rule_event();
                        _prevctx = _localctx.clone();
                        {
                            recog.base.set_state(150);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(9, &mut recog.base)? {
                                1 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp = OperationContextExt::new(
                                            _parentctx.clone(),
                                            _parentState,
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_operation,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(138);
                                        if !({ recog.precpred(None, 4) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 4)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(139);
                                        recog.base.match_token(PLUS, &mut recog.err_handler)?;

                                        /*InvokeRule operation*/
                                        recog.base.set_state(140);
                                        recog.operation_rec(5)?;
                                    }
                                }
                                2 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp = OperationContextExt::new(
                                            _parentctx.clone(),
                                            _parentState,
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_operation,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(141);
                                        if !({ recog.precpred(None, 3) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 3)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(142);
                                        recog.base.match_token(MINUS, &mut recog.err_handler)?;

                                        /*InvokeRule operation*/
                                        recog.base.set_state(143);
                                        recog.operation_rec(4)?;
                                    }
                                }
                                3 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp = OperationContextExt::new(
                                            _parentctx.clone(),
                                            _parentState,
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_operation,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(144);
                                        if !({ recog.precpred(None, 2) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 2)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(145);
                                        recog.base.match_token(KLEENE, &mut recog.err_handler)?;

                                        /*InvokeRule operation*/
                                        recog.base.set_state(146);
                                        recog.operation_rec(3)?;
                                    }
                                }
                                4 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp = OperationContextExt::new(
                                            _parentctx.clone(),
                                            _parentState,
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_operation,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(147);
                                        if !({ recog.precpred(None, 1) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 1)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(148);
                                        recog.base.match_token(SLASH, &mut recog.err_handler)?;

                                        /*InvokeRule operation*/
                                        recog.base.set_state(149);
                                        recog.operation_rec(2)?;
                                    }
                                }

                                _ => {}
                            }
                        }
                    }
                    recog.base.set_state(154);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(10, &mut recog.base)?;
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.unroll_recursion_context(_parentctx);

        Ok(_localctx)
    }
}
//------------------- get_function ----------------
pub type Get_functionContextAll<'input> = Get_functionContext<'input>;

pub type Get_functionContext<'input> =
    BaseParserRuleContext<'input, Get_functionContextExt<'input>>;

#[derive(Clone)]
pub struct Get_functionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> lifParserContext<'input> for Get_functionContext<'input> {}

impl<'input, 'a> Listenable<dyn lifListener<'input> + 'a> for Get_functionContext<'input> {
    fn enter(&self, listener: &mut (dyn lifListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_get_function(self);
    }
}

impl<'input> CustomRuleContext<'input> for Get_functionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = lifParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_get_function
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_get_function }
}
antlr_rust::type_id! {Get_functionContextExt<'a>}

impl<'input> Get_functionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn lifParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Get_functionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Get_functionContextExt { ph: PhantomData },
        ))
    }
}

pub trait Get_functionContextAttrs<'input>:
    lifParserContext<'input> + BorrowMut<Get_functionContextExt<'input>>
{
    fn tuple(&self) -> Option<Rc<TupleContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token DOT
    /// Returns `None` if there is no child corresponding to token DOT
    fn DOT(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GET
    /// Returns `None` if there is no child corresponding to token GET
    fn GET(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GET, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LPAR
    /// Returns `None` if there is no child corresponding to token LPAR
    fn LPAR(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LPAR, 0)
    }
    fn right_expr(&self) -> Option<Rc<Right_exprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token RPAR
    /// Returns `None` if there is no child corresponding to token RPAR
    fn RPAR(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RPAR, 0)
    }
}

impl<'input> Get_functionContextAttrs<'input> for Get_functionContext<'input> {}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn get_function(&mut self) -> Result<Rc<Get_functionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Get_functionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 22, RULE_get_function);
        let mut _localctx: Rc<Get_functionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule tuple*/
                recog.base.set_state(155);
                recog.tuple()?;

                recog.base.set_state(156);
                recog.base.match_token(DOT, &mut recog.err_handler)?;

                recog.base.set_state(157);
                recog.base.match_token(GET, &mut recog.err_handler)?;

                recog.base.set_state(158);
                recog.base.match_token(LPAR, &mut recog.err_handler)?;

                /*InvokeRule right_expr*/
                recog.base.set_state(159);
                recog.right_expr()?;

                recog.base.set_state(160);
                recog.base.match_token(RPAR, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
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
//------------------- len_function ----------------
pub type Len_functionContextAll<'input> = Len_functionContext<'input>;

pub type Len_functionContext<'input> =
    BaseParserRuleContext<'input, Len_functionContextExt<'input>>;

#[derive(Clone)]
pub struct Len_functionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> lifParserContext<'input> for Len_functionContext<'input> {}

impl<'input, 'a> Listenable<dyn lifListener<'input> + 'a> for Len_functionContext<'input> {
    fn enter(&self, listener: &mut (dyn lifListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_len_function(self);
    }
}

impl<'input> CustomRuleContext<'input> for Len_functionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = lifParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_len_function
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_len_function }
}
antlr_rust::type_id! {Len_functionContextExt<'a>}

impl<'input> Len_functionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn lifParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Len_functionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Len_functionContextExt { ph: PhantomData },
        ))
    }
}

pub trait Len_functionContextAttrs<'input>:
    lifParserContext<'input> + BorrowMut<Len_functionContextExt<'input>>
{
    fn tuple(&self) -> Option<Rc<TupleContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token DOT
    /// Returns `None` if there is no child corresponding to token DOT
    fn DOT(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LEN
    /// Returns `None` if there is no child corresponding to token LEN
    fn LEN(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LEN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LPAR
    /// Returns `None` if there is no child corresponding to token LPAR
    fn LPAR(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LPAR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RPAR
    /// Returns `None` if there is no child corresponding to token RPAR
    fn RPAR(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RPAR, 0)
    }
}

impl<'input> Len_functionContextAttrs<'input> for Len_functionContext<'input> {}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn len_function(&mut self) -> Result<Rc<Len_functionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Len_functionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 24, RULE_len_function);
        let mut _localctx: Rc<Len_functionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule tuple*/
                recog.base.set_state(162);
                recog.tuple()?;

                recog.base.set_state(163);
                recog.base.match_token(DOT, &mut recog.err_handler)?;

                recog.base.set_state(164);
                recog.base.match_token(LEN, &mut recog.err_handler)?;

                recog.base.set_state(165);
                recog.base.match_token(LPAR, &mut recog.err_handler)?;

                recog.base.set_state(166);
                recog.base.match_token(RPAR, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
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
//------------------- assignation ----------------
pub type AssignationContextAll<'input> = AssignationContext<'input>;

pub type AssignationContext<'input> = BaseParserRuleContext<'input, AssignationContextExt<'input>>;

#[derive(Clone)]
pub struct AssignationContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> lifParserContext<'input> for AssignationContext<'input> {}

impl<'input, 'a> Listenable<dyn lifListener<'input> + 'a> for AssignationContext<'input> {
    fn enter(&self, listener: &mut (dyn lifListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_assignation(self);
    }
}

impl<'input> CustomRuleContext<'input> for AssignationContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = lifParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_assignation
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_assignation }
}
antlr_rust::type_id! {AssignationContextExt<'a>}

impl<'input> AssignationContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn lifParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<AssignationContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            AssignationContextExt { ph: PhantomData },
        ))
    }
}

pub trait AssignationContextAttrs<'input>:
    lifParserContext<'input> + BorrowMut<AssignationContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token VAR
    /// Returns `None` if there is no child corresponding to token VAR
    fn VAR(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(VAR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ID, 0)
    }
    /// Retrieves first TerminalNode corresponding to token EQUAL
    /// Returns `None` if there is no child corresponding to token EQUAL
    fn EQUAL(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EQUAL, 0)
    }
    fn init_var(&self) -> Option<Rc<Init_varContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn read(&self) -> Option<Rc<ReadContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn in_instr(&self) -> Option<Rc<In_instrContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn operation(&self) -> Option<Rc<OperationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> AssignationContextAttrs<'input> for AssignationContext<'input> {}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn assignation(&mut self) -> Result<Rc<AssignationContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = AssignationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 26, RULE_assignation);
        let mut _localctx: Rc<AssignationContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(168);
                recog.base.match_token(VAR, &mut recog.err_handler)?;

                recog.base.set_state(169);
                recog.base.match_token(ID, &mut recog.err_handler)?;

                recog.base.set_state(170);
                recog.base.match_token(EQUAL, &mut recog.err_handler)?;

                recog.base.set_state(175);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(11, &mut recog.base)? {
                    1 => {
                        {
                            /*InvokeRule init_var*/
                            recog.base.set_state(171);
                            recog.init_var()?;
                        }
                    }
                    2 => {
                        {
                            /*InvokeRule read*/
                            recog.base.set_state(172);
                            recog.read()?;
                        }
                    }
                    3 => {
                        {
                            /*InvokeRule in_instr*/
                            recog.base.set_state(173);
                            recog.in_instr()?;
                        }
                    }
                    4 => {
                        {
                            /*InvokeRule operation*/
                            recog.base.set_state(174);
                            recog.operation_rec(0)?;
                        }
                    }

                    _ => {}
                }
            }
        };
        match result {
            Ok(_) => {}
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

pub type AttributContext<'input> = BaseParserRuleContext<'input, AttributContextExt<'input>>;

#[derive(Clone)]
pub struct AttributContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> lifParserContext<'input> for AttributContext<'input> {}

impl<'input, 'a> Listenable<dyn lifListener<'input> + 'a> for AttributContext<'input> {
    fn enter(&self, listener: &mut (dyn lifListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_attribut(self);
    }
}

impl<'input> CustomRuleContext<'input> for AttributContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = lifParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_attribut
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_attribut }
}
antlr_rust::type_id! {AttributContextExt<'a>}

impl<'input> AttributContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn lifParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<AttributContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            AttributContextExt { ph: PhantomData },
        ))
    }
}

pub trait AttributContextAttrs<'input>:
    lifParserContext<'input> + BorrowMut<AttributContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token STRING
    /// Returns `None` if there is no child corresponding to token STRING
    fn STRING(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ID, 0)
    }
}

impl<'input> AttributContextAttrs<'input> for AttributContext<'input> {}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn attribut(&mut self) -> Result<Rc<AttributContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = AttributContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_attribut);
        let mut _localctx: Rc<AttributContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(177);
                _la = recog.base.input.la(1);
                if !(_la == ID || _la == STRING) {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
        };
        match result {
            Ok(_) => {}
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
//------------------- right_expr ----------------
pub type Right_exprContextAll<'input> = Right_exprContext<'input>;

pub type Right_exprContext<'input> = BaseParserRuleContext<'input, Right_exprContextExt<'input>>;

#[derive(Clone)]
pub struct Right_exprContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> lifParserContext<'input> for Right_exprContext<'input> {}

impl<'input, 'a> Listenable<dyn lifListener<'input> + 'a> for Right_exprContext<'input> {
    fn enter(&self, listener: &mut (dyn lifListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_right_expr(self);
    }
}

impl<'input> CustomRuleContext<'input> for Right_exprContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = lifParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_right_expr
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_right_expr }
}
antlr_rust::type_id! {Right_exprContextExt<'a>}

impl<'input> Right_exprContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn lifParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Right_exprContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Right_exprContextExt { ph: PhantomData },
        ))
    }
}

pub trait Right_exprContextAttrs<'input>:
    lifParserContext<'input> + BorrowMut<Right_exprContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ID, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NUMBER
    /// Returns `None` if there is no child corresponding to token NUMBER
    fn NUMBER(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NUMBER, 0)
    }
}

impl<'input> Right_exprContextAttrs<'input> for Right_exprContext<'input> {}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn right_expr(&mut self) -> Result<Rc<Right_exprContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Right_exprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 30, RULE_right_expr);
        let mut _localctx: Rc<Right_exprContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(179);
                _la = recog.base.input.la(1);
                if !(_la == ID || _la == NUMBER) {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
        };
        match result {
            Ok(_) => {}
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

pub type TupleContext<'input> = BaseParserRuleContext<'input, TupleContextExt<'input>>;

#[derive(Clone)]
pub struct TupleContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> lifParserContext<'input> for TupleContext<'input> {}

impl<'input, 'a> Listenable<dyn lifListener<'input> + 'a> for TupleContext<'input> {
    fn enter(&self, listener: &mut (dyn lifListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_tuple(self);
    }
}

impl<'input> CustomRuleContext<'input> for TupleContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = lifParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_tuple
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_tuple }
}
antlr_rust::type_id! {TupleContextExt<'a>}

impl<'input> TupleContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn lifParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<TupleContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            TupleContextExt { ph: PhantomData },
        ))
    }
}

pub trait TupleContextAttrs<'input>:
    lifParserContext<'input> + BorrowMut<TupleContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token LPAR
    /// Returns `None` if there is no child corresponding to token LPAR
    fn LPAR(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LPAR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RPAR
    /// Returns `None` if there is no child corresponding to token RPAR
    fn RPAR(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RPAR, 0)
    }
    fn tuple_content_all(&self) -> Vec<Rc<Tuple_contentContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn tuple_content(&self, i: usize) -> Option<Rc<Tuple_contentContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ID, 0)
    }
}

impl<'input> TupleContextAttrs<'input> for TupleContext<'input> {}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn tuple(&mut self) -> Result<Rc<TupleContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = TupleContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_tuple);
        let mut _localctx: Rc<TupleContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(193);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                LPAR => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(181);
                        recog.base.match_token(LPAR, &mut recog.err_handler)?;

                        {
                            /*InvokeRule tuple_content*/
                            recog.base.set_state(182);
                            recog.tuple_content()?;

                            recog.base.set_state(187);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            while _la == COMMA {
                                {
                                    {
                                        recog.base.set_state(183);
                                        recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                        /*InvokeRule tuple_content*/
                                        recog.base.set_state(184);
                                        recog.tuple_content()?;
                                    }
                                }
                                recog.base.set_state(189);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                            }
                        }
                        recog.base.set_state(190);
                        recog.base.match_token(RPAR, &mut recog.err_handler)?;
                    }
                }

                ID => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(192);
                        recog.base.match_token(ID, &mut recog.err_handler)?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
        };
        match result {
            Ok(_) => {}
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

pub type Tuple_contentContext<'input> =
    BaseParserRuleContext<'input, Tuple_contentContextExt<'input>>;

#[derive(Clone)]
pub struct Tuple_contentContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> lifParserContext<'input> for Tuple_contentContext<'input> {}

impl<'input, 'a> Listenable<dyn lifListener<'input> + 'a> for Tuple_contentContext<'input> {
    fn enter(&self, listener: &mut (dyn lifListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_tuple_content(self);
    }
}

impl<'input> CustomRuleContext<'input> for Tuple_contentContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = lifParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_tuple_content
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_tuple_content }
}
antlr_rust::type_id! {Tuple_contentContextExt<'a>}

impl<'input> Tuple_contentContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn lifParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Tuple_contentContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Tuple_contentContextExt { ph: PhantomData },
        ))
    }
}

pub trait Tuple_contentContextAttrs<'input>:
    lifParserContext<'input> + BorrowMut<Tuple_contentContextExt<'input>>
{
    fn init_var(&self) -> Option<Rc<Init_varContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token WILDCARD
    /// Returns `None` if there is no child corresponding to token WILDCARD
    fn WILDCARD(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(WILDCARD, 0)
    }
}

impl<'input> Tuple_contentContextAttrs<'input> for Tuple_contentContext<'input> {}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn tuple_content(&mut self) -> Result<Rc<Tuple_contentContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Tuple_contentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 34, RULE_tuple_content);
        let mut _localctx: Rc<Tuple_contentContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(197);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                LPAR | ID | NUMBER | STRING | CHARACTER => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule init_var*/
                        recog.base.set_state(195);
                        recog.init_var()?;
                    }
                }

                WILDCARD => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(196);
                        recog.base.match_token(WILDCARD, &mut recog.err_handler)?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
        };
        match result {
            Ok(_) => {}
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
//------------------- tuple_space_name ----------------
pub type Tuple_space_nameContextAll<'input> = Tuple_space_nameContext<'input>;

pub type Tuple_space_nameContext<'input> =
    BaseParserRuleContext<'input, Tuple_space_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Tuple_space_nameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> lifParserContext<'input> for Tuple_space_nameContext<'input> {}

impl<'input, 'a> Listenable<dyn lifListener<'input> + 'a> for Tuple_space_nameContext<'input> {
    fn enter(&self, listener: &mut (dyn lifListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_tuple_space_name(self);
    }
}

impl<'input> CustomRuleContext<'input> for Tuple_space_nameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = lifParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_tuple_space_name
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_tuple_space_name }
}
antlr_rust::type_id! {Tuple_space_nameContextExt<'a>}

impl<'input> Tuple_space_nameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn lifParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Tuple_space_nameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Tuple_space_nameContextExt { ph: PhantomData },
        ))
    }
}

pub trait Tuple_space_nameContextAttrs<'input>:
    lifParserContext<'input> + BorrowMut<Tuple_space_nameContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token STRING
    /// Returns `None` if there is no child corresponding to token STRING
    fn STRING(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ID, 0)
    }
}

impl<'input> Tuple_space_nameContextAttrs<'input> for Tuple_space_nameContext<'input> {}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn tuple_space_name(
        &mut self,
    ) -> Result<Rc<Tuple_space_nameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Tuple_space_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 36, RULE_tuple_space_name);
        let mut _localctx: Rc<Tuple_space_nameContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(199);
                _la = recog.base.input.la(1);
                if !(_la == ID || _la == STRING) {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
        };
        match result {
            Ok(_) => {}
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
//------------------- init_var ----------------
pub type Init_varContextAll<'input> = Init_varContext<'input>;

pub type Init_varContext<'input> = BaseParserRuleContext<'input, Init_varContextExt<'input>>;

#[derive(Clone)]
pub struct Init_varContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> lifParserContext<'input> for Init_varContext<'input> {}

impl<'input, 'a> Listenable<dyn lifListener<'input> + 'a> for Init_varContext<'input> {
    fn enter(&self, listener: &mut (dyn lifListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_init_var(self);
    }
}

impl<'input> CustomRuleContext<'input> for Init_varContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = lifParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_init_var
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_init_var }
}
antlr_rust::type_id! {Init_varContextExt<'a>}

impl<'input> Init_varContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn lifParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Init_varContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Init_varContextExt { ph: PhantomData },
        ))
    }
}

pub trait Init_varContextAttrs<'input>:
    lifParserContext<'input> + BorrowMut<Init_varContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token NUMBER
    /// Returns `None` if there is no child corresponding to token NUMBER
    fn NUMBER(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NUMBER, 0)
    }
    /// Retrieves first TerminalNode corresponding to token STRING
    /// Returns `None` if there is no child corresponding to token STRING
    fn STRING(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING, 0)
    }
    /// Retrieves first TerminalNode corresponding to token CHARACTER
    /// Returns `None` if there is no child corresponding to token CHARACTER
    fn CHARACTER(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CHARACTER, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ID, 0)
    }
    fn tuple(&self) -> Option<Rc<TupleContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Init_varContextAttrs<'input> for Init_varContext<'input> {}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn init_var(&mut self) -> Result<Rc<Init_varContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Init_varContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_init_var);
        let mut _localctx: Rc<Init_varContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(206);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(15, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(201);
                        recog.base.match_token(NUMBER, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(202);
                        recog.base.match_token(STRING, &mut recog.err_handler)?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        recog.base.set_state(203);
                        recog.base.match_token(CHARACTER, &mut recog.err_handler)?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        recog.base.set_state(204);
                        recog.base.match_token(ID, &mut recog.err_handler)?;
                    }
                }
                5 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        /*InvokeRule tuple*/
                        recog.base.set_state(205);
                        recog.tuple()?;
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
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

pub type ProtocolContext<'input> = BaseParserRuleContext<'input, ProtocolContextExt<'input>>;

#[derive(Clone)]
pub struct ProtocolContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> lifParserContext<'input> for ProtocolContext<'input> {}

impl<'input, 'a> Listenable<dyn lifListener<'input> + 'a> for ProtocolContext<'input> {
    fn enter(&self, listener: &mut (dyn lifListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_protocol(self);
    }
}

impl<'input> CustomRuleContext<'input> for ProtocolContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = lifParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_protocol
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_protocol }
}
antlr_rust::type_id! {ProtocolContextExt<'a>}

impl<'input> ProtocolContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn lifParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ProtocolContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ProtocolContextExt { ph: PhantomData },
        ))
    }
}

pub trait ProtocolContextAttrs<'input>:
    lifParserContext<'input> + BorrowMut<ProtocolContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token UDP
    /// Returns `None` if there is no child corresponding to token UDP
    fn UDP(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(UDP, 0)
    }
    /// Retrieves first TerminalNode corresponding to token TCP
    /// Returns `None` if there is no child corresponding to token TCP
    fn TCP(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(TCP, 0)
    }
}

impl<'input> ProtocolContextAttrs<'input> for ProtocolContext<'input> {}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn protocol(&mut self) -> Result<Rc<ProtocolContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ProtocolContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_protocol);
        let mut _localctx: Rc<ProtocolContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(208);
                _la = recog.base.input.la(1);
                if !(_la == TCP || _la == UDP) {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
        };
        match result {
            Ok(_) => {}
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
//------------------- ip_address ----------------
pub type Ip_addressContextAll<'input> = Ip_addressContext<'input>;

pub type Ip_addressContext<'input> = BaseParserRuleContext<'input, Ip_addressContextExt<'input>>;

#[derive(Clone)]
pub struct Ip_addressContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> lifParserContext<'input> for Ip_addressContext<'input> {}

impl<'input, 'a> Listenable<dyn lifListener<'input> + 'a> for Ip_addressContext<'input> {
    fn enter(&self, listener: &mut (dyn lifListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_ip_address(self);
    }
}

impl<'input> CustomRuleContext<'input> for Ip_addressContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = lifParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_ip_address
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_ip_address }
}
antlr_rust::type_id! {Ip_addressContextExt<'a>}

impl<'input> Ip_addressContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn lifParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Ip_addressContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Ip_addressContextExt { ph: PhantomData },
        ))
    }
}

pub trait Ip_addressContextAttrs<'input>:
    lifParserContext<'input> + BorrowMut<Ip_addressContextExt<'input>>
{
    /// Retrieves all `TerminalNode`s corresponding to token NUMBER in current rule
    fn NUMBER_all(&self) -> Vec<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token NUMBER, starting from 0.
    /// Returns `None` if number of children corresponding to token NUMBER is less or equal than `i`.
    fn NUMBER(&self, i: usize) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NUMBER, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
    fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
    /// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
    fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, i)
    }
}

impl<'input> Ip_addressContextAttrs<'input> for Ip_addressContext<'input> {}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn ip_address(&mut self) -> Result<Rc<Ip_addressContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Ip_addressContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 42, RULE_ip_address);
        let mut _localctx: Rc<Ip_addressContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(210);
                recog.base.match_token(NUMBER, &mut recog.err_handler)?;

                recog.base.set_state(211);
                recog.base.match_token(DOT, &mut recog.err_handler)?;

                recog.base.set_state(212);
                recog.base.match_token(NUMBER, &mut recog.err_handler)?;

                recog.base.set_state(213);
                recog.base.match_token(DOT, &mut recog.err_handler)?;

                recog.base.set_state(214);
                recog.base.match_token(NUMBER, &mut recog.err_handler)?;

                recog.base.set_state(215);
                recog.base.match_token(DOT, &mut recog.err_handler)?;

                recog.base.set_state(216);
                recog.base.match_token(NUMBER, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
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

pub type PortContext<'input> = BaseParserRuleContext<'input, PortContextExt<'input>>;

#[derive(Clone)]
pub struct PortContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> lifParserContext<'input> for PortContext<'input> {}

impl<'input, 'a> Listenable<dyn lifListener<'input> + 'a> for PortContext<'input> {
    fn enter(&self, listener: &mut (dyn lifListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_port(self);
    }
}

impl<'input> CustomRuleContext<'input> for PortContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = lifParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_port
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_port }
}
antlr_rust::type_id! {PortContextExt<'a>}

impl<'input> PortContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn lifParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<PortContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            PortContextExt { ph: PhantomData },
        ))
    }
}

pub trait PortContextAttrs<'input>:
    lifParserContext<'input> + BorrowMut<PortContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token NUMBER
    /// Returns `None` if there is no child corresponding to token NUMBER
    fn NUMBER(&self) -> Option<Rc<TerminalNode<'input, lifParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NUMBER, 0)
    }
}

impl<'input> PortContextAttrs<'input> for PortContext<'input> {}

impl<'input, I, H> lifParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn port(&mut self) -> Result<Rc<PortContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = PortContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_port);
        let mut _localctx: Rc<PortContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(218);
                recog.base.match_token(NUMBER, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
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
            dfa.push(DFA::new(_ATN.clone(), _ATN.get_decision_state(i), i as isize).into())
        }
        Arc::new(dfa)
    };
}

const _serializedATN: &'static str =
    "\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x2b\u{df}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\x0a\
	\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\x0e\
	\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\x13\
	\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\x17\
	\x04\x18\x09\x18\x03\x02\x07\x02\x32\x0a\x02\x0c\x02\x0e\x02\x35\x0b\x02\
	\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x05\x03\x3e\x0a\
	\x03\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x05\x03\
	\x05\x03\x05\x03\x05\x07\x05\x4b\x0a\x05\x0c\x05\x0e\x05\x4e\x0b\x05\x03\
	\x06\x03\x06\x03\x06\x03\x06\x03\x07\x03\x07\x03\x07\x07\x07\x57\x0a\x07\
	\x0c\x07\x0e\x07\x5a\x0b\x07\x03\x08\x03\x08\x03\x08\x03\x08\x07\x08\x60\
	\x0a\x08\x0c\x08\x0e\x08\x63\x0b\x08\x03\x09\x03\x09\x03\x09\x03\x09\x07\
	\x09\x69\x0a\x09\x0c\x09\x0e\x09\x6c\x0b\x09\x03\x0a\x03\x0a\x03\x0a\x03\
	\x0a\x07\x0a\x72\x0a\x0a\x0c\x0a\x0e\x0a\x75\x0b\x0a\x03\x0b\x03\x0b\x03\
	\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x06\x0b\u{81}\
	\x0a\x0b\x0d\x0b\x0e\x0b\u{82}\x03\x0b\x03\x0b\x03\x0c\x03\x0c\x03\x0c\x03\
	\x0c\x05\x0c\u{8b}\x0a\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\
	\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x07\x0c\u{99}\x0a\x0c\x0c\
	\x0c\x0e\x0c\u{9c}\x0b\x0c\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\
	\x03\x0d\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0f\x03\x0f\
	\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x05\x0f\u{b2}\x0a\x0f\x03\x10\x03\
	\x10\x03\x11\x03\x11\x03\x12\x03\x12\x03\x12\x03\x12\x07\x12\u{bc}\x0a\x12\
	\x0c\x12\x0e\x12\u{bf}\x0b\x12\x03\x12\x03\x12\x03\x12\x05\x12\u{c4}\x0a\
	\x12\x03\x13\x03\x13\x05\x13\u{c8}\x0a\x13\x03\x14\x03\x14\x03\x15\x03\x15\
	\x03\x15\x03\x15\x03\x15\x05\x15\u{d1}\x0a\x15\x03\x16\x03\x16\x03\x17\x03\
	\x17\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\x03\x18\x03\x18\x03\
	\x18\x02\x03\x16\x19\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x1a\
	\x1c\x1e\x20\x22\x24\x26\x28\x2a\x2c\x2e\x02\x05\x04\x02\x24\x24\x26\x26\
	\x03\x02\x24\x25\x03\x02\x0a\x0b\x02\u{e4}\x02\x33\x03\x02\x02\x02\x04\x3d\
	\x03\x02\x02\x02\x06\x3f\x03\x02\x02\x02\x08\x46\x03\x02\x02\x02\x0a\x4f\
	\x03\x02\x02\x02\x0c\x53\x03\x02\x02\x02\x0e\x5b\x03\x02\x02\x02\x10\x64\
	\x03\x02\x02\x02\x12\x6d\x03\x02\x02\x02\x14\x76\x03\x02\x02\x02\x16\u{8a}\
	\x03\x02\x02\x02\x18\u{9d}\x03\x02\x02\x02\x1a\u{a4}\x03\x02\x02\x02\x1c\
	\u{aa}\x03\x02\x02\x02\x1e\u{b3}\x03\x02\x02\x02\x20\u{b5}\x03\x02\x02\x02\
	\x22\u{c3}\x03\x02\x02\x02\x24\u{c7}\x03\x02\x02\x02\x26\u{c9}\x03\x02\x02\
	\x02\x28\u{d0}\x03\x02\x02\x02\x2a\u{d2}\x03\x02\x02\x02\x2c\u{d4}\x03\x02\
	\x02\x02\x2e\u{dc}\x03\x02\x02\x02\x30\x32\x05\x04\x03\x02\x31\x30\x03\x02\
	\x02\x02\x32\x35\x03\x02\x02\x02\x33\x31\x03\x02\x02\x02\x33\x34\x03\x02\
	\x02\x02\x34\x03\x03\x02\x02\x02\x35\x33\x03\x02\x02\x02\x36\x3e\x05\x06\
	\x04\x02\x37\x3e\x05\x08\x05\x02\x38\x3e\x05\x0a\x06\x02\x39\x3e\x05\x0c\
	\x07\x02\x3a\x3e\x05\x12\x0a\x02\x3b\x3e\x05\x14\x0b\x02\x3c\x3e\x05\x1c\
	\x0f\x02\x3d\x36\x03\x02\x02\x02\x3d\x37\x03\x02\x02\x02\x3d\x38\x03\x02\
	\x02\x02\x3d\x39\x03\x02\x02\x02\x3d\x3a\x03\x02\x02\x02\x3d\x3b\x03\x02\
	\x02\x02\x3d\x3c\x03\x02\x02\x02\x3e\x05\x03\x02\x02\x02\x3f\x40\x07\x03\
	\x02\x02\x40\x41\x05\x2a\x16\x02\x41\x42\x07\x1b\x02\x02\x42\x43\x05\x2c\
	\x17\x02\x43\x44\x07\x1b\x02\x02\x44\x45\x05\x2e\x18\x02\x45\x07\x03\x02\
	\x02\x02\x46\x47\x07\x05\x02\x02\x47\x48\x05\x1e\x10\x02\x48\x4c\x05\x26\
	\x14\x02\x49\x4b\x05\x1e\x10\x02\x4a\x49\x03\x02\x02\x02\x4b\x4e\x03\x02\
	\x02\x02\x4c\x4a\x03\x02\x02\x02\x4c\x4d\x03\x02\x02\x02\x4d\x09\x03\x02\
	\x02\x02\x4e\x4c\x03\x02\x02\x02\x4f\x50\x07\x06\x02\x02\x50\x51\x05\x1e\
	\x10\x02\x51\x52\x05\x26\x14\x02\x52\x0b\x03\x02\x02\x02\x53\x54\x07\x04\
	\x02\x02\x54\x58\x05\x26\x14\x02\x55\x57\x05\x1e\x10\x02\x56\x55\x03\x02\
	\x02\x02\x57\x5a\x03\x02\x02\x02\x58\x56\x03\x02\x02\x02\x58\x59\x03\x02\
	\x02\x02\x59\x0d\x03\x02\x02\x02\x5a\x58\x03\x02\x02\x02\x5b\x5c\x07\x08\
	\x02\x02\x5c\x61\x05\x22\x12\x02\x5d\x5e\x07\x13\x02\x02\x5e\x60\x05\x22\
	\x12\x02\x5f\x5d\x03\x02\x02\x02\x60\x63\x03\x02\x02\x02\x61\x5f\x03\x02\
	\x02\x02\x61\x62\x03\x02\x02\x02\x62\x0f\x03\x02\x02\x02\x63\x61\x03\x02\
	\x02\x02\x64\x65\x07\x09\x02\x02\x65\x6a\x05\x22\x12\x02\x66\x67\x07\x13\
	\x02\x02\x67\x69\x05\x22\x12\x02\x68\x66\x03\x02\x02\x02\x69\x6c\x03\x02\
	\x02\x02\x6a\x68\x03\x02\x02\x02\x6a\x6b\x03\x02\x02\x02\x6b\x11\x03\x02\
	\x02\x02\x6c\x6a\x03\x02\x02\x02\x6d\x6e\x07\x07\x02\x02\x6e\x73\x05\x22\
	\x12\x02\x6f\x70\x07\x13\x02\x02\x70\x72\x05\x22\x12\x02\x71\x6f\x03\x02\
	\x02\x02\x72\x75\x03\x02\x02\x02\x73\x71\x03\x02\x02\x02\x73\x74\x03\x02\
	\x02\x02\x74\x13\x03\x02\x02\x02\x75\x73\x03\x02\x02\x02\x76\x77\x07\x0f\
	\x02\x02\x77\x78\x07\x24\x02\x02\x78\x79\x07\x1f\x02\x02\x79\x7a\x05\x16\
	\x0c\x02\x7a\x7b\x07\x10\x02\x02\x7b\x7c\x07\x11\x02\x02\x7c\x7d\x05\x16\
	\x0c\x02\x7d\x7e\x07\x12\x02\x02\x7e\u{80}\x07\x23\x02\x02\x7f\u{81}\x05\
	\x04\x03\x02\u{80}\x7f\x03\x02\x02\x02\u{81}\u{82}\x03\x02\x02\x02\u{82}\
	\u{80}\x03\x02\x02\x02\u{82}\u{83}\x03\x02\x02\x02\u{83}\u{84}\x03\x02\x02\
	\x02\u{84}\u{85}\x07\x22\x02\x02\u{85}\x15\x03\x02\x02\x02\u{86}\u{87}\x08\
	\x0c\x01\x02\u{87}\u{8b}\x05\x18\x0d\x02\u{88}\u{8b}\x05\x1a\x0e\x02\u{89}\
	\u{8b}\x05\x20\x11\x02\u{8a}\u{86}\x03\x02\x02\x02\u{8a}\u{88}\x03\x02\x02\
	\x02\u{8a}\u{89}\x03\x02\x02\x02\u{8b}\u{9a}\x03\x02\x02\x02\u{8c}\u{8d}\
	\x0c\x06\x02\x02\u{8d}\u{8e}\x07\x20\x02\x02\u{8e}\u{99}\x05\x16\x0c\x07\
	\u{8f}\u{90}\x0c\x05\x02\x02\u{90}\u{91}\x07\x21\x02\x02\u{91}\u{99}\x05\
	\x16\x0c\x06\u{92}\u{93}\x0c\x04\x02\x02\u{93}\u{94}\x07\x1d\x02\x02\u{94}\
	\u{99}\x05\x16\x0c\x05\u{95}\u{96}\x0c\x03\x02\x02\u{96}\u{97}\x07\x16\x02\
	\x02\u{97}\u{99}\x05\x16\x0c\x04\u{98}\u{8c}\x03\x02\x02\x02\u{98}\u{8f}\
	\x03\x02\x02\x02\u{98}\u{92}\x03\x02\x02\x02\u{98}\u{95}\x03\x02\x02\x02\
	\u{99}\u{9c}\x03\x02\x02\x02\u{9a}\u{98}\x03\x02\x02\x02\u{9a}\u{9b}\x03\
	\x02\x02\x02\u{9b}\x17\x03\x02\x02\x02\u{9c}\u{9a}\x03\x02\x02\x02\u{9d}\
	\u{9e}\x05\x22\x12\x02\u{9e}\u{9f}\x07\x1a\x02\x02\u{9f}\u{a0}\x07\x0d\x02\
	\x02\u{a0}\u{a1}\x07\x11\x02\x02\u{a1}\u{a2}\x05\x20\x11\x02\u{a2}\u{a3}\
	\x07\x12\x02\x02\u{a3}\x19\x03\x02\x02\x02\u{a4}\u{a5}\x05\x22\x12\x02\u{a5}\
	\u{a6}\x07\x1a\x02\x02\u{a6}\u{a7}\x07\x0e\x02\x02\u{a7}\u{a8}\x07\x11\x02\
	\x02\u{a8}\u{a9}\x07\x12\x02\x02\u{a9}\x1b\x03\x02\x02\x02\u{aa}\u{ab}\x07\
	\x0c\x02\x02\u{ab}\u{ac}\x07\x24\x02\x02\u{ac}\u{b1}\x07\x1f\x02\x02\u{ad}\
	\u{b2}\x05\x28\x15\x02\u{ae}\u{b2}\x05\x0e\x08\x02\u{af}\u{b2}\x05\x10\x09\
	\x02\u{b0}\u{b2}\x05\x16\x0c\x02\u{b1}\u{ad}\x03\x02\x02\x02\u{b1}\u{ae}\
	\x03\x02\x02\x02\u{b1}\u{af}\x03\x02\x02\x02\u{b1}\u{b0}\x03\x02\x02\x02\
	\u{b2}\x1d\x03\x02\x02\x02\u{b3}\u{b4}\x09\x02\x02\x02\u{b4}\x1f\x03\x02\
	\x02\x02\u{b5}\u{b6}\x09\x03\x02\x02\u{b6}\x21\x03\x02\x02\x02\u{b7}\u{b8}\
	\x07\x11\x02\x02\u{b8}\u{bd}\x05\x24\x13\x02\u{b9}\u{ba}\x07\x13\x02\x02\
	\u{ba}\u{bc}\x05\x24\x13\x02\u{bb}\u{b9}\x03\x02\x02\x02\u{bc}\u{bf}\x03\
	\x02\x02\x02\u{bd}\u{bb}\x03\x02\x02\x02\u{bd}\u{be}\x03\x02\x02\x02\u{be}\
	\u{c0}\x03\x02\x02\x02\u{bf}\u{bd}\x03\x02\x02\x02\u{c0}\u{c1}\x07\x12\x02\
	\x02\u{c1}\u{c4}\x03\x02\x02\x02\u{c2}\u{c4}\x07\x24\x02\x02\u{c3}\u{b7}\
	\x03\x02\x02\x02\u{c3}\u{c2}\x03\x02\x02\x02\u{c4}\x23\x03\x02\x02\x02\u{c5}\
	\u{c8}\x05\x28\x15\x02\u{c6}\u{c8}\x07\x1e\x02\x02\u{c7}\u{c5}\x03\x02\x02\
	\x02\u{c7}\u{c6}\x03\x02\x02\x02\u{c8}\x25\x03\x02\x02\x02\u{c9}\u{ca}\x09\
	\x02\x02\x02\u{ca}\x27\x03\x02\x02\x02\u{cb}\u{d1}\x07\x25\x02\x02\u{cc}\
	\u{d1}\x07\x26\x02\x02\u{cd}\u{d1}\x07\x27\x02\x02\u{ce}\u{d1}\x07\x24\x02\
	\x02\u{cf}\u{d1}\x05\x22\x12\x02\u{d0}\u{cb}\x03\x02\x02\x02\u{d0}\u{cc}\
	\x03\x02\x02\x02\u{d0}\u{cd}\x03\x02\x02\x02\u{d0}\u{ce}\x03\x02\x02\x02\
	\u{d0}\u{cf}\x03\x02\x02\x02\u{d1}\x29\x03\x02\x02\x02\u{d2}\u{d3}\x09\x04\
	\x02\x02\u{d3}\x2b\x03\x02\x02\x02\u{d4}\u{d5}\x07\x25\x02\x02\u{d5}\u{d6}\
	\x07\x1a\x02\x02\u{d6}\u{d7}\x07\x25\x02\x02\u{d7}\u{d8}\x07\x1a\x02\x02\
	\u{d8}\u{d9}\x07\x25\x02\x02\u{d9}\u{da}\x07\x1a\x02\x02\u{da}\u{db}\x07\
	\x25\x02\x02\u{db}\x2d\x03\x02\x02\x02\u{dc}\u{dd}\x07\x25\x02\x02\u{dd}\
	\x2f\x03\x02\x02\x02\x12\x33\x3d\x4c\x58\x61\x6a\x73\u{82}\u{8a}\u{98}\u{9a}\
	\u{b1}\u{bd}\u{c3}\u{c7}\u{d0}";
