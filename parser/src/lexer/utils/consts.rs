pub const TUPLE_START: &str = "(";
pub const TUPLE_END: &str = ")";
pub const TUPLE_END_CHAR: char = ')';
pub const TUPLE_DELIMITER: &str = ",";
pub const TUPLE_DELIMITER_CHAR: char = ',';

pub const VARIABLE_DECLARATION: &str = "LET";
pub const VARIABLE_TYPE_START: &str = ":";

pub const GROUP_START: &str = "(";
pub const GROUP_END: &str = ")";

pub const LIST_START: &str = "[";
pub const LIST_END: &str = "]";
pub const LIST_END_CHAR: char = ']';
pub const LIST_DELIMITER: &str = ",";
pub const LIST_DELIMITER_CHAR: char = ',';
pub const LIST_UNPACKING_KW: &str = "...";

pub const FUNCTION_IMPLEMENTATION_KW: &str = "IMPL";
pub const FUNCTION_IMPLEMENTATION_START_KW: &str = "DOES";
pub const FUNCTION_DECLARATION_KW: &str = "DECL";
pub const FUNCTION_BODY_END_KW: &str = "END";
pub const FUNCTION_PARAMETERS_START: &str = "(";
pub const FUNCTION_PARAMETERS_END: &str = ")";
pub const FUNCTION_PARAMETERS_END_CHAR: char = ')';
pub const FUNCTION_PARAMETERS_DELIMITER: &str = ",";
pub const FUNCTION_PARAMETERS_DELIMITER_CHAR: char = ',';

pub const FUNCTION_CALL_START: &str = "(";
pub const FUNCTION_CALL_END: &str = ")";
pub const FUNCTION_CALL_END_CHAR: char = ')';
pub const FUNCTION_CALL_DELIMITER: &str = ",";
pub const FUNCTION_CALL_DELIMITER_CHAR: char = ',';

pub const BEGIN_MODULE_KW: &str = "BEGIN";
pub const MODULE_KW: &str = "MODULE";
pub const END_MODULE_KW: &str = "END";
pub const ONE_SPACE: &str = " ";

pub const STR_TYPE_KW: &str = "STR";
pub const INT_TYPE_KW: &str = "INT";
pub const BOOL_TYPE_KW: &str = "BOOL";

pub const STRUCT_KW: &str = "STRUCTURE";
pub const STRUCT_DECL_KW: &str = "WHERE";
pub const STRUCT_START_KW: &str = "WITH";
pub const STRUCT_FIELD_DECL_KW: &str = "DECL";
pub const STRUCT_FIELD_IMPL_KW: &str = "IMPL";
pub const STRUCT_FIELD_ASSIGNMENT: &str = "=";
pub const STRUCT_END_KW: &str = "END";
pub const STRUCT_FIELD_END: &str = ";";
pub const STRUCT_FIELD_ACCESS_KW: &str = "::";

pub const IF_STATEMENT_START_KW: &str = "IF";
pub const IF_STATEMENT_BODY_START_KW: &str = "THEN";
pub const IF_STATEMENT_END_KW: &str = "FI";

pub const WHILE_STATEMENT_START_KW: &str = "WHILE";
pub const WHILE_STATEMENT_BODY_START_KW: &str = "DO";
pub const WHILE_STATEMENT_END_KW: &str = "END";

pub const BLOCK_STATEMENT_START_KW: &str = "DO";
pub const BLOCK_STATEMENT_END_KW: &str = "END";

pub const SINGLE_LINE_COMMENT: &str = "#";
pub const SINGLE_LINE_COMMENT_END: &str = "\n";

pub const FUNCTION_INVOCATION_START_KW: &str = "INVOKE";
pub const FUNCTION_INVOCATION_END: &str = ";";

pub const VARIABLE_REASIGNMENT: &str = "=";
pub const END_STMT_KW: &str = ";";
pub const BREAK_STMT_KW: &str = "BREAK";
pub const RETURN_STMT_KW: &str = "RETURN";

pub const STRING_KW: &str = "\"";
pub const STRING_CHAR: char = '"';
pub const ASSIGNMENT_KW: &str = "=";

pub const INCR_KW: &str = "++";
pub const DECR_KW: &str = "--";
pub const NEGATION_KW: &str = "!";

pub const OR_KW: &str = "||";
pub const AND_KW: &str = "&&";
pub const MOD_KW: &str = "%";
pub const NEQ_KW: &str = "!=";
pub const LT_KW: &str = "<";
pub const GT_KW: &str = ">";
pub const GEQ_KW: &str = ">=";
pub const LEQ_KW: &str = "<=";
pub const EQ_KW: &str = "==";
pub const POW_KW: &str = "^";
pub const DIV_KW: &str = "/";
pub const SUB_KW: &str = "-";
pub const MUL_KW: &str = "*";
pub const ADD_KW: &str = "+";

pub const TYPE_ALIAS_KW: &str = "ALIAS";

pub const IMPORT_MODULE_KW: &str = "USE";

pub const INDEX_START_KW: &str = "[";
pub const INDEX_END_KW: &str = "]";

pub const UNPACK_KW: &str = "><";

pub const MULTI_LINE_COMMENT_START: &str = "#-";
pub const MULTI_LINE_COMMENT_END: &str = "-#";

pub const FOR_START_KW: &str = "FOR";
pub const FOR_CONDITION_START_KW: &str = "(";
pub const FOR_CONDITION_END_KW: &str = ")";
pub const FOR_BODY_START_KW: &str = "THEN";
pub const FOR_END_KW: &str = "END";

pub const ENUM_START_KW: &str = "ENUMS";
pub const ENUM_END_KW: &str = "END";
pub const ENUM_INDEXING: &str = ".";

#[cfg(test)]
pub mod test_const {
    pub const EMPTY_PROGRAM_PATH: &str = "../assets/basic-examples/EmptyExample.b2";
    pub const BASIC_EXAMPLE_FOLDER: &str = "../assets/basic-examples/";
}
