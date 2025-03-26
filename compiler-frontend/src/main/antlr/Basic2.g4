grammar Basic2;

import Basic2Tokens;

program
  : BEGIN_KW PROC_KW IDENTIFIER
    stmt*
    PROC_KW IDENTIFIER END_KW
  ;

stmt
  : ifElseStmtBlock                  # ElseBlock
  | ifElseStmt                       # ifElse
  | ifElifStmtBlock                  # elifBlock
  | ifElifStmt                       # Elif
  | ifStmtBlock                      # ifBlock
  | ifStmt                           # if
  | whileStmt                        # while
  | whileStmtBlock                   # whileBlock
  | forRange                         # forR
  | forStmt                          # for
  | returnStmt                       # ret
  | breakStmt                        # brk
  | continueStmt                     # cont
  | printStmt                        # prnt
  | arrReStmt                        # arrReAss
  | varDeclStmt                      # varDecl
  | varDeclAssignStmt                # varAs
  | varReStmt                        # varReAs
  | blockStmt                        # block
  | fnDeclStmt                       # decl
  | fnImplStmt                       # impl
  | importStmt                       # use
  | expr incrUni END_STMT_KW         # preIncr
  | IDENTIFIER incr expr END_STMT_KW # binopIncr
  ;

returnStmt : RET_KW expr? END_STMT_KW;
breakStmt : BREAK_KW END_STMT_KW;
continueStmt : CONTINUE_KW END_STMT_KW;
printStmt : PRINT_KW TUPLE_STRT expr TUPLE_END END_STMT_KW;
arrReStmt : IDENTIFIER ARRAY_STRT expr ARRAY_END ASS_KW expr END_STMT_KW;
importStmt
  : IMPORT_KW IDENTIFIER renaming? END_STMT_KW # useAll
  | IMPORT_KW IDENTIFIER
    ARRAY_STRT importItems?
      (SEP importItems?) SEP?
    ARRAY_END
    END_STMT_KW                                # useSpecific
  ;

renaming : AS_KW IDENTIFIER;

importItems
  : FUNCTION_DECL IDENTIFIER renaming? # fnDecl
  | FUNCTION_IMPL IDENTIFIER renaming? # fnImpl
  | FUNCTION_KW IDENTIFIER renaming?   # fnDeclImpl
  | IDENTIFIER renaming?               # var
  ;

varDeclStmt : LET_KW IDENTIFIER typing END_STMT_KW;
varDeclAssignStmt : LET_KW IDENTIFIER typing? ASS_KW expr END_STMT_KW;
varReStmt : IDENTIFIER ASS_KW expr END_STMT_KW;

typing : ':' type;
blockStmt : BLOCK_STRT stmt* END_KW;
fnDeclStmt
  : FUNCTION_DECL IDENTIFIER TUPLE_STRT type? (SEP type)* SEP? TUPLE_END (':' type)? END_STMT_KW
  ;

fnImplStmt
  : FUNCTION_IMPL IDENTIFIER TUPLE_STRT fnParam? (SEP fnParam)* SEP? TUPLE_END stmt
  ;
fnParam : IDENTIFIER (ASS_KW expr)?;

ifElifStmt
  : IF_KW TUPLE_STRT expr TUPLE_END THEN_KW
      stmt
    (ELIF_KW expr THEN_KW stmt)+
    ELSE_KW
      stmt
    END_IF_KW
  ;


ifElifStmtBlock
  : IF_KW TUPLE_STRT expr TUPLE_END BLOCK_STRT
    thenBlock+=stmt*
    elifStmtBlock+
    ELSE_KW BLOCK_STRT
    elseBlock+=stmt*
    END_KW
  ;

elifStmtBlock
  : ELIF_KW TUPLE_STRT expr TUPLE_END BLOCK_STRT stmt*
  ;

ifElseStmt : IF_KW TUPLE_STRT expr TUPLE_END THEN_KW stmt ELSE_KW stmt END_IF_KW;

ifElseStmtBlock : IF_KW TUPLE_STRT expr TUPLE_END
    ifThenBlock
  ELSE_KW
    ifElseBlock;

ifThenBlock : BLOCK_STRT stmt*;

ifElseBlock : stmt* END_KW;

ifStmt : IF_KW TUPLE_STRT expr TUPLE_END THEN_KW stmt END_IF_KW;

ifStmtBlock : IF_KW TUPLE_STRT expr TUPLE_END BLOCK_STRT stmt* END_KW;

whileStmt : WHILE_KW TUPLE_STRT expr TUPLE_END THEN_KW stmt END_KW;
whileStmtBlock : WHILE_KW TUPLE_STRT expr TUPLE_END blockStmt;

forStmt
  : FOR_KW TUPLE_STRT
      IDENTIFIER ASS_KW expr END_KW
      IDENTIFIER comp expr END_KW
      IDENTIFIER ((incr expr) | incrUni) END_KW
    TUPLE_END
    THEN_KW stmt END_KW
  ;

forRange : FOR_KW TUPLE_STRT IDENTIFIER 'IN' iterable TUPLE_END THEN_KW stmt END_KW;

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

incrUni
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

binOp
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
  : NUM_LIT                                                             # intLit
  | FLOAT_LIT                                                           # floatLit
  | STR_LIT                                                             # strLit
  | BOOL_LIT                                                            # boolLit
  | TUPLE_STRT expr TUPLE_END                                           # group
  | TUPLE_STRT expr SEP expr TUPLE_END                                  # tuple
  | ARRAY_STRT expr (SEP expr)* SEP? ARRAY_END                          # arrExpLit
  | ARRAY_STRT type ';' NUM_LIT ARRAY_END                               # arrLit
  | expr AS_KW type                                                     # cast
  | expr binOp expr                                                     # binop
  | expr comp expr                                                      # binopComp
  | expr ARRAY_STRT expr ARRAY_END                                      # arrInd
  | TRIM_KW TUPLE_STRT expr TUPLE_END                                   # trim
  | inputExpr                                                           # input
  | lenExpr                                                             # len
  | IDENTIFIER TUPLE_STRT (expr (SEP expr)* SEP?)? TUPLE_END            # fnCall
  | IDENTIFIER                                                          # ident
  ;

inputExpr : INPUT_KW TUPLE_STRT expr? TUPLE_END;
lenExpr : LEN_KW TUPLE_STRT expr TUPLE_END;
