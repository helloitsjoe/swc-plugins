use convert_case::{Case, Casing};
use swc_core::ecma::{
    ast::{CallExpr, Callee, Expr, FnDecl, Program},
    transforms::testing::test,
    visit::{as_folder, FoldWith, VisitMut},
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    // Implement necessary visit_mut_* methods for actual custom transform.
    // A comprehensive list of possible visitor methods can be found here:
    // https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html

    fn visit_mut_fn_decl(&mut self, node: &mut FnDecl) {
        println!("Sym in fn_decl: {:?}", node.ident.sym);
        if node.ident.sym.starts_with('_') {
            // let sym = node.ident.sym.to_string().to_case(Case::Snake);
            // node.ident.sym = format!("_{}", sym).into();
            // } else {
            // node.ident.sym = node.ident.sym.to_string().to_case(Case::Snake).into();
        }
        println!("Sym in fn_decl after: {:?}", node.ident.sym);
    }

    // https://swc.rs/docs/plugin/ecmascript/cheatsheet
    fn visit_mut_call_expr(&mut self, node: &mut CallExpr) {
        if let Callee::Expr(expr) = &mut node.callee {
            // println!("{:?}", expr);
            if let Expr::Ident(ident) = &mut **expr {
                println!("Sym in call_expr: {:?}", ident.sym);
                if ident.sym == *"require" {
                    return;
                }
                if ident.sym.starts_with('_') {
                    // let sym = ident.sym.to_string().to_case(Case::Snake);
                    // ident.sym = format!("_{}", sym).into();
                    // } else {
                    // ident.sym = ident.sym.to_string().to_case(Case::Snake).into();
                }
                println!("Sym in call_expr after: {:?}", ident.sym);
            }
        }
    }
}

/// An example plugin function with macro support.
/// `plugin_transform` macro interop pointers into deserialized structs, as well
/// as returning ptr back to host.
///
/// It is possible to opt out from macro by writing transform fn manually
/// if plugin need to handle low-level ptr directly via
/// `__transform_plugin_process_impl(
///     ast_ptr: *const u8, ast_ptr_len: i32,
///     unresolved_mark: u32, should_enable_comments_proxy: i32) ->
///     i32 /*  0 for success, fail otherwise.
///             Note this is only for internal pointer interop result,
///             not actual transform result */`
///
/// This requires manual handling of serialization / deserialization from ptrs.
/// Refer swc_plugin_macro to see how does it work internally.
#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor))
}

// An example to test plugin transform.
// Recommended strategy to test plugin's transform is verify
// the Visitor's behavior, instead of trying to run `process_transform` with mocks
// unless explicitly required to do so.
test!(
    Default::default(),
    |_| as_folder(TransformVisitor),
    boo,
    // Input codes
    r#"function helloThere() { console.log('hi'); }; helloThere();"#,
    // Output codes after transformed with plugin
    r#"function hello_there() { console.log('hi'); }; hello_there();"#
);
