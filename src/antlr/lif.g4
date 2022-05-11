grammar lif;

import lifWords;

/*
 * Parser Rules
 */
root:instruction*;

instruction:    connect
                |   create
                |   delete
                |   attach
                |   out
                |   assignation;

connect : CONNECT protocol DOUBLEDOT ip_address DOUBLEDOT port;

create: CREATE attribut tuple_space_name (attribut)*;

delete:  DELETE attribut tuple_space_name;

attach: ATTACH tuple_space_name (attribut)*;

read: READ tuple(COMMA tuple)*;

in_instr: IN tuple(COMMA tuple)*;

out: OUT tuple(COMMA tuple)*;

get_value: tuple DOT GET LPAR NUMBER RPAR;

assignation : VAR ID EQUAL ( init_var  |   read   |   in_instr | get_value );

attribut: STRING | ID ;

tuple : LPAR (tuple_content (COMMA tuple_content)*) RPAR | ID;
tuple_content : init_var | WILDCARD ;

tuple_space_name: STRING | ID ;

init_var:   NUMBER
            | STRING
            | CHARACTER
            | ID
            | tuple;

protocol : UDP | TCP;

ip_address: NUMBER DOT NUMBER DOT NUMBER DOT NUMBER;

port: NUMBER;