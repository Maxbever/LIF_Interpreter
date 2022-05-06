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
                |   read
                |   in_instr;

connect : CONNECT protocol DOUBLEDOT ip_address DOUBLEDOT port;

create: CREATE attribut tuple_space_name (attribut)*;

delete:  DELETE attribut tuple_space_name;

attach: ATTACH tuple_space_name (attribut)*;

read: READ tuple;

in_instr: IN tuple;

out: OUT tuple;

attribut: init_var;

tuple : LPAR (tuple_content (COMMA tuple_content)*) RPAR;
tuple_content : STRING | WILDCARD | NUMBER | CHARACTER | tuple | ID ;

tuple_space_name: init_var;

init_var:   NUMBER
            | STRING
            | CHARACTER
            | ID;

protocol : UDP | TCP;

ip_address: NUMBER DOT NUMBER DOT NUMBER DOT NUMBER;

port: NUMBER;