//
// Copyright (c) 2024 Mohamed Seleem <oss@mselee.com>.
//
// This file is part of dnsaur.
// See https://github.com/mselee/dnsaur for further info.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//

use file_header::add_headers_recursively;
use file_header::license::spdx::license::License;
use file_header::license::spdx::{license, LicenseTokens, SpdxLicense};
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::{env, path::PathBuf};

pub struct MPL2Tokens;

pub struct YearCopyrightRepoValue {
    year: u32,
    authors: String,
    project: String,
    repository: String,
}

impl YearCopyrightRepoValue {
    fn new(year: u32, authors: String, project: String, repository: String) -> Self {
        Self {
            year,
            authors,
            project,
            repository,
        }
    }
}

impl LicenseTokens for MPL2Tokens {
    type TokenReplacementValues = YearCopyrightRepoValue;

    fn replacement_pairs(
        replacements: Self::TokenReplacementValues,
    ) -> Vec<(&'static str, String)> {
        vec![
            ("<yyyy>", replacements.year.to_string()),
            ("<authors>", replacements.authors),
            ("<project>", replacements.project),
            ("<repo>", replacements.repository),
        ]
    }
}

struct MPL2WithCopy;
impl License for MPL2WithCopy {
    fn id(&self) -> &'static str {
        license::licenses::Mpl2_0.id()
    }

    fn name(&self) -> &'static str {
        license::licenses::Mpl2_0.name()
    }

    fn text(&self) -> &'static str {
        license::licenses::Mpl2_0.text()
    }

    fn header(&self) -> Option<&'static str> {
        Some("\nCopyright (c) <yyyy> <authors>.\n\nThis file is part of <project>.\nSee <repo> for further info.\n\nThis Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.\nIf a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.\n")
    }

    fn is_osi_approved(&self) -> bool {
        license::licenses::Mpl2_0.is_osi_approved()
    }

    fn is_fsf_libre(&self) -> bool {
        license::licenses::Mpl2_0.is_fsf_libre()
    }

    fn is_deprecated(&self) -> bool {
        license::licenses::Mpl2_0.is_deprecated()
    }

    fn comments(&self) -> Option<&'static str> {
        license::licenses::Mpl2_0.comments()
    }

    fn see_also(&self) -> &'static [&'static str] {
        license::licenses::Mpl2_0.see_also()
    }
}

fn main() {
    let rust_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let authors = env::var("CARGO_PKG_AUTHORS").unwrap();
    let project = env::var("CARGO_PKG_NAME").unwrap();
    let repository = env::var("CARGO_PKG_REPOSITORY").unwrap();
    let ignore_globset = ignore_globset().unwrap();
    let license = SpdxLicense::<MPL2Tokens>::new(
        Box::new(MPL2WithCopy),
        "Mozilla Public License, v. 2.0".to_string(),
        10,
    );

    let header = license.build_header(YearCopyrightRepoValue::new(
        2024, authors, project, repository,
    ));
    let files_with_new_header =
        add_headers_recursively(&rust_dir, |p| !ignore_globset.is_match(p), header).unwrap();
    files_with_new_header
        .iter()
        .for_each(|path| println!("Added header to: {path:?}"));
}

fn ignore_globset() -> Result<GlobSet, globset::Error> {
    Ok(GlobSetBuilder::new()
        .add(Glob::new("**/.idea/**")?)
        .add(Glob::new("**/.git/**")?)
        .add(Glob::new("**/target/**")?)
        .add(Glob::new("**/.gitignore")?)
        .add(Glob::new("**/CHANGELOG.md")?)
        .add(Glob::new("**/LICENSE")?)
        .add(Glob::new("**/.github/**")?)
        .add(Glob::new("**/mise.toml")?)
        .add(Glob::new("**/Cargo.lock")?)
        .add(Glob::new("**/Cargo.toml")?)
        .add(Glob::new("**/README.md")?)
        .add(Glob::new("*.bin")?)
        .build()?)
}
