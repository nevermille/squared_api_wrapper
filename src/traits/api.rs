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

use curl::easy::{Easy, List};

/// The base functions of an API wrapper
pub trait Api {
    /// Returns an Easy object preconfigured for every route
    fn get_easy_base(&self) -> Easy;

    /// Returns a headers list with data to use for every route
    fn get_headers_base(&self) -> List;

    /// Returns an Easy object and a headers list with data to use for every route
    fn get_base_data(&self) -> (Easy, List) {
        (self.get_easy_base(), self.get_headers_base())
    }

    /// Returns the root url to use for every route
    fn get_root_url(&self) -> String;
}
