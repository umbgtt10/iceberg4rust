// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::BTreeMap;

use syn::{ImplItem, Item, ItemImpl, ItemMod, Type, Visibility};

use crate::helper_struct_kind::HelperStructKind;
use crate::test_attr_checker::TestAttrChecker;

pub struct PrivateHelperClassifier {
    helper_structs: BTreeMap<String, HelperStructKind>,
    checker: TestAttrChecker,
}

impl PrivateHelperClassifier {
    pub fn new(items: &[Item], primary_struct: Option<&str>) -> Self {
        let mut classifier = Self {
            helper_structs: BTreeMap::new(),
            checker: TestAttrChecker::new(),
        };
        classifier.collect_structs(items, primary_struct);
        classifier.upgrade_impls(items);
        classifier
    }

    pub fn into_map(self) -> BTreeMap<String, HelperStructKind> {
        self.helper_structs
    }

    fn collect_structs(&mut self, items: &[Item], primary_struct: Option<&str>) {
        for item in items {
            match item {
                Item::Struct(item_struct) => {
                    self.try_insert_data_struct(item_struct, primary_struct);
                }
                Item::Mod(item_mod) => {
                    self.recurse_collect_structs(item_mod, primary_struct);
                }
                _ => {}
            }
        }
    }

    fn try_insert_data_struct(
        &mut self,
        item_struct: &syn::ItemStruct,
        primary_struct: Option<&str>,
    ) {
        let name = item_struct.ident.to_string();
        if matches!(item_struct.vis, Visibility::Inherited)
            && primary_struct.is_none_or(|primary| primary != name)
        {
            self.helper_structs.insert(name, HelperStructKind::Data);
        }
    }

    fn recurse_collect_structs(&mut self, item_mod: &ItemMod, primary_struct: Option<&str>) {
        if self.checker.has_test_attrs(&item_mod.attrs) {
            return;
        }

        if let Some((_, inline_items)) = &item_mod.content {
            self.collect_structs(inline_items, primary_struct);
        }
    }

    fn upgrade_impls(&mut self, items: &[Item]) {
        for item in items {
            match item {
                Item::Impl(item_impl) => self.try_upgrade_impl(item_impl),
                Item::Mod(item_mod) => self.recurse_upgrade_impls(item_mod),
                _ => {}
            }
        }
    }

    fn try_upgrade_impl(&mut self, item_impl: &ItemImpl) {
        if self.checker.has_test_attrs(&item_impl.attrs) {
            return;
        }

        let Some(target_name) = impl_target_name(item_impl) else {
            return;
        };
        if !self.helper_structs.contains_key(&target_name) {
            return;
        }

        let has_methods = item_impl.items.iter().any(
            |impl_item| matches!(impl_item, ImplItem::Fn(method) if !self.checker.has_test_attrs(&method.attrs)),
        );
        if has_methods {
            self.helper_structs
                .insert(target_name, HelperStructKind::Behavioral);
        }
    }

    fn recurse_upgrade_impls(&mut self, item_mod: &ItemMod) {
        if self.checker.has_test_attrs(&item_mod.attrs) {
            return;
        }

        if let Some((_, inline_items)) = &item_mod.content {
            self.upgrade_impls(inline_items);
        }
    }
}

fn impl_target_name(item_impl: &ItemImpl) -> Option<String> {
    let Type::Path(type_path) = item_impl.self_ty.as_ref() else {
        return None;
    };

    type_path
        .path
        .segments
        .last()
        .map(|segment| segment.ident.to_string())
}
