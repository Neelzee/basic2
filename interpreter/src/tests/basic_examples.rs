use crate::{
    interpreter::{gst::GlobalSymbolTable, interpret},
    stdlib::{PRINT_IDENT, add_stdlib},
};
use parser::lexer::{lex_mod::LexModule, utils::Span};
use std::{fs::File, io::Read};

#[test]
fn test_hello_world() {
    let mut file = File::open("../assets/basic-examples/HelloWorld.b2").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let module_res = LexModule::parse_program(Span::new(&buf));
    assert!(module_res.is_ok(), "{module_res:?}");
    let (_, module) = module_res.unwrap();
    let gst = GlobalSymbolTable::from_module(module);
    let gst_with_lib = add_stdlib(gst);
    assert!(gst_with_lib.lookup_fn_decl(PRINT_IDENT).is_some());
}
