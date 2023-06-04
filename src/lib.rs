use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::*,
        transforms::testing::test,
        visit::{as_folder, FoldWith, VisitMut},
    },
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    // Implement necessary visit_mut_* methods for actual custom transform.
    // A comprehensive list of possible visitor methods can be found here:
    // https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html
    fn visit_mut_call_expr(&mut self, e: &mut CallExpr) {
        match &mut e.callee {
            Callee::Expr(i) => {
                if let Expr::Member(MemberExpr { obj, prop, .. }) = &**i {
                    if let Expr::Ident(Ident { sym: obj_sym, .. }) = &**obj {
                        if obj_sym.as_ref() == "console" {
                            if let MemberProp::Ident(Ident { sym: prop_sym, .. }) = prop {
                                if prop_sym.as_ref() == "log" {
                                    for arg in &mut e.args {
                                        let ExprOrSpread { expr, .. } = arg;
                                        if let Expr::Lit(Lit::Str(Str { value: str_val, .. })) =
                                            &**expr
                                        {
                                            if str_val.as_ref() == "transform" {
                                                *expr = Box::new(Expr::Lit(Lit::Str(Str {
                                                    span: DUMMY_SP,
                                                    value: "TRANSFORM".into(),
                                                    raw: None,
                                                })));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
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
// the Visitor's behavior, instead of trying to run `process_transform` with
// mocks unless explicitly required to do so.
test!(
    Default::default(),
    |_| as_folder(TransformVisitor),
    boo,
    // Input codes
    r#"console.log("transform");"#,
    // Output codes after transformed with plugin
    r#"console.log("TRANSFORM");"#
);
