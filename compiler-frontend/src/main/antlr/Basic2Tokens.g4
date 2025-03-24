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
BLOCK_STRT : 'DO' ;
GEN_TYPE_STRT : '<';
GEN_TYPE_END : '>';
FROM_KW : 'FROM';
TO_KW : 'TO';
fragment CHARS : [a-zA-Z];
SEP : ',';
IMPORT_KW : 'USE';
PRINT_KW : 'PRINT';
INPUT_KW : 'INPUT';
FUNCTION_DECL : 'DECL' ;
FUNCTION_IMPL : 'IMPL' ;
FUNCTION_KW : 'FUNCTION' ;
END_STMT_KW : ';';
AS_KW : 'AS' ;
LET_KW : 'LET';
IF_KW : 'IF';
THEN_KW : 'THEN';
ELSE_KW : 'ELSE';
ELIF_KW : 'ELIF';
END_IF_KW : 'FI';
WHILE_KW : 'WHILE';
RET_KW : 'RETURN';
BREAK_KW : 'BREAK';
CONTINUE_KW : 'CONTINUE';
END_KW : 'END';
BEGIN_KW : 'BEGIN';
PROC_KW : 'PROC';
FOR_KW : 'FOR';
ASS_KW : '=';
LEN_KW : 'LEN';
APPEND_KW : 'ADD';
TRIM_KW : 'TRIM';
PRIM_TYPES
  : 'INT'
  | 'FLOAT'
  | 'STR'
  | 'BOOL'
  ;
IDENTIFIER : (CHARS | '-' | '_')+ (CHARS | '-' | '_' | [0-9])*;
