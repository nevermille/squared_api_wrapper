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

use curl::easy::List;

/// Adds a header in the list
///
/// # Parameters
///
/// * `headers_list`: The list where you add
/// * `key`: The header key
/// * `value`: The header value
pub fn add_header(headers_list: &mut List, key: &str, value: &str) -> anyhow::Result<()> {
    headers_list.append(&format!("{}: {}", key, value))?;

    Ok(())
}
