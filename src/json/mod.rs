// Copyright (C) 2019 RoccoDev
// 
// This file is part of pj.
// 
// pj is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// pj is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with pj.  If not, see <http://www.gnu.org/licenses/>.

use serde_json::{Result, Value};

pub fn parse(input: String) -> Result<()> {
    let parsed: Value = serde_json::from_str(&input.as_str())?;

    Ok(())
}