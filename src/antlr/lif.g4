grammar lif;

import lifWords;

/*
 * Parser Rules
 */
root:instruction*;

instruction:    CONNECT protocol DOUBLEDOT ipAddress DOUBLEDOT port
                |   CREATE attribut tupleSpaceName (attribut)*
                |   DELETE attribut tupleSpaceName
                |   ATTACH tupleSpaceName (attribut)*
                |   OUT tuple
                |   READ tuple
                |   IN tuple;

attribut: initvar;

tuple : LPAR (tuple_content (COMMA tuple_content)*) RPAR;
tuple_content : STRING | WILDCARD | NUMBER | CHARACTER | tuple | ID ;

tupleSpaceName: initvar;

initvar:   NUMBER
            | STRING
            | CHARACTER
            | ID;

protocol : UDP | TCP;

ipAddress: NUMBER DOT NUMBER DOT NUMBER DOT NUMBER;

port: NUMBER;