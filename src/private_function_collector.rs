// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::BTreeMap;

use syn::{
    Attribute, Block, ImplItem, ImplItemFn, Item, ItemFn, ItemImpl, ItemMod, Signature, Visibility,
};

use crate::complexity_scorer::ComplexityScorer;
use crate::helper_struct_kind::HelperStructKind;
use crate::private_function_metrics::PrivateFunctionMetrics;
use crate::struct_usage_collector::collect_used_private_structs;
use crate::test_attr_checker::TestAttrChecker;

pub struct PrivateFunctionCollector<'a> {
    helper_structs: &'a BTreeMap<String, HelperStructKind>,
    functions: Vec<PrivateFunctionMetrics>,
}

impl<'a> PrivateFunctionCollector<'a> {
    pub fn new(helper_structs: &'a BTreeMap<String, HelperStructKind>) -> Self {
        Self {
            helper_structs,
            functions: Vec::new(),
        }
    }

    pub fn collect(mut self, items: &[Item]) -> Vec<PrivateFunctionMetrics> {
        self.process_items(items);
        self.functions
    }

    fn process_items(&mut self, items: &[Item]) {
        for item in items {
            self.process_item(item);
        }
    }

    fn process_item(&mut self, item: &Item) {
        match item {
            Item::Fn(item_fn) => self.process_fn(item_fn),
            Item::Impl(item_impl) => self.process_impl(item_impl),
            Item::Mod(item_mod) => self.process_mod(item_mod),
            _ => {}
        }
    }

    fn process_fn(&mut self, item_fn: &ItemFn) {
        if !is_private_item(&item_fn.vis, &item_fn.attrs) {
            return;
        }
        self.push_function(&item_fn.sig, &item_fn.block, true);
    }

    fn process_impl(&mut self, item_impl: &ItemImpl) {
        let checker = TestAttrChecker::new();
        if checker.has_test_attrs(&item_impl.attrs) {
            return;
        }

        // A trait impl method is reachable by anyone holding the trait, so it is
        // not hidden implementation however little visibility syntax it carries.
        // It is still collected: its complexity is real burden, and a wide trait
        // implemented with empty bodies should cost nothing while a heavy `next`
        // or `poll` should still register.
        let is_hidden = item_impl.trait_.is_none();

        for item in &item_impl.items {
            if let ImplItem::Fn(method) = item {
                self.process_impl_method(method, is_hidden);
            }
        }
    }

    fn process_impl_method(&mut self, method: &ImplItemFn, is_hidden: bool) {
        if !is_private_item(&method.vis, &method.attrs) {
            return;
        }
        self.push_function(&method.sig, &method.block, is_hidden);
    }

    fn process_mod(&mut self, item_mod: &ItemMod) {
        let checker = TestAttrChecker::new();
        if checker.has_test_attrs(&item_mod.attrs) {
            return;
        }

        if let Some((_, inline_items)) = &item_mod.content {
            self.process_items(inline_items);
        }
    }

    fn push_function(&mut self, sig: &Signature, block: &Block, is_hidden: bool) {
        let scorer = ComplexityScorer::new();
        self.functions.push(PrivateFunctionMetrics {
            name: sig.ident.to_string(),
            line: sig.ident.span().start().line,
            complexity: scorer.score(block),
            used_private_structs: collect_used_private_structs(sig, block, self.helper_structs),
            is_hidden,
        });
    }
}

// Only `pub` escapes. Every restricted form -- pub(crate), pub(super),
// pub(in path), pub(self) -- is unreachable from outside the crate, so it is
// hidden implementation exactly as a bare `fn` is. Restricting visibility is not
// a way out of the measurement; publishing or relocating the behaviour is.
fn is_private_item(vis: &Visibility, attrs: &[Attribute]) -> bool {
    let checker = TestAttrChecker::new();
    matches!(vis, Visibility::Inherited | Visibility::Restricted(_))
        && !checker.has_test_attrs(attrs)
}
