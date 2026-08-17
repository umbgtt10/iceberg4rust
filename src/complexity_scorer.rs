// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use syn::{
    Block, Expr, ExprBinary, ExprBlock, ExprCall, ExprForLoop, ExprIf, ExprLoop, ExprMatch,
    ExprWhile, ImplItem, Item, ItemImpl, Stmt,
};

#[derive(Default)]
pub struct ComplexityScorer;

impl ComplexityScorer {
    pub fn new() -> Self {
        Self
    }

    pub fn score(&self, block: &Block) -> u32 {
        self.score_block(block, 0)
    }

    fn score_block(&self, block: &Block, nesting: u32) -> u32 {
        block
            .stmts
            .iter()
            .map(|stmt| self.score_stmt(stmt, nesting))
            .sum()
    }

    fn score_stmt(&self, stmt: &Stmt, nesting: u32) -> u32 {
        match stmt {
            Stmt::Item(item) => self.score_item(item, nesting),
            Stmt::Expr(expr, _) => self.score_expr(expr, nesting),
            Stmt::Local(local) => local
                .init
                .as_ref()
                .map_or(0, |init| self.score_expr(&init.expr, nesting)),
            Stmt::Macro(_) => 0,
        }
    }

    fn score_item(&self, item: &Item, nesting: u32) -> u32 {
        match item {
            Item::Fn(item_fn) => self.score_block(&item_fn.block, nesting),
            Item::Mod(item_mod) => item_mod.content.as_ref().map_or(0, |(_, items)| {
                items
                    .iter()
                    .map(|item| self.score_item(item, nesting))
                    .sum()
            }),
            Item::Impl(ItemImpl { items, .. }) => items
                .iter()
                .map(|item| match item {
                    ImplItem::Fn(method) => self.score_block(&method.block, nesting),
                    _ => 0,
                })
                .sum(),
            _ => 0,
        }
    }

    fn score_expr(&self, expr: &Expr, nesting: u32) -> u32 {
        match expr {
            Expr::If(ExprIf {
                cond,
                then_branch,
                else_branch,
                ..
            }) => {
                let mut score = 1 + nesting + self.score_expr(cond, nesting);
                score += self.score_block(then_branch, nesting + 1);
                if let Some((_, else_branch)) = else_branch {
                    score += self.score_expr(else_branch, nesting + 1);
                }
                score
            }
            Expr::Match(ExprMatch { expr, arms, .. }) => {
                1 + nesting
                    + self.score_expr(expr, nesting)
                    + arms
                        .iter()
                        .map(|arm| self.score_expr(&arm.body, nesting + 1))
                        .sum::<u32>()
            }
            Expr::ForLoop(ExprForLoop { expr, body, .. }) => {
                1 + nesting + self.score_expr(expr, nesting) + self.score_block(body, nesting + 1)
            }
            Expr::While(ExprWhile { cond, body, .. }) => {
                1 + nesting + self.score_expr(cond, nesting) + self.score_block(body, nesting + 1)
            }
            Expr::Loop(ExprLoop { body, .. }) => 1 + nesting + self.score_block(body, nesting + 1),
            Expr::Block(ExprBlock { block, .. }) => self.score_block(block, nesting),
            Expr::Binary(ExprBinary { left, right, .. }) => {
                self.score_expr(left, nesting) + self.score_expr(right, nesting)
            }
            Expr::Call(ExprCall { func, args, .. }) => {
                self.score_expr(func, nesting)
                    + args
                        .iter()
                        .map(|arg| self.score_expr(arg, nesting))
                        .sum::<u32>()
            }
            _ => 0,
        }
    }
}
