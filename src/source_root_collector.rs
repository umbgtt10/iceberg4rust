// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use cargo_metadata::Target;

use crate::manifest_resolver::ManifestResolver;

#[derive(Default)]
pub struct SourceRootCollector {
    source_roots: BTreeSet<PathBuf>,
}

impl SourceRootCollector {
    pub fn new() -> Self {
        Self {
            source_roots: BTreeSet::new(),
        }
    }

    pub fn collect_from_targets(&mut self, targets: &[Target]) {
        for target in targets {
            self.try_insert_root(target);
        }
    }

    fn try_insert_root(&mut self, target: &Target) {
        if !ManifestResolver::is_production_target(target) {
            return;
        }

        if target
            .src_path
            .extension()
            .is_none_or(|extension| extension != "rs")
        {
            return;
        }

        let path = target.src_path.clone().into_std_path_buf();
        if let Some(parent) = path.parent() {
            self.source_roots.insert(parent.to_path_buf());
        }
    }

    pub fn ensure_fallback(&mut self, manifest_dir: &Path) {
        if self.source_roots.is_empty() {
            self.source_roots.insert(manifest_dir.join("src"));
        }
    }

    pub fn into_roots(self) -> Vec<PathBuf> {
        self.source_roots.into_iter().collect()
    }
}
