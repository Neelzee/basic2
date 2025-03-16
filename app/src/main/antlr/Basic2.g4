grammar Basic2;

import Basic2Tokens;

program : stmt*;

stmt
  : print_stmt        # print
  | input_stmt        # input
  | var_decl_stmt     # var_decl
  | var_decl_ass_stmt # var_ass
  | var_re_ass_stmt   # var_re_ass
  | body_stmt         # body
  | fn_decl_stmt      # fn_decl
  | fn_impl_stmt      # fn_impl
  ;

print_stmt : PRINT_KW TUPLE_STRT expr TUPLE_END END_KW;
input_stmt : var_decl_stmt? INPUT_KW TUPLE_STRT expr TUPLE_END END_KW;

var_decl_stmt : LET_KW IDENTIFIER typing END_KW;
var_decl_ass_stmt : LET_KW IDENTIFIER typing? ASS_KW expr END_KW;
var_re_ass_stmt : IDENTIFIER ASS_KW expr END_KW;

typing : IDENTIFIER ':' type;
body_stmt : BODY_STRT stmt* BODY_END;
fn_decl_stmt
  : FUNCTION_DECL IDENTIFIER TUPLE_STRT type? (SEP type)* SEP? TUPLE_END ':' type END_KW
  ;
fn_impl_stmt
  : FUNCTION_IMPL IDENTIFIER TUPLE_STRT fn_param? (SEP fn_param)* SEP? TUPLE_END stmt
  ;
fn_param : IDENTIFIER (ASS_KW expr)?;

type
  : PRIM_TYPES
  | ARRAY_STRT type ARRAY_END
  | TUPLE_STRT type SEP type TUPLE_END
  ;

expr
  : IDENTIFIER                                 # ident
  | NUM_LIT                                    # num
  | FLOAT_LIT                                  # float
  | STR_LIT                                    # str
  | BOOL_LIT                                   # bool
  | TUPLE_STRT expr SEP expr TUPLE_END         # tuple
  | ARRAY_STRT expr (SEP expr)* SEP? ARRAY_END # array
  | expr BIN_OP expr                           # binop
  | UNI_OP expr                                # opuni
  | expr UNI_OP                                # uniop
  ;
