lexer grammar lifWords;

/*
 * Lexer Rules
 */

// Words
CONNECT : 'connect';
ATTACH : 'attach';
CREATE : 'create';
DELETE : 'delete';
OUT : 'out';
READ : 'read';
IN : 'in';
TCP : 'tcp';
UDP : 'udp';
VAR : 'var';
GET : 'get';
LEN : 'len';
FOR : 'for';
TO : 'to';

// Char
LPAR: '(';
RPAR: ')';
COMMA: ',';
DOUBLEQUOTE:'"';
QUOTE:'\'';
SLASH : '/';
BACKSLASH:'\\';
LBRACKET:'[';
RBRACKET:']';
DOT:'.';
DOUBLEDOT : ':';
SEMICOLON : ';';
KLEENE : '*';
WILDCARD: '_';
EQUAL : '=';
PLUS: '+';
MINUS: '-';
RIGHT_BRACE : '}';
LEFT_BRACE : '{';

ID: LETTER (LETTER | DIGIT | WILDCARD)* ;
fragment LETTER: 'A'..'Z' | 'a'..'z' ;
fragment DIGIT: '0'..'9' ;

//Types
NUMBER: (DIGIT)+;
STRING : DOUBLEQUOTE(LBRACKET|BACKSLASH|COMMA|NEWLINE|RBRACKET|ID)+DOUBLEQUOTE;
CHARACTER : QUOTE (NUMBER|LETTER|DOUBLEDOT|DOT|SLASH|BACKSLASH|SEMICOLON) QUOTE;

// Comments -> ignored

LINECOMMENT : (SLASH SLASH .*? (NEWLINE|EOF)) ->skip;
COMMENT: (SLASH KLEENE .*? KLEENE SLASH) -> skip;

// Whitespaces -> ignored

NEWLINE: '\r'? '\n'  -> skip ;
WS: [ \t]+ -> skip ;