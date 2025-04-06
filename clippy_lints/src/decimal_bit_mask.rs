use clippy_utils::diagnostics::span_lint;
use clippy_utils::source::snippet_opt;
use rustc_hir::*;
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::declare_lint_pass;
use rustc_span::source_map::Spanned;

declare_clippy_lint! {
    /// ### What it does
    ///
    /// ### Why is this bad?
    ///
    /// ### Example
    /// ```no_run
    /// // example code where clippy issues a warning
    /// ```
    /// Use instead:
    /// ```no_run
    /// // example code which does not raise clippy warning
    /// ```
    #[clippy::version = "1.87.0"]
    pub DECIMAL_BIT_MASK,
    nursery,
    "default lint description"
}

declare_lint_pass!(DecimalBitMask => [DECIMAL_BIT_MASK]);

impl<'tcx> LateLintPass<'tcx> for DecimalBitMask {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, e: &'tcx Expr<'_>) {
        if let ExprKind::Binary(
            Spanned {
                node: BinOpKind::BitAnd | BinOpKind::BitOr | BinOpKind::BitXor,
                ..
            },
            Expr {
                kind: kind1,
                span: span1,
                ..
            },
            Expr {
                kind: kind2,
                span: span2,
                ..
            },
        ) = &e.kind
        {
            if let ExprKind::Lit(_) = kind1 {
                if let Some(snippet) = snippet_opt(cx, *span1)
                    && !snippet.starts_with("0b")
                    && !snippet.starts_with("0x")
                {
                    span_lint(cx, DECIMAL_BIT_MASK, e.span, "Decimal bit mask");
                }
            }

            if let ExprKind::Lit(_) = kind2 {
                if let Some(snippet) = snippet_opt(cx, *span2)
                    && !snippet.starts_with("0b")
                    && !snippet.starts_with("0x")
                {
                    span_lint(cx, DECIMAL_BIT_MASK, e.span, "Decimal bit mask");
                }
            }
        }
        if let ExprKind::AssignOp(
            Spanned {
                node: BinOpKind::BitAnd | BinOpKind::BitOr | BinOpKind::BitXor,
                ..
            },
            _,
            Expr {
                kind: ExprKind::Lit(_),
                span,
                ..
            },
        ) = &e.kind
        {
            if let Some(snippet) = snippet_opt(cx, *span)
                && !snippet.starts_with("0b")
                && !snippet.starts_with("0x")
            {
                span_lint(cx, DECIMAL_BIT_MASK, e.span, "Decimal bit mask");
            }
        }
    }
}
