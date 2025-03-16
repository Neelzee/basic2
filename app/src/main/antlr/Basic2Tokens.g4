lexer grammar Basic2Tokens;

STR_LIT : '"' ~["] '"' ;
NUM_LIT : '-'? [0-9] [0-9]* ;
FLOAT_LIT : '-'? [0-9]* '.'? [0-9]* 'f' ;
BOOL_LIT : 'TRUE' | 'FALSE' ;
TUPLE_STRT : '(';
TUPLE_END : ')';
ARRAY_STRT : '[';
ARRAY_END : ']';
BODY_STRT : '{' ;
BODY_END : '}' ;
BIN_OP
  : '+'
  | '-'
  | '/'
  | '*'
  | '%'
  | '^'
  | '&&'
  | '||'
  | '+='
  | '-='
  | '/='
  | '*='
  | '%='
  | '^='
  | '&='
  | '|='
  | '=='
  | '!='
  | '<='
  | '<'
  | '>='
  | '>'
  ;
UNI_OP
  : '!'
  | '++'
  | '--'
  ;
fragment CHARS : [a-zA-Z];
IDENTIFIER : (CHARS | '-' | '_')+ (CHARS | '-' | '_' | [0-9])*;
SEP : ',';
PRINT_KW : 'PRINT';
INPUT_KW : 'INPUT';
FUNCTION_DECL : 'DECL' ;
FUNCTION_IMPL : 'IMPL' ;
END_KW : ';';
LET_KW : 'LET';
ASS_KW : '=';
PRIM_TYPES
  : 'INT'
  | 'FLOAT'
  | 'STR'
  | 'BOOL'
  ;