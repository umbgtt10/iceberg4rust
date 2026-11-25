// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use syn::visit::{Visit, visit_expr_closure};
use syn::{Block, ExprClosure, ExprForLoop, ExprIf, ExprLoop, ExprMatch, ExprWhile};

#[derive(Default)]
pub struct ComplexityScorer {
    nesting: u32,
    total: u32,
}

impl ComplexityScorer {
    pub fn new() -> Self {
        Self {
            nesting: 0,
            total: 0,
        }
    }

    pub fn score(mut self, block: &Block) -> u32 {
        self.visit_block(block);
        self.total
    }

    fn count(&mut self) {
        self.total += 1 + self.nesting;
    }

    fn count_with_body(&mut self, body: &Block) {
        self.count();
        self.enter();
        self.visit_block(body);
        self.leave();
    }

    fn enter(&mut self) {
        self.nesting += 1;
    }

    fn leave(&mut self) {
        self.nesting -= 1;
    }
}

impl<'ast> Visit<'ast> for ComplexityScorer {
    fn visit_expr_closure(&mut self, node: &'ast ExprClosure) {
        self.enter();
        visit_expr_closure(self, node);
        self.leave();
    }

    fn visit_expr_for_loop(&mut self, node: &'ast ExprForLoop) {
        self.visit_expr(&node.expr);
        self.count_with_body(&node.body);
    }

    fn visit_expr_if(&mut self, node: &'ast ExprIf) {
        self.visit_expr(&node.cond);
        self.count_with_body(&node.then_branch);
        if let Some((_, else_branch)) = &node.else_branch {
            self.enter();
            self.visit_expr(else_branch);
            self.leave();
        }
    }

    fn visit_expr_loop(&mut self, node: &'ast ExprLoop) {
        self.count_with_body(&node.body);
    }

    fn visit_expr_match(&mut self, node: &'ast ExprMatch) {
        self.count();
        self.visit_expr(&node.expr);
        self.enter();
        for arm in &node.arms {
            self.visit_arm(arm);
        }
        self.leave();
    }

    fn visit_expr_while(&mut self, node: &'ast ExprWhile) {
        self.visit_expr(&node.cond);
        self.count_with_body(&node.body);
    }
}
