use crate::interpreter::gst::GlobalSymbolTable;
use parser::{
    pstmt::decls::variants::FunctionDeclaration,
    ltype::{LexMonoType, LexType},
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
