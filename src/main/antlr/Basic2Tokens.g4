lexer grammar Basic2Tokens;

WHITESPACE : [ \r\n\t] -> skip;
STR_LIT : '"' (~["])* '"' ;
NUM_LIT : '-'? [0-9] [0-9]* ;
FLOAT_LIT : '-'? [0-9]* '.'? [0-9]* 'f' ;
BOOL_LIT : 'TRUE' | 'FALSE' ;
TUPLE_STRT : '(';
TUPLE_END : ')';
ARRAY_STRT : '[';
ARRAY_END : ']';
BODY_STRT : '{' ;
BODY_END : '}' ;
fragment CHARS : [a-zA-Z];
SEP : ',';
PRINT_KW : 'PRINT';
INPUT_KW : 'INPUT';
FUNCTION_DECL : 'DECL' ;
FUNCTION_IMPL : 'IMPL' ;
END_KW : ';';
LET_KW : 'LET';
IF_KW : 'IF';
THEN_KW : 'THEN';
ELSE_KW : 'ELSE';
END_IF_KW : 'FI';
ASS_KW : '=';
PRIM_TYPES
  : 'INT'
  | 'FLOAT'
  | 'STR'
  | 'BOOL'
  ;
IDENTIFIER : (CHARS | '-' | '_')+ (CHARS | '-' | '_' | [0-9])*;
