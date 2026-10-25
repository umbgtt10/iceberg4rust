// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::collections::BTreeSet;
use alloc::vec::Vec;

use crate::invocation::manifest_resolver::ManifestResolver;
use crate::workspace::source_path::SourcePath;
use crate::workspace::target_description::TargetDescription;

#[derive(Default)]
pub struct SourceRootCollector {
    source_roots: BTreeSet<SourcePath>,
}

impl SourceRootCollector {
    pub fn new() -> Self {
        Self {
            source_roots: BTreeSet::new(),
        }
    }

    pub fn collect_from_targets(&mut self, targets: &[TargetDescription]) {
        for target in targets {
            self.try_insert_root(target);
        }
    }

    pub fn ensure_fallback(&mut self, manifest_dir: &SourcePath) {
        if self.source_roots.is_empty() {
            self.source_roots.insert(manifest_dir.join("src"));
        }
    }

    pub fn into_roots(self) -> Vec<SourcePath> {
        self.source_roots.into_iter().collect()
    }

    fn try_insert_root(&mut self, target: &TargetDescription) {
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

        if let Some(parent) = target.src_path.parent() {
            self.source_roots.insert(parent);
        }
    }
}
