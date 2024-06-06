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

use crate::response::BinaryResponse;
use curl::easy::{Easy, Form};
use std::io::Read;

/// Executes an easy request
///
/// # Parameters
///
/// * `curl`: The request to execute
pub fn curl_easy_execute(curl: &mut Easy, data: Option<&[u8]>) -> anyhow::Result<BinaryResponse> {
    let mut response: Vec<u8> = Vec::new(); // That's where the response will be written on
    let mut raw_data = data.unwrap_or_default();

    {
        // We need this for lifetime reasons
        let mut transfer = curl.transfer();

        // How we pass data to mailjet
        if !raw_data.is_empty() {
            transfer.read_function(|buffer| Ok(raw_data.read(buffer).unwrap_or_default()))?
        };

        // How we read mailjet's response
        transfer.write_function(|buffer| {
            let _ = &response.extend_from_slice(buffer);
            Ok(buffer.len())
        })?;

        // Request execution
        transfer.perform()?;
    }

    Ok(BinaryResponse {
        http_code: curl.response_code().ok().into(),
        raw_data: response,
    })
}

/// Executes an HTTP easy request with a GET
///
/// # Parameters
///
/// * `curl`: The request to execute
pub fn get(curl: &mut Easy) -> anyhow::Result<BinaryResponse> {
    curl_easy_execute(curl, None)
}

/// Executes an HTTP easy request with a POST
///
/// # Parameters
///
/// * `curl`: The request to execute
/// * `data`: The data to send
/// * `form`: The multipart POST form
pub fn post(
    curl: &mut Easy,
    data: Option<&[u8]>,
    form: Option<Form>,
) -> anyhow::Result<BinaryResponse> {
    curl.post(true)?;

    if let Some(v) = form {
        curl.httppost(v)?;
    }

    curl_easy_execute(curl, data)
}

/// Executes an HTTP easy request with a DELETE
///
/// # Parameters
///
/// * `curl`: The request to execute
/// * `data`: The data to send
pub fn put(curl: &mut Easy, data: Option<&[u8]>) -> anyhow::Result<BinaryResponse> {
    curl.put(true)?;
    curl_easy_execute(curl, data)
}

/// Executes an HTTP easy request with a DELETE
///
/// # Parameters
///
/// * `curl`: The request to execute
pub fn delete(curl: &mut Easy) -> anyhow::Result<BinaryResponse> {
    curl.custom_request("DELETE")?;
    curl_easy_execute(curl, None)
}

/// Adds basic authentification to request
///
/// * `curl`: The request to edit
/// * `username`: The username
/// * `password`: The password
pub fn basic_auth(curl: &mut Easy, username: &str, password: &str) -> anyhow::Result<()> {
    curl.username(username)?;
    curl.password(password)?;

    Ok(())
}
