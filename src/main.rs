// Copyright 2019 Henri Sivonen. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn main() -> std::io::Result<()> {
    let mut removals = Vec::new();
    for entry in std::fs::read_dir(".")? {
        let dir = entry?;
        let path = dir.path();
        if let Some(ext) = path.extension() {
            let raw_ext = if ext == "JPG" {
                "RW2"
            } else if ext == "jpg" {
                "dng"
            } else {
                continue;
            };
            let raw_path = path.with_extension(raw_ext);
            if raw_path.exists() {
                removals.push(path);
            }
        }
    }
    for path in removals {
        std::fs::remove_file(path)?;
    }
    Ok(())
}
