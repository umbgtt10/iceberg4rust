// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

use iceberg4rust::traits::source_tree::SourceTree;
use iceberg4rust::workspace::source_path::SourcePath;
use iceberg4rust::workspace::source_read_failure::SourceReadFailure;
use walkdir::{DirEntry, WalkDir};

use crate::system::portable_path::PortablePath;

// The source tree on disk. A root that does not exist holds no files, which is
// what a package with no `src/` should score. A Rust file whose path is not
// UTF-8 cannot become a `SourcePath`, so it fails the listing by name rather
// than being skipped unscored.
#[derive(Debug, Clone, Copy, Default)]
pub struct WalkdirSourceTree;

impl WalkdirSourceTree {
    pub fn new() -> Self {
        Self
    }

    fn source_path(entry: &DirEntry) -> Result<SourcePath, SourceReadFailure> {
        PortablePath::of(entry.path()).ok_or_else(|| SourceReadFailure {
            path: PortablePath::of_text(&entry.path().to_string_lossy()),
            cause: Box::new(Error::new(
                ErrorKind::InvalidData,
                "the path is not valid UTF-8",
            )),
        })
    }
}

impl SourceTree for WalkdirSourceTree {
    fn read_source(&self, file: &SourcePath) -> Result<String, SourceReadFailure> {
        fs::read_to_string(Path::new(file.as_str())).map_err(|error| SourceReadFailure {
            path: file.clone(),
            cause: Box::new(error),
        })
    }

    fn rust_files(&self, root: &SourcePath) -> Result<Vec<SourcePath>, SourceReadFailure> {
        WalkDir::new(root.as_str())
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .is_some_and(|extension| extension == "rs")
            })
            .map(|entry| Self::source_path(&entry))
            .collect()
    }
}
