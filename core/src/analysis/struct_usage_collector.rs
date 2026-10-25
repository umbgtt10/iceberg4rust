// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::{String, ToString};

use syn::visit::{Visit, visit_expr_call, visit_expr_struct, visit_type_path};
use syn::{Block, Expr, ExprCall, ExprStruct, Path, ReturnType, Signature, TypePath};

use crate::analysis::helper_struct_kind::HelperStructKind;

pub struct StructUsageCollector<'a> {
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

    fn record_if_private_struct(&mut self, path: &Path) {
        if let Some(segment) = path.segments.last() {
            let ident = segment.ident.to_string();
            if self.helper_structs.contains_key(&ident) {
                self.found.insert(ident);
            }
        }
    }
}

impl<'ast, 'a> Visit<'ast> for StructUsageCollector<'a> {
    fn visit_type_path(&mut self, node: &'ast TypePath) {
        self.record_if_private_struct(&node.path);
        visit_type_path(self, node);
    }

    fn visit_expr_struct(&mut self, node: &'ast ExprStruct) {
        self.record_if_private_struct(&node.path);
        visit_expr_struct(self, node);
    }

    fn visit_expr_call(&mut self, node: &'ast ExprCall) {
        if let Expr::Path(expr_path) = node.func.as_ref() {
            self.record_if_private_struct(&expr_path.path);
        }
        visit_expr_call(self, node);
    }

    fn visit_return_type(&mut self, node: &'ast ReturnType) {
        if let ReturnType::Type(_, ty) = node {
            self.visit_type(ty);
        }
    }
}
