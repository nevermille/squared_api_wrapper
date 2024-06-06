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

use crate::response::{BinaryObjectResponse, StringResponse};
use http_codex::HttpCode;

#[derive(Default)]
/// A response from a request with binary raw data
pub struct BinaryResponse {
    /// The HTTP Response code
    pub http_code: HttpCode,

    /// The data as directly returned by Mailjet
    pub raw_data: Vec<u8>,
}

impl BinaryResponse {
    /// Creates a new object
    ///
    /// # Parameters
    ///
    /// * `http_code`: The HTTP response code if supported
    /// * `raw_data` : The response body
    /// * `object` : The parsed object if parsing was successful
    pub fn create_from_data(http_code: HttpCode, raw_data: &[u8]) -> Self {
        Self {
            http_code,
            raw_data: raw_data.to_vec(),
        }
    }

    /// Creates a version of the response where `raw_data` is a string
    pub fn to_string_response(&self) -> StringResponse {
        StringResponse {
            http_code: self.http_code,
            raw_data: String::from_utf8_lossy(&self.raw_data).to_string(),
        }
    }

    /// Adds an object to the response
    ///
    /// # Parameters
    ///
    /// `object`: The object to add
    pub fn add_object<T>(self, object: T) -> BinaryObjectResponse<T> {
        BinaryObjectResponse {
            http_code: self.http_code,
            raw_data: self.raw_data,
            object: Some(object),
        }
    }

    /// Adds an object inside an `Option` to the response
    ///
    /// # Parameters
    ///
    /// `object`: The object to add
    pub fn add_optional_object<T>(self, object: Option<T>) -> BinaryObjectResponse<T> {
        BinaryObjectResponse {
            http_code: self.http_code,
            raw_data: self.raw_data,
            object,
        }
    }
}
