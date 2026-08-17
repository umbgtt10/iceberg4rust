// Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use cargo_metadata::{Metadata, MetadataCommand, Package, Target};

use crate::config::Config;
use crate::package_context::PackageContext;
use crate::source_root_collector::SourceRootCollector;

pub struct ManifestResolver {
    config: Config,
}

impl ManifestResolver {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn resolve_packages(&self) -> Result<Vec<PackageContext>> {
        let mut command = MetadataCommand::new();
        command.no_deps();
        if let Some(manifest_path) = &self.config.manifest_path {
            command.manifest_path(manifest_path);
        }

        let metadata = command.exec().context("failed to read Cargo metadata")?;
        let packages = Self::select_packages(&metadata, &self.config.packages)?;

        packages
            .into_iter()
            .map(Self::build_package_context)
            .collect()
    }

    fn build_package_context(package: &Package) -> Result<PackageContext> {
        let manifest_dir = package
            .manifest_path
            .clone()
            .into_std_path_buf()
            .parent()
            .map(PathBuf::from)
            .context("package manifest has no parent directory")?;

        let mut collector = SourceRootCollector::new();
        collector.collect_from_targets(&package.targets);
        collector.ensure_fallback(&manifest_dir);

        Ok(PackageContext {
            name: package.name.to_string(),
            manifest_dir,
            source_roots: collector.into_roots(),
        })
    }

    pub fn is_production_target(target: &Target) -> bool {
        let kinds = target
            .kind
            .iter()
            .map(|kind| kind.to_string())
            .collect::<Vec<_>>();

        if kinds
            .iter()
            .any(|kind| matches!(kind.as_str(), "test" | "bench" | "example" | "custom-build"))
        {
            return false;
        }

        kinds.iter().any(|kind| {
            matches!(
                kind.as_str(),
                "lib" | "bin" | "proc-macro" | "rlib" | "dylib" | "cdylib" | "staticlib"
            )
        })
    }

    pub fn select_packages<'a>(
        metadata: &'a Metadata,
        requested: &[String],
    ) -> Result<Vec<&'a Package>> {
        if !requested.is_empty() {
            let mut selected = Vec::new();
            for package_name in requested {
                let package = metadata
                    .packages
                    .iter()
                    .find(|package| package.name == package_name)
                    .with_context(|| {
                        format!("package {package_name} was not found in the manifest")
                    })?;
                selected.push(package);
            }
            return Ok(selected);
        }

        if let Some(root) = metadata.root_package() {
            return Ok(vec![root]);
        }

        bail!("manifest contains multiple packages; pass --package <name>")
    }

    pub fn relative_file(base_dir: &Path, file_path: &Path) -> String {
        file_path
            .strip_prefix(base_dir)
            .unwrap_or(file_path)
            .to_string_lossy()
            .replace('\\', "/")
    }

    pub fn is_production_relative_file(relative_file: &str) -> bool {
        !relative_file.starts_with("tests/")
            && !relative_file.starts_with("examples/")
            && !relative_file.starts_with("benches/")
            && relative_file != "build.rs"
    }
}
