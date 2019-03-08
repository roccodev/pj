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

use serde_json::{map, Result, Value};

pub fn parse(input: String) -> Result<Vec<(String, String)>> {
    /* Parse the input string as JSON */
    let parsed: Value = serde_json::from_str(&input.as_str())?;
    let empty_root = String::new();

    let mut contents: Vec<(String, String)> = vec![];

    match parsed {
        /* If the result is an object, parse it */
        Value::Object(map) => parse_object(empty_root, &map, &mut contents),

        /* If the result is an array, parse it as an array */
        Value::Array(array) => parse_array(empty_root, &array, &mut contents),

        /* If the result is something else, print it */
        _ => parse_other(empty_root, &parsed, &mut contents),
    }

    Ok(contents)
}

fn parse_object(
    input_key: String,
    map: &map::Map<String, Value>,
    append_to: &mut Vec<(String, String)>,
) {
    let iter = map.iter();

    /* Iterate through the entries */
    for (key, value) in iter {
        let final_key = format!("{}/{}", input_key, key);
        /* Check the value's type */
        match *value {
            /* If it's an object, recursively call this method on the value */
            Value::Object(ref new_map) => parse_object(final_key, new_map, append_to),

            /* If it's an array, call the parse_array method on the value */
            Value::Array(ref array) => parse_array(final_key, array, append_to),
            _ => parse_other(final_key, value, append_to),
        }
    }
}

fn parse_array(key: String, array: &Vec<Value>, append_to: &mut Vec<(String, String)>) {
    let iter = array.iter();

    /* Iterate through the items */
    for (index, item) in iter.enumerate() {
        let final_key = format!("{}/{}", key, index);

        /* Check the item's type */
        match *item {
            Value::Object(ref map) => parse_object(final_key, map, append_to),
            Value::Array(ref vec) => parse_array(final_key, vec, append_to),
            _ => parse_other(final_key, item, append_to),
        }
    }
}

fn parse_other(key: String, value: &Value, append_to: &mut Vec<(String, String)>) {
    match *value {
        Value::Null => append_to.push((key, "null".to_string())),
        Value::Bool(ref b) => append_to.push((key, format!("{}", b))),
        Value::Number(ref n) => append_to.push((key, format!("{}", n))),
        Value::String(ref s) => append_to.push((key, format!(r#""{}""#, s))),
        _ => (),
    }
}
