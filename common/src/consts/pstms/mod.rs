pub mod fns;
pub mod skill;
pub mod structs;
pub mod when;

pub const VARIABLE_DECLARATION: &str = "LET";
pub const VARIABLE_TYPE_START: &str = ":";

pub const BEGIN_MODULE_KW: &str = "BEGIN";
pub const MODULE_KW: &str = "MODULE";
pub const END_MODULE_KW: &str = "END";
pub const ONE_SPACE: &str = " ";

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

pub const VARIABLE_REASIGNMENT: &str = "=";
pub const END_STMT_KW: &str = ";";
pub const BREAK_STMT_KW: &str = "BREAK";
pub const CONTINUE_STMT_KW: &str = "CONTINUE";
pub const RETURN_STMT_KW: &str = "RETURN";

pub const ASSIGNMENT_KW: &str = "=";

pub const TYPE_ALIAS_KW: &str = "ALIAS";

pub const IMPORT_MODULE_KW: &str = "USE";

pub const MULTI_LINE_COMMENT_START: &str = "-#";
pub const MULTI_LINE_COMMENT_END: &str = "#-";

pub const FOR_START_KW: &str = "FOR";
pub const FOR_CONDITION_START_KW: &str = "(";
pub const FOR_CONDITION_END_KW: &str = ")";
pub const FOR_BODY_START_KW: &str = "THEN";
pub const FOR_END_KW: &str = "END";

pub const ENUM_START_KW: &str = "ENUMS";
pub const ENUM_END_KW: &str = "END";
pub const ENUM_INDEXING: &str = ".";

pub const UNUSED_IDENTIFIER: &str = "_";
pub const UNUSED_IDENTIFIER_CHAR: char = '_';
