// Generated from .\lif.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use antlr_rust::atn::ATN;
use antlr_rust::char_stream::CharStream;
use antlr_rust::int_stream::IntStream;
use antlr_rust::lexer::{BaseLexer, Lexer, LexerRecog};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::lexer_atn_simulator::{LexerATNSimulator, ILexerATNSimulator};
use antlr_rust::PredictionContextCache;
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::error_listener::ErrorListener;
use antlr_rust::TokenSource;
use antlr_rust::token_factory::{TokenFactory,CommonTokenFactory,TokenAware};
use antlr_rust::token::*;
use antlr_rust::rule_context::{BaseRuleContext,EmptyCustomRuleContext,EmptyContext};
use antlr_rust::parser_rule_context::{ParserRuleContext,BaseParserRuleContext,cast};
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};

use antlr_rust::{lazy_static,Tid,TidAble,TidExt};

use std::sync::Arc;
use std::cell::RefCell;
use std::rc::Rc;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};


	pub const CONNECT:isize=1; 
	pub const ATTACH:isize=2; 
	pub const CREATE:isize=3; 
	pub const DELETE:isize=4; 
	pub const OUT:isize=5; 
	pub const READ:isize=6; 
	pub const IN:isize=7; 
	pub const TCP:isize=8; 
	pub const UDP:isize=9; 
	pub const VAR:isize=10; 
	pub const GET:isize=11; 
	pub const LEN:isize=12; 
	pub const LPAR:isize=13; 
	pub const RPAR:isize=14; 
	pub const COMMA:isize=15; 
	pub const DOUBLEQUOTE:isize=16; 
	pub const QUOTE:isize=17; 
	pub const SLASH:isize=18; 
	pub const BACKSLASH:isize=19; 
	pub const LBRACKET:isize=20; 
	pub const RBRACKET:isize=21; 
	pub const DOT:isize=22; 
	pub const DOUBLEDOT:isize=23; 
	pub const SEMICOLON:isize=24; 
	pub const KLEENE:isize=25; 
	pub const WILDCARD:isize=26; 
	pub const EQUAL:isize=27; 
	pub const PLUS:isize=28; 
	pub const MINUS:isize=29; 
	pub const ID:isize=30; 
	pub const NUMBER:isize=31; 
	pub const STRING:isize=32; 
	pub const CHARACTER:isize=33; 
	pub const LINECOMMENT:isize=34; 
	pub const COMMENT:isize=35; 
	pub const NEWLINE:isize=36; 
	pub const WS:isize=37;
	pub const channelNames: [&'static str;0+2] = [
		"DEFAULT_TOKEN_CHANNEL", "HIDDEN"
	];

	pub const modeNames: [&'static str;1] = [
		"DEFAULT_MODE"
	];

	pub const ruleNames: [&'static str;39] = [
		"CONNECT", "ATTACH", "CREATE", "DELETE", "OUT", "READ", "IN", "TCP", "UDP", 
		"VAR", "GET", "LEN", "LPAR", "RPAR", "COMMA", "DOUBLEQUOTE", "QUOTE", 
		"SLASH", "BACKSLASH", "LBRACKET", "RBRACKET", "DOT", "DOUBLEDOT", "SEMICOLON", 
		"KLEENE", "WILDCARD", "EQUAL", "PLUS", "MINUS", "ID", "LETTER", "DIGIT", 
		"NUMBER", "STRING", "CHARACTER", "LINECOMMENT", "COMMENT", "NEWLINE", 
		"WS"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;30] = [
		None, Some("'connect'"), Some("'attach'"), Some("'create'"), Some("'delete'"), 
		Some("'out'"), Some("'read'"), Some("'in'"), Some("'tcp'"), Some("'udp'"), 
		Some("'var'"), Some("'get'"), Some("'len'"), Some("'('"), Some("')'"), 
		Some("','"), Some("'\"'"), Some("'''"), Some("'/'"), Some("'\\'"), Some("'['"), 
		Some("']'"), Some("'.'"), Some("':'"), Some("';'"), Some("'*'"), Some("'_'"), 
		Some("'='"), Some("'+'"), Some("'-'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;38]  = [
		None, Some("CONNECT"), Some("ATTACH"), Some("CREATE"), Some("DELETE"), 
		Some("OUT"), Some("READ"), Some("IN"), Some("TCP"), Some("UDP"), Some("VAR"), 
		Some("GET"), Some("LEN"), Some("LPAR"), Some("RPAR"), Some("COMMA"), Some("DOUBLEQUOTE"), 
		Some("QUOTE"), Some("SLASH"), Some("BACKSLASH"), Some("LBRACKET"), Some("RBRACKET"), 
		Some("DOT"), Some("DOUBLEDOT"), Some("SEMICOLON"), Some("KLEENE"), Some("WILDCARD"), 
		Some("EQUAL"), Some("PLUS"), Some("MINUS"), Some("ID"), Some("NUMBER"), 
		Some("STRING"), Some("CHARACTER"), Some("LINECOMMENT"), Some("COMMENT"), 
		Some("NEWLINE"), Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


pub type LexerContext<'input> = BaseRuleContext<'input,EmptyCustomRuleContext<'input,LocalTokenFactory<'input> >>;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

type From<'a> = <LocalTokenFactory<'a> as TokenFactory<'a> >::From;

#[derive(Tid)]
pub struct lifLexer<'input, Input:CharStream<From<'input> >> {
	base: BaseLexer<'input,lifLexerActions,Input,LocalTokenFactory<'input>>,
}

impl<'input, Input:CharStream<From<'input> >> Deref for lifLexer<'input,Input>{
	type Target = BaseLexer<'input,lifLexerActions,Input,LocalTokenFactory<'input>>;

	fn deref(&self) -> &Self::Target {
		&self.base
	}
}

impl<'input, Input:CharStream<From<'input> >> DerefMut for lifLexer<'input,Input>{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.base
	}
}


impl<'input, Input:CharStream<From<'input> >> lifLexer<'input,Input>{
    fn get_rule_names(&self) -> &'static [&'static str] {
        &ruleNames
    }
    fn get_literal_names(&self) -> &[Option<&str>] {
        &_LITERAL_NAMES
    }

    fn get_symbolic_names(&self) -> &[Option<&str>] {
        &_SYMBOLIC_NAMES
    }

    fn get_grammar_file_name(&self) -> &'static str {
        "lifLexer.g4"
    }

	pub fn new_with_token_factory(input: Input, tf: &'input LocalTokenFactory<'input>) -> Self {
		antlr_rust::recognizer::check_version("0","2");
    	Self {
			base: BaseLexer::new_base_lexer(
				input,
				LexerATNSimulator::new_lexer_atnsimulator(
					_ATN.clone(),
					_decision_to_DFA.clone(),
					_shared_context_cache.clone(),
				),
				lifLexerActions{},
				tf
			)
	    }
	}
}

impl<'input, Input:CharStream<From<'input> >> lifLexer<'input,Input> where &'input LocalTokenFactory<'input>:Default{
	pub fn new(input: Input) -> Self{
		lifLexer::new_with_token_factory(input, <&LocalTokenFactory<'input> as Default>::default())
	}
}

pub struct lifLexerActions {
}

impl lifLexerActions{
}

impl<'input, Input:CharStream<From<'input> >> Actions<'input,BaseLexer<'input,lifLexerActions,Input,LocalTokenFactory<'input>>> for lifLexerActions{
	}

	impl<'input, Input:CharStream<From<'input> >> lifLexer<'input,Input>{

}

impl<'input, Input:CharStream<From<'input> >> LexerRecog<'input,BaseLexer<'input,lifLexerActions,Input,LocalTokenFactory<'input>>> for lifLexerActions{
}
impl<'input> TokenAware<'input> for lifLexerActions{
	type TF = LocalTokenFactory<'input>;
}

impl<'input, Input:CharStream<From<'input> >> TokenSource<'input> for lifLexer<'input,Input>{
	type TF = LocalTokenFactory<'input>;

    fn next_token(&mut self) -> <Self::TF as TokenFactory<'input>>::Tok {
        self.base.next_token()
    }

    fn get_line(&self) -> isize {
        self.base.get_line()
    }

    fn get_char_position_in_line(&self) -> isize {
        self.base.get_char_position_in_line()
    }

    fn get_input_stream(&mut self) -> Option<&mut dyn IntStream> {
        self.base.get_input_stream()
    }

	fn get_source_name(&self) -> String {
		self.base.get_source_name()
	}

    fn get_token_factory(&self) -> &'input Self::TF {
        self.base.get_token_factory()
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
		"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x02\
		\x27\u{103}\x08\x01\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\
		\x05\x09\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\
		\x09\x04\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\
		\x0e\x09\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\
		\x12\x04\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\
		\x17\x09\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\
		\x1b\x04\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\
		\x20\x09\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\
		\x24\x04\x25\x09\x25\x04\x26\x09\x26\x04\x27\x09\x27\x04\x28\x09\x28\x03\
		\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x03\x03\
		\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x04\x03\x04\x03\x04\x03\
		\x04\x03\x04\x03\x04\x03\x04\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\
		\x05\x03\x05\x03\x06\x03\x06\x03\x06\x03\x06\x03\x07\x03\x07\x03\x07\x03\
		\x07\x03\x07\x03\x08\x03\x08\x03\x08\x03\x09\x03\x09\x03\x09\x03\x09\x03\
		\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0c\x03\
		\x0c\x03\x0c\x03\x0c\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0e\x03\x0e\x03\
		\x0f\x03\x0f\x03\x10\x03\x10\x03\x11\x03\x11\x03\x12\x03\x12\x03\x13\x03\
		\x13\x03\x14\x03\x14\x03\x15\x03\x15\x03\x16\x03\x16\x03\x17\x03\x17\x03\
		\x18\x03\x18\x03\x19\x03\x19\x03\x1a\x03\x1a\x03\x1b\x03\x1b\x03\x1c\x03\
		\x1c\x03\x1d\x03\x1d\x03\x1e\x03\x1e\x03\x1f\x03\x1f\x03\x1f\x07\x1f\u{b4}\
		\x0a\x1f\x0c\x1f\x0e\x1f\u{b7}\x0b\x1f\x03\x20\x03\x20\x03\x21\x03\x21\
		\x03\x22\x06\x22\u{be}\x0a\x22\x0d\x22\x0e\x22\u{bf}\x03\x23\x03\x23\x03\
		\x23\x03\x23\x03\x23\x03\x23\x03\x23\x06\x23\u{c9}\x0a\x23\x0d\x23\x0e\
		\x23\u{ca}\x03\x23\x03\x23\x03\x24\x03\x24\x03\x24\x03\x24\x03\x24\x03\
		\x24\x03\x24\x03\x24\x05\x24\u{d7}\x0a\x24\x03\x24\x03\x24\x03\x25\x03\
		\x25\x03\x25\x07\x25\u{de}\x0a\x25\x0c\x25\x0e\x25\u{e1}\x0b\x25\x03\x25\
		\x03\x25\x05\x25\u{e5}\x0a\x25\x03\x25\x03\x25\x03\x26\x03\x26\x03\x26\
		\x07\x26\u{ec}\x0a\x26\x0c\x26\x0e\x26\u{ef}\x0b\x26\x03\x26\x03\x26\x03\
		\x26\x03\x26\x03\x26\x03\x27\x05\x27\u{f7}\x0a\x27\x03\x27\x03\x27\x03\
		\x27\x03\x27\x03\x28\x06\x28\u{fe}\x0a\x28\x0d\x28\x0e\x28\u{ff}\x03\x28\
		\x03\x28\x04\u{df}\u{ed}\x02\x29\x03\x03\x05\x04\x07\x05\x09\x06\x0b\x07\
		\x0d\x08\x0f\x09\x11\x0a\x13\x0b\x15\x0c\x17\x0d\x19\x0e\x1b\x0f\x1d\x10\
		\x1f\x11\x21\x12\x23\x13\x25\x14\x27\x15\x29\x16\x2b\x17\x2d\x18\x2f\x19\
		\x31\x1a\x33\x1b\x35\x1c\x37\x1d\x39\x1e\x3b\x1f\x3d\x20\x3f\x02\x41\x02\
		\x43\x21\x45\x22\x47\x23\x49\x24\x4b\x25\x4d\x26\x4f\x27\x03\x02\x04\x04\
		\x02\x43\x5c\x63\x7c\x04\x02\x0b\x0b\x22\x22\x02\u{114}\x02\x03\x03\x02\
		\x02\x02\x02\x05\x03\x02\x02\x02\x02\x07\x03\x02\x02\x02\x02\x09\x03\x02\
		\x02\x02\x02\x0b\x03\x02\x02\x02\x02\x0d\x03\x02\x02\x02\x02\x0f\x03\x02\
		\x02\x02\x02\x11\x03\x02\x02\x02\x02\x13\x03\x02\x02\x02\x02\x15\x03\x02\
		\x02\x02\x02\x17\x03\x02\x02\x02\x02\x19\x03\x02\x02\x02\x02\x1b\x03\x02\
		\x02\x02\x02\x1d\x03\x02\x02\x02\x02\x1f\x03\x02\x02\x02\x02\x21\x03\x02\
		\x02\x02\x02\x23\x03\x02\x02\x02\x02\x25\x03\x02\x02\x02\x02\x27\x03\x02\
		\x02\x02\x02\x29\x03\x02\x02\x02\x02\x2b\x03\x02\x02\x02\x02\x2d\x03\x02\
		\x02\x02\x02\x2f\x03\x02\x02\x02\x02\x31\x03\x02\x02\x02\x02\x33\x03\x02\
		\x02\x02\x02\x35\x03\x02\x02\x02\x02\x37\x03\x02\x02\x02\x02\x39\x03\x02\
		\x02\x02\x02\x3b\x03\x02\x02\x02\x02\x3d\x03\x02\x02\x02\x02\x43\x03\x02\
		\x02\x02\x02\x45\x03\x02\x02\x02\x02\x47\x03\x02\x02\x02\x02\x49\x03\x02\
		\x02\x02\x02\x4b\x03\x02\x02\x02\x02\x4d\x03\x02\x02\x02\x02\x4f\x03\x02\
		\x02\x02\x03\x51\x03\x02\x02\x02\x05\x59\x03\x02\x02\x02\x07\x60\x03\x02\
		\x02\x02\x09\x67\x03\x02\x02\x02\x0b\x6e\x03\x02\x02\x02\x0d\x72\x03\x02\
		\x02\x02\x0f\x77\x03\x02\x02\x02\x11\x7a\x03\x02\x02\x02\x13\x7e\x03\x02\
		\x02\x02\x15\u{82}\x03\x02\x02\x02\x17\u{86}\x03\x02\x02\x02\x19\u{8a}\
		\x03\x02\x02\x02\x1b\u{8e}\x03\x02\x02\x02\x1d\u{90}\x03\x02\x02\x02\x1f\
		\u{92}\x03\x02\x02\x02\x21\u{94}\x03\x02\x02\x02\x23\u{96}\x03\x02\x02\
		\x02\x25\u{98}\x03\x02\x02\x02\x27\u{9a}\x03\x02\x02\x02\x29\u{9c}\x03\
		\x02\x02\x02\x2b\u{9e}\x03\x02\x02\x02\x2d\u{a0}\x03\x02\x02\x02\x2f\u{a2}\
		\x03\x02\x02\x02\x31\u{a4}\x03\x02\x02\x02\x33\u{a6}\x03\x02\x02\x02\x35\
		\u{a8}\x03\x02\x02\x02\x37\u{aa}\x03\x02\x02\x02\x39\u{ac}\x03\x02\x02\
		\x02\x3b\u{ae}\x03\x02\x02\x02\x3d\u{b0}\x03\x02\x02\x02\x3f\u{b8}\x03\
		\x02\x02\x02\x41\u{ba}\x03\x02\x02\x02\x43\u{bd}\x03\x02\x02\x02\x45\u{c1}\
		\x03\x02\x02\x02\x47\u{ce}\x03\x02\x02\x02\x49\u{da}\x03\x02\x02\x02\x4b\
		\u{e8}\x03\x02\x02\x02\x4d\u{f6}\x03\x02\x02\x02\x4f\u{fd}\x03\x02\x02\
		\x02\x51\x52\x07\x65\x02\x02\x52\x53\x07\x71\x02\x02\x53\x54\x07\x70\x02\
		\x02\x54\x55\x07\x70\x02\x02\x55\x56\x07\x67\x02\x02\x56\x57\x07\x65\x02\
		\x02\x57\x58\x07\x76\x02\x02\x58\x04\x03\x02\x02\x02\x59\x5a\x07\x63\x02\
		\x02\x5a\x5b\x07\x76\x02\x02\x5b\x5c\x07\x76\x02\x02\x5c\x5d\x07\x63\x02\
		\x02\x5d\x5e\x07\x65\x02\x02\x5e\x5f\x07\x6a\x02\x02\x5f\x06\x03\x02\x02\
		\x02\x60\x61\x07\x65\x02\x02\x61\x62\x07\x74\x02\x02\x62\x63\x07\x67\x02\
		\x02\x63\x64\x07\x63\x02\x02\x64\x65\x07\x76\x02\x02\x65\x66\x07\x67\x02\
		\x02\x66\x08\x03\x02\x02\x02\x67\x68\x07\x66\x02\x02\x68\x69\x07\x67\x02\
		\x02\x69\x6a\x07\x6e\x02\x02\x6a\x6b\x07\x67\x02\x02\x6b\x6c\x07\x76\x02\
		\x02\x6c\x6d\x07\x67\x02\x02\x6d\x0a\x03\x02\x02\x02\x6e\x6f\x07\x71\x02\
		\x02\x6f\x70\x07\x77\x02\x02\x70\x71\x07\x76\x02\x02\x71\x0c\x03\x02\x02\
		\x02\x72\x73\x07\x74\x02\x02\x73\x74\x07\x67\x02\x02\x74\x75\x07\x63\x02\
		\x02\x75\x76\x07\x66\x02\x02\x76\x0e\x03\x02\x02\x02\x77\x78\x07\x6b\x02\
		\x02\x78\x79\x07\x70\x02\x02\x79\x10\x03\x02\x02\x02\x7a\x7b\x07\x76\x02\
		\x02\x7b\x7c\x07\x65\x02\x02\x7c\x7d\x07\x72\x02\x02\x7d\x12\x03\x02\x02\
		\x02\x7e\x7f\x07\x77\x02\x02\x7f\u{80}\x07\x66\x02\x02\u{80}\u{81}\x07\
		\x72\x02\x02\u{81}\x14\x03\x02\x02\x02\u{82}\u{83}\x07\x78\x02\x02\u{83}\
		\u{84}\x07\x63\x02\x02\u{84}\u{85}\x07\x74\x02\x02\u{85}\x16\x03\x02\x02\
		\x02\u{86}\u{87}\x07\x69\x02\x02\u{87}\u{88}\x07\x67\x02\x02\u{88}\u{89}\
		\x07\x76\x02\x02\u{89}\x18\x03\x02\x02\x02\u{8a}\u{8b}\x07\x6e\x02\x02\
		\u{8b}\u{8c}\x07\x67\x02\x02\u{8c}\u{8d}\x07\x70\x02\x02\u{8d}\x1a\x03\
		\x02\x02\x02\u{8e}\u{8f}\x07\x2a\x02\x02\u{8f}\x1c\x03\x02\x02\x02\u{90}\
		\u{91}\x07\x2b\x02\x02\u{91}\x1e\x03\x02\x02\x02\u{92}\u{93}\x07\x2e\x02\
		\x02\u{93}\x20\x03\x02\x02\x02\u{94}\u{95}\x07\x24\x02\x02\u{95}\x22\x03\
		\x02\x02\x02\u{96}\u{97}\x07\x29\x02\x02\u{97}\x24\x03\x02\x02\x02\u{98}\
		\u{99}\x07\x31\x02\x02\u{99}\x26\x03\x02\x02\x02\u{9a}\u{9b}\x07\x5e\x02\
		\x02\u{9b}\x28\x03\x02\x02\x02\u{9c}\u{9d}\x07\x5d\x02\x02\u{9d}\x2a\x03\
		\x02\x02\x02\u{9e}\u{9f}\x07\x5f\x02\x02\u{9f}\x2c\x03\x02\x02\x02\u{a0}\
		\u{a1}\x07\x30\x02\x02\u{a1}\x2e\x03\x02\x02\x02\u{a2}\u{a3}\x07\x3c\x02\
		\x02\u{a3}\x30\x03\x02\x02\x02\u{a4}\u{a5}\x07\x3d\x02\x02\u{a5}\x32\x03\
		\x02\x02\x02\u{a6}\u{a7}\x07\x2c\x02\x02\u{a7}\x34\x03\x02\x02\x02\u{a8}\
		\u{a9}\x07\x61\x02\x02\u{a9}\x36\x03\x02\x02\x02\u{aa}\u{ab}\x07\x3f\x02\
		\x02\u{ab}\x38\x03\x02\x02\x02\u{ac}\u{ad}\x07\x2d\x02\x02\u{ad}\x3a\x03\
		\x02\x02\x02\u{ae}\u{af}\x07\x2f\x02\x02\u{af}\x3c\x03\x02\x02\x02\u{b0}\
		\u{b5}\x05\x3f\x20\x02\u{b1}\u{b4}\x05\x3f\x20\x02\u{b2}\u{b4}\x05\x41\
		\x21\x02\u{b3}\u{b1}\x03\x02\x02\x02\u{b3}\u{b2}\x03\x02\x02\x02\u{b4}\
		\u{b7}\x03\x02\x02\x02\u{b5}\u{b3}\x03\x02\x02\x02\u{b5}\u{b6}\x03\x02\
		\x02\x02\u{b6}\x3e\x03\x02\x02\x02\u{b7}\u{b5}\x03\x02\x02\x02\u{b8}\u{b9}\
		\x09\x02\x02\x02\u{b9}\x40\x03\x02\x02\x02\u{ba}\u{bb}\x04\x32\x3b\x02\
		\u{bb}\x42\x03\x02\x02\x02\u{bc}\u{be}\x05\x41\x21\x02\u{bd}\u{bc}\x03\
		\x02\x02\x02\u{be}\u{bf}\x03\x02\x02\x02\u{bf}\u{bd}\x03\x02\x02\x02\u{bf}\
		\u{c0}\x03\x02\x02\x02\u{c0}\x44\x03\x02\x02\x02\u{c1}\u{c8}\x05\x21\x11\
		\x02\u{c2}\u{c9}\x05\x29\x15\x02\u{c3}\u{c9}\x05\x27\x14\x02\u{c4}\u{c9}\
		\x05\x1f\x10\x02\u{c5}\u{c9}\x05\x4d\x27\x02\u{c6}\u{c9}\x05\x2b\x16\x02\
		\u{c7}\u{c9}\x05\x3d\x1f\x02\u{c8}\u{c2}\x03\x02\x02\x02\u{c8}\u{c3}\x03\
		\x02\x02\x02\u{c8}\u{c4}\x03\x02\x02\x02\u{c8}\u{c5}\x03\x02\x02\x02\u{c8}\
		\u{c6}\x03\x02\x02\x02\u{c8}\u{c7}\x03\x02\x02\x02\u{c9}\u{ca}\x03\x02\
		\x02\x02\u{ca}\u{c8}\x03\x02\x02\x02\u{ca}\u{cb}\x03\x02\x02\x02\u{cb}\
		\u{cc}\x03\x02\x02\x02\u{cc}\u{cd}\x05\x21\x11\x02\u{cd}\x46\x03\x02\x02\
		\x02\u{ce}\u{d6}\x05\x23\x12\x02\u{cf}\u{d7}\x05\x43\x22\x02\u{d0}\u{d7}\
		\x05\x3f\x20\x02\u{d1}\u{d7}\x05\x2f\x18\x02\u{d2}\u{d7}\x05\x2d\x17\x02\
		\u{d3}\u{d7}\x05\x25\x13\x02\u{d4}\u{d7}\x05\x27\x14\x02\u{d5}\u{d7}\x05\
		\x31\x19\x02\u{d6}\u{cf}\x03\x02\x02\x02\u{d6}\u{d0}\x03\x02\x02\x02\u{d6}\
		\u{d1}\x03\x02\x02\x02\u{d6}\u{d2}\x03\x02\x02\x02\u{d6}\u{d3}\x03\x02\
		\x02\x02\u{d6}\u{d4}\x03\x02\x02\x02\u{d6}\u{d5}\x03\x02\x02\x02\u{d7}\
		\u{d8}\x03\x02\x02\x02\u{d8}\u{d9}\x05\x23\x12\x02\u{d9}\x48\x03\x02\x02\
		\x02\u{da}\u{db}\x05\x25\x13\x02\u{db}\u{df}\x05\x25\x13\x02\u{dc}\u{de}\
		\x0b\x02\x02\x02\u{dd}\u{dc}\x03\x02\x02\x02\u{de}\u{e1}\x03\x02\x02\x02\
		\u{df}\u{e0}\x03\x02\x02\x02\u{df}\u{dd}\x03\x02\x02\x02\u{e0}\u{e4}\x03\
		\x02\x02\x02\u{e1}\u{df}\x03\x02\x02\x02\u{e2}\u{e5}\x05\x4d\x27\x02\u{e3}\
		\u{e5}\x07\x02\x02\x03\u{e4}\u{e2}\x03\x02\x02\x02\u{e4}\u{e3}\x03\x02\
		\x02\x02\u{e5}\u{e6}\x03\x02\x02\x02\u{e6}\u{e7}\x08\x25\x02\x02\u{e7}\
		\x4a\x03\x02\x02\x02\u{e8}\u{e9}\x05\x25\x13\x02\u{e9}\u{ed}\x05\x33\x1a\
		\x02\u{ea}\u{ec}\x0b\x02\x02\x02\u{eb}\u{ea}\x03\x02\x02\x02\u{ec}\u{ef}\
		\x03\x02\x02\x02\u{ed}\u{ee}\x03\x02\x02\x02\u{ed}\u{eb}\x03\x02\x02\x02\
		\u{ee}\u{f0}\x03\x02\x02\x02\u{ef}\u{ed}\x03\x02\x02\x02\u{f0}\u{f1}\x05\
		\x33\x1a\x02\u{f1}\u{f2}\x05\x25\x13\x02\u{f2}\u{f3}\x03\x02\x02\x02\u{f3}\
		\u{f4}\x08\x26\x02\x02\u{f4}\x4c\x03\x02\x02\x02\u{f5}\u{f7}\x07\x0f\x02\
		\x02\u{f6}\u{f5}\x03\x02\x02\x02\u{f6}\u{f7}\x03\x02\x02\x02\u{f7}\u{f8}\
		\x03\x02\x02\x02\u{f8}\u{f9}\x07\x0c\x02\x02\u{f9}\u{fa}\x03\x02\x02\x02\
		\u{fa}\u{fb}\x08\x27\x02\x02\u{fb}\x4e\x03\x02\x02\x02\u{fc}\u{fe}\x09\
		\x03\x02\x02\u{fd}\u{fc}\x03\x02\x02\x02\u{fe}\u{ff}\x03\x02\x02\x02\u{ff}\
		\u{fd}\x03\x02\x02\x02\u{ff}\u{100}\x03\x02\x02\x02\u{100}\u{101}\x03\x02\
		\x02\x02\u{101}\u{102}\x08\x28\x02\x02\u{102}\x50\x03\x02\x02\x02\x0e\x02\
		\u{b3}\u{b5}\u{bf}\u{c8}\u{ca}\u{d6}\u{df}\u{e4}\u{ed}\u{f6}\u{ff}\x03\
		\x08\x02\x02";
