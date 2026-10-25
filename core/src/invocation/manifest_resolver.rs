// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use crate::analysis::source_root_collector::SourceRootCollector;
use crate::invocation::config::Config;
use crate::invocation::package_context::PackageContext;
use crate::invocation::run_failure::RunFailure;
use crate::traits::workspace_reader::WorkspaceReader;
use crate::workspace::package_description::PackageDescription;
use crate::workspace::source_path::SourcePath;
use crate::workspace::target_description::TargetDescription;
use crate::workspace::workspace_description::WorkspaceDescription;

pub struct ManifestResolver<'a> {
    workspace_reader: &'a dyn WorkspaceReader,
}

impl<'a> ManifestResolver<'a> {
    pub fn new(workspace_reader: &'a dyn WorkspaceReader) -> Self {
        Self { workspace_reader }
    }

    pub fn is_production_relative_file(relative_file: &str) -> bool {
        !relative_file.starts_with("tests/")
            && !relative_file.starts_with("examples/")
            && !relative_file.starts_with("benches/")
            && relative_file != "build.rs"
    }

    pub fn is_production_target(target: &TargetDescription) -> bool {
        if target
            .kinds
            .iter()
            .any(|kind| matches!(kind.as_str(), "test" | "bench" | "example" | "custom-build"))
        {
            return false;
        }

        target.kinds.iter().any(|kind| {
            matches!(
                kind.as_str(),
                "lib" | "bin" | "proc-macro" | "rlib" | "dylib" | "cdylib" | "staticlib"
            )
        })
    }

    pub fn relative_file(base_dir: &SourcePath, file_path: &SourcePath) -> String {
        file_path
            .strip_prefix(base_dir)
            .unwrap_or(file_path.as_str())
            .replace('\\', "/")
    }

    pub fn resolve_packages(&self, config: &Config) -> Result<Vec<PackageContext>, RunFailure> {
        let workspace = self.workspace_reader.describe()?;
        let packages = Self::select_packages(&workspace, &config.packages)?;

        packages
            .into_iter()
            .map(Self::build_package_context)
            .collect()
    }

    pub fn select_packages<'w>(
        workspace: &'w WorkspaceDescription,
        requested: &[String],
    ) -> Result<Vec<&'w PackageDescription>, RunFailure> {
        if !requested.is_empty() {
            let mut selected = Vec::new();
            for package_name in requested {
                let package = workspace
                    .packages
                    .iter()
                    .find(|package| &package.name == package_name)
                    .ok_or_else(|| RunFailure::PackageNotFound(package_name.clone()))?;
                selected.push(package);
            }
            return Ok(selected);
        }

        workspace
            .root_package
            .as_ref()
            .and_then(|root| {
                workspace
                    .packages
                    .iter()
                    .find(|package| &package.name == root)
            })
            .map(|root| vec![root])
            .ok_or(RunFailure::NoRootPackage)
    }

    fn build_package_context(package: &PackageDescription) -> Result<PackageContext, RunFailure> {
        let manifest_dir = package
            .manifest_path
            .parent()
            .ok_or(RunFailure::NoParentDirectory)?;

        let mut collector = SourceRootCollector::new();
        collector.collect_from_targets(&package.targets);
        collector.ensure_fallback(&manifest_dir);

        Ok(PackageContext {
            name: package.name.clone(),
            manifest_dir,
            source_roots: collector.into_roots(),
        })
    }
}
