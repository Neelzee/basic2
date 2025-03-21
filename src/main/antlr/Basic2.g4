grammar Basic2;

import Basic2Tokens;

program : stmt*;

stmt
  : print_stmt          # print
  | input_stmt          # input
  | var_decl_stmt       # var_decl
  | var_decl_ass_stmt   # var_ass
  | var_re_ass_stmt     # var_re_ass
  | block_stmt          # block
  | fn_decl_stmt        # fn_decl
  | fn_impl_stmt        # fn_impl
  | if_else_stmt        # if_else
  | if_stmt             # if
  | if_else_stmt_block  # if_else_block
  | if_stmt_block       # if_block
  | while_stmt          # while
  | for_range           # for_r
  | for_stmt            # for
  | return_stmt         # ret
  | break_stmt          # break
  ;

return_stmt : RET_KW expr? END_KW;
break_stmt : BREAK_KW END_KW;
print_stmt : PRINT_KW TUPLE_STRT expr TUPLE_END END_KW;
input_stmt : var_decl_stmt? INPUT_KW TUPLE_STRT expr TUPLE_END END_KW;

var_decl_stmt : LET_KW IDENTIFIER  typing END_KW;
var_decl_ass_stmt : LET_KW IDENTIFIER typing? ASS_KW expr END_KW;
var_re_ass_stmt : IDENTIFIER ASS_KW expr END_KW;

typing : ':' type;
block_stmt : BLOCK_STRT stmt* BLOCK_END;
fn_decl_stmt
  : FUNCTION_DECL IDENTIFIER TUPLE_STRT type? (SEP type)* SEP? TUPLE_END ':' type END_KW
  ;
fn_impl_stmt
  : FUNCTION_IMPL IDENTIFIER TUPLE_STRT fn_param? (SEP fn_param)* SEP? TUPLE_END stmt
  ;
fn_param : IDENTIFIER (ASS_KW expr)?;

if_else_stmt : IF_KW TUPLE_STRT expr TUPLE_END THEN_KW stmt ELSE_KW stmt END_IF_KW;
if_else_stmt_block : IF_KW TUPLE_STRT expr TUPLE_END block_stmt ELSE_KW block_stmt;
if_stmt : IF_KW TUPLE_STRT expr TUPLE_END THEN_KW stmt END_IF_KW;
if_stmt_block : IF_KW TUPLE_STRT expr TUPLE_END block_stmt;

while_stmt : WHILE_KW TUPLE_STRT expr TUPLE_END THEN_KW stmt END_LOOP_KW;

for_stmt
  : FOR_KW TUPLE_STRT
      IDENTIFIER ASS_KW NUM_LIT END_KW
      IDENTIFIER comp expr END_KW
      IDENTIFIER ((incr expr) | incr_uni) END_KW
    TUPLE_END
    THEN_KW stmt END_LOOP_KW
  ;

for_range : FOR_KW TUPLE_STRT IDENTIFIER 'IN' iterable TUPLE_END THEN_KW stmt END_LOOP_KW;

comp
  : '==' # eq
  | '!=' # neq
  | '<=' # gteq
  | '<'  # gt
  | '>=' # lteq
  | '>'  # lt
  ;

incr
  : '+=' # mutAdd
  | '-=' # mutSub
  | '/=' # mutDiv
  | '*=' # mutMul
  | '%=' # mutMod
  | '^=' # mutPow
  ;

incr_uni
  : '++' # inc
  | '--' # dec
  ;

type
  : PRIM_TYPES
  | ARRAY_STRT type ARRAY_END
  | TUPLE_STRT type SEP type TUPLE_END
  ;

iterable
  : 'FROM' NUM_LIT 'TO' NUM_LIT
  | expr
  ;

bin_op
  : '+'  # add
  | '-'  # sub
  | '/'  # div
  | '*'  # mul
  | '%'  # mod
  | '^'  # pow
  | '&&' # and
  | '||' # or
  | '&=' # mutAnd
  | '|=' # mutOr
  | incr # incr_bin
  | comp # comp_bin
  ;

uni_op
  : '!'      # neg
  | '-'      # sig
  | incr_uni # uni
  ;

expr
  : NUM_LIT                                                  # num
  | FLOAT_LIT                                                # float
  | STR_LIT                                                  # str
  | BOOL_LIT                                                 # bool
  | TUPLE_STRT expr SEP expr TUPLE_END                       # tuple
  | ARRAY_STRT expr (SEP expr)* SEP? ARRAY_END               # array
  | expr bin_op expr                                         # binop
  | uni_op expr                                              # opuni
  | expr uni_op                                              # uniop
  | expr '?' expr ':' expr                                   # ternary
  | IDENTIFIER TUPLE_STRT (expr (SEP expr)* SEP?)? TUPLE_END # fnCall
  | IDENTIFIER                                               # ident
  ;
