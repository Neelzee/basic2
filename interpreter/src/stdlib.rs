use crate::interpreter::gst::GlobalSymbolTable;
use parser::lexer::{
    lex_stmt::decls::variants::FunctionDeclaration,
    lex_type::{LexMonoType, LexType},
};

pub const PRINT_IDENT: &'static str = "PRINT";

pub fn add_stdlib(mut gst: GlobalSymbolTable) -> GlobalSymbolTable {
    gst = gst.insert_fn_decl(
        PRINT_IDENT,
        FunctionDeclaration::new(
            PRINT_IDENT,
            vec![LexType::Mono(LexMonoType::TypeVarGen(
                "T",
                vec!["SHOWABLE"],
            ))],
            Vec::new(),
            None,
        ),
    );
    gst
}
