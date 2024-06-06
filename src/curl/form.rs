// This file is part of squared_api_wrapper <https://github.com/nevermille/squared_api_wrapper>
// Copyright (C) 2024 Camille Nevermind
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 3 of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program; if not, write to the Free Software Foundation,
// Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.

use curl::easy::Form;
use mime_guess::mime;
use std::path::Path;

/// Easily adds a string element to a form
///
/// # Parameters
///
/// * `form`: The form to edit
/// * `key`: The data key
/// * `value`: The string data to add
pub fn add_form_string_element(form: &mut Form, key: &str, value: &str) -> anyhow::Result<()> {
    let mut form_element = form.part(key);

    form_element.contents(value.as_bytes());
    form_element.add()?;

    Ok(())
}

/// Easily adds a binary element to a form
///
/// # Parameters
///
/// * `form`: The form to edit
/// * `key`: The data key
/// * `value`: The binary data to add
pub fn add_form_binary_element(form: &mut Form, key: &str, value: &[u8]) -> anyhow::Result<()> {
    let mut form_element = form.part(key);

    form_element.contents(value);
    form_element.add()?;

    Ok(())
}

/// Easily adds a file element to a form
///
/// # Parameters
///
/// * `form`: The form to edit
/// * `key`: The data key
/// * `file`: The file path to add
pub fn add_form_file_element(form: &mut Form, key: &str, file: &str) -> anyhow::Result<()> {
    let file_path = Path::new(file);
    let file_name = file_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let mime_type = mime_guess::from_path(file)
        .first()
        .unwrap_or(mime::APPLICATION_OCTET_STREAM)
        .to_string();

    let mut form_element = form.part(key);
    form_element.file(file);
    form_element.filename(&file_name);
    form_element.content_type(&mime_type);
    form_element.add()?;

    Ok(())
}
