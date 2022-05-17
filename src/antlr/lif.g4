grammar lif;

import lifWords;

/*
 * Parser Rules
 */
root:instruction*;

instruction:        connect
                |   create
                |   delete
                |   attach
                |   out
                |   for_instr
                |   assignation;

connect : CONNECT server_name protocol DOUBLEDOT ip_address DOUBLEDOT port;

create: CREATE attribut server_name DOUBLEDOT tuple_space_name (attribut)*;

delete:  DELETE attribut server_name DOUBLEDOT tuple_space_name;

attach: ATTACH server_name DOUBLEDOT tuple_space_name (attribut)*;

out: OUT tuple(COMMA tuple)*;

for_instr : FOR ID EQUAL operation TO LPAR operation RPAR LEFT_BRACE instruction+ RIGHT_BRACE;

operation :   get_function
            | len_function
            | right_expr
            | operation PLUS operation
            | operation MINUS operation
            | operation KLEENE operation
            | operation SLASH operation;

get_function: tuple DOT GET LPAR right_expr RPAR;

len_function: tuple DOT LEN LPAR RPAR;

right_expr : ID | NUMBER;

assignation : VAR ID EQUAL ( init_var  |   read   |   in_instr | operation);

read: READ tuple(COMMA tuple)*;

in_instr: IN tuple(COMMA tuple)*;

attribut: STRING | ID ;

tuple : LPAR (tuple_content (COMMA tuple_content)*) RPAR | ID;

tuple_content : init_var | WILDCARD ;

tuple_space_name: STRING | ID ;

server_name: STRING | ID ;

init_var:   NUMBER
            | STRING
            | CHARACTER
            | ID
            | tuple;

protocol : UDP | TCP;

ip_address: NUMBER DOT NUMBER DOT NUMBER DOT NUMBER;

port: NUMBER;