// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use syn::Attribute;

#[derive(Default)]
pub struct TestAttrChecker;

impl TestAttrChecker {
    pub fn new() -> Self {
        Self
    }

    pub fn has_test_attrs(&self, attrs: &[Attribute]) -> bool {
        attrs.iter().any(|attr| self.has_test_attr(attr))
    }

    fn has_test_attr(&self, attr: &Attribute) -> bool {
        if attr.path().is_ident("test") {
            return true;
        }

        let mut found = false;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("test")
                || meta
                    .path
                    .segments
                    .last()
                    .is_some_and(|segment| segment.ident == "test")
            {
                found = true;
            }
            Ok(())
        });
        found
    }
}
