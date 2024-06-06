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

use crate::response::StringObjectResponse;
use http_codex::HttpCode;

/// A response from a request with string raw data
#[derive(Default)]
/// A response from a request
pub struct StringResponse {
    /// The HTTP Response code
    pub http_code: HttpCode,

    /// The data as directly returned by Mailjet
    pub raw_data: String,
}

impl StringResponse {
    /// Creates a new object
    ///
    /// # Parameters
    ///
    /// * `http_code`: The HTTP response code if supported
    /// * `raw_data` : The response body
    pub fn create_from_data(http_code: HttpCode, raw_data: String) -> Self {
        Self {
            http_code,
            raw_data,
        }
    }

    /// Adds an object to the response
    ///
    /// # Parameters
    ///
    /// `object`: The object to add
    pub fn add_object<T>(self, object: T) -> StringObjectResponse<T> {
        StringObjectResponse {
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
    pub fn add_optional_object<T>(self, object: Option<T>) -> StringObjectResponse<T> {
        StringObjectResponse {
            http_code: self.http_code,
            raw_data: self.raw_data,
            object,
        }
    }
}
