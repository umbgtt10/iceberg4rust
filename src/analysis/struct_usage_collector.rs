// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::{BTreeMap, BTreeSet};

use syn::visit::{Visit, visit_expr_call, visit_expr_struct, visit_type_path};
use syn::{Block, Expr, ExprCall, ExprStruct, ReturnType, Signature, TypePath};

use crate::analysis::helper_struct_kind::HelperStructKind;

pub fn collect_used_private_structs(
    signature: &Signature,
    block: &Block,
    helper_structs: &BTreeMap<String, HelperStructKind>,
) -> BTreeSet<String> {
    let mut collector = StructUsageCollector::new(helper_structs);
    collector.visit_signature(signature);
    collector.visit_block(block);
    collector.found
}

struct StructUsageCollector<'a> {
    helper_structs: &'a BTreeMap<String, HelperStructKind>,
    found: BTreeSet<String>,
}

impl<'a> StructUsageCollector<'a> {
    fn new(helper_structs: &'a BTreeMap<String, HelperStructKind>) -> Self {
        Self {
            helper_structs,
            found: BTreeSet::new(),
        }
    }

    fn record_if_private_struct(&mut self, ident: &str) {
        if self.helper_structs.contains_key(ident) {
            self.found.insert(ident.to_string());
        }
    }
}

impl<'ast, 'a> Visit<'ast> for StructUsageCollector<'a> {
    fn visit_type_path(&mut self, node: &'ast TypePath) {
        if let Some(segment) = node.path.segments.last() {
            self.record_if_private_struct(&segment.ident.to_string());
        }
        visit_type_path(self, node);
    }

    fn visit_expr_struct(&mut self, node: &'ast ExprStruct) {
        if let Some(segment) = node.path.segments.last() {
            self.record_if_private_struct(&segment.ident.to_string());
        }
        visit_expr_struct(self, node);
    }

    fn visit_expr_call(&mut self, node: &'ast ExprCall) {
        if let Expr::Path(expr_path) = node.func.as_ref()
            && let Some(segment) = expr_path.path.segments.last()
        {
            self.record_if_private_struct(&segment.ident.to_string());
        }
        visit_expr_call(self, node);
    }

    fn visit_return_type(&mut self, node: &'ast ReturnType) {
        if let ReturnType::Type(_, ty) = node {
            self.visit_type(ty);
        }
    }
}
