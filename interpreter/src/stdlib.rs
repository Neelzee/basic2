use crate::interpreter::gst::GlobalSymbolTable;
use parser::{
    pstmt::decls::variants::FunctionDeclaration,
};
use common::types::{Type, MonoType};

pub const PRINT_IDENT: &'static str = "PRINT";

pub fn add_stdlib(mut gst: GlobalSymbolTable) -> GlobalSymbolTable {
    gst = gst.insert_fn_decl(
        PRINT_IDENT,
        FunctionDeclaration::new(
            PRINT_IDENT,
            vec![Type::Mono(MonoType::TypeVarGen(
                "T",
                vec!["SHOWABLE"],
            ))],
            Vec::new(),
            None,
        ),
    );
    gst
}
