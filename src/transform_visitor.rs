pub use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::*,
        transforms::testing::test,
        visit::{as_folder, VisitMut},
    },
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
