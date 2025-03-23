grammar Basic2;

import Basic2Tokens;

program
  : BEGIN_KW PROC_KW IDENTIFIER
    stmt*
    PROC_KW IDENTIFIER END_KW
  ;

stmt
  : if_else_stmt_block               # if_else_block
  | if_else_stmt                     # if_else
  | if_elif_stmt_block               # if_elif_block
  | if_elif_stmt                     # if_elif
  | if_stmt_block                    # if_block
  | if_stmt                          # if
  | while_stmt                       # while
  | while_stmt_block                 # while_block
  | for_range                        # for_r
  | for_stmt                         # for
  | return_stmt                      # ret
  | break_stmt                       # break
  | continue_stmt                    # continue
  | print_stmt                       # print
  | append_stmt                      # append
  | arr_re_ass_stmt                  # arrReAss
  | var_decl_stmt                    # var_decl
  | var_decl_ass_stmt                # var_ass
  | var_re_ass_stmt                  # var_re_ass
  | block_stmt                       # block
  | fn_decl_stmt                     # fn_decl
  | fn_impl_stmt                     # fn_impl
  | import_stmt                      # use
  | expr incr_uni END_STMT_KW        # preIncr
  | incr_uni expr  END_STMT_KW       # postIncr
  | IDENTIFIER incr expr END_STMT_KW # binopIncr
  ;

return_stmt : RET_KW expr? END_STMT_KW;
break_stmt : BREAK_KW END_STMT_KW;
continue_stmt : CONTINUE_KW END_STMT_KW;
print_stmt : PRINT_KW TUPLE_STRT expr TUPLE_END END_STMT_KW;
append_stmt : APPEND_KW TUPLE_STRT expr SEP IDENTIFIER TUPLE_END END_STMT_KW;
arr_re_ass_stmt : IDENTIFIER ARRAY_STRT expr ARRAY_END ASS_KW expr END_STMT_KW;
import_stmt
  : IMPORT_KW IDENTIFIER renaming? END_STMT_KW # useAll
  | IMPORT_KW IDENTIFIER
    ARRAY_STRT import_items renaming?
      (SEP import_items renaming?) SEP?
    ARRAY_END
    END_STMT_KW                                # useSpecific
  ;

renaming : AS_KW IDENTIFIER;

import_items
  : FUNCTION_DECL IDENTIFIER # fnDecl
  | FUNCTION_IMPL IDENTIFIER # fnImpl
  | FUNCTION_KW IDENTIFIER   # fnDeclImpl
  | IDENTIFIER               # var
  ;

var_decl_stmt : LET_KW IDENTIFIER typing END_STMT_KW;
var_decl_ass_stmt : LET_KW IDENTIFIER typing? ASS_KW expr END_STMT_KW;
var_re_ass_stmt : IDENTIFIER ASS_KW expr END_STMT_KW;

typing : ':' type;
block_stmt : BLOCK_STRT stmt* END_KW;
fn_decl_stmt
  : FUNCTION_DECL IDENTIFIER TUPLE_STRT type? (SEP type)* SEP? TUPLE_END (':' type)? END_STMT_KW
  ;
fn_impl_stmt
  : FUNCTION_IMPL IDENTIFIER TUPLE_STRT fn_param? (SEP fn_param)* SEP? TUPLE_END stmt
  ;
fn_param : IDENTIFIER (ASS_KW expr)?;

if_elif_stmt
  : IF_KW TUPLE_STRT expr TUPLE_END THEN_KW
      stmt
    (ELIF_KW expr THEN_KW stmt)+
    ELSE_KW
      stmt
    END_IF_KW
  ;


if_elif_stmt_block
  : IF_KW TUPLE_STRT expr TUPLE_END BLOCK_STRT
    thenBlock+=stmt*
    (elifBlocks+=elif_stmt_block)+
    ELSE_KW BLOCK_STRT
    elseBlock+=stmt*
    END_KW
  ;

elif_stmt_block
  : ELIF_KW TUPLE_STRT expr TUPLE_END BLOCK_STRT stmt* #ElifBlockBranch
  ;

if_else_stmt : IF_KW TUPLE_STRT expr TUPLE_END THEN_KW stmt ELSE_KW stmt END_IF_KW;

if_else_stmt_block : IF_KW TUPLE_STRT expr TUPLE_END
    ifThenBlock
  ELSE_KW
    ifElseBlock;

ifThenBlock : BLOCK_STRT stmt*;

ifElseBlock : stmt* END_KW;

if_stmt : IF_KW TUPLE_STRT expr TUPLE_END THEN_KW stmt END_IF_KW;

if_stmt_block : IF_KW TUPLE_STRT expr TUPLE_END BLOCK_STRT stmt* END_KW;

while_stmt : WHILE_KW TUPLE_STRT expr TUPLE_END THEN_KW stmt END_KW;
while_stmt_block : WHILE_KW TUPLE_STRT expr TUPLE_END block_stmt;

for_stmt
  : FOR_KW TUPLE_STRT
      IDENTIFIER ASS_KW expr END_KW
      IDENTIFIER comp expr END_KW
      IDENTIFIER ((incr expr) | incr_uni) END_KW
    TUPLE_END
    THEN_KW stmt END_KW
  ;

for_range : FOR_KW TUPLE_STRT IDENTIFIER 'IN' iterable TUPLE_END THEN_KW stmt END_KW;

comp
  : '=='
  | '!='
  | '<='
  | '<'
  | '>='
  | '>'
  ;

incr
  : '+='
  | '-='
  | '/='
  | '*='
  | '%='
  | '^='
  | '&='
  | '|='
  ;

incr_uni
  : '++'
  | '--'
  ;

type
  : PRIM_TYPES
  | ARRAY_STRT type ARRAY_END
  | TUPLE_STRT type SEP type TUPLE_END
  ;

iterable
  : FROM_KW expr TO_KW expr
  | expr
  ;

bin_op
  : '+'
  | '-'
  | '/'
  | '*'
  | '%'
  | '^'
  | '&&'
  | '||'
  ;

expr
  : NUM_LIT                                                             # num
  | FLOAT_LIT                                                           # float
  | STR_LIT                                                             # str
  | BOOL_LIT                                                            # bool
  | TUPLE_STRT expr TUPLE_END                                           # group
  | TUPLE_STRT expr SEP expr TUPLE_END                                  # tuple
  | (ARRAY_STRT expr (SEP expr)* SEP? ARRAY_END | ARRAY_STRT ARRAY_END) # array
  | expr AS_KW type                                                     # cast
  | expr bin_op expr                                                    # binop
  | expr comp expr                                                      # binopComp
  | expr '?' expr ':' expr                                              # ternary
  | expr ARRAY_STRT expr ARRAY_END                                      # arrInd
  | TRIM_KW TUPLE_STRT expr TUPLE_END                                   # trim
  | inputExpr                                                           # input
  | lenExpr                                                             # len
  | IDENTIFIER TUPLE_STRT (expr (SEP expr)* SEP?)? TUPLE_END            # fnCall
  | IDENTIFIER                                                          # ident
  ;

inputExpr : INPUT_KW TUPLE_STRT expr? TUPLE_END;
lenExpr : LEN_KW TUPLE_STRT expr TUPLE_END;
