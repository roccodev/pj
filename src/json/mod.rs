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
use std::fmt::Write;

pub fn parse_new(input: String) -> Result<Vec<(String, String)>> {
    /* Parse the input string as JSON */
    let parsed: Value = serde_json::from_str(&input.as_str())?;
    let empty_root = String::new();

    let contents: Vec<(String, String)> = vec![];

    Ok(contents)
}

pub fn parse(input: String) -> Result<()> {
    /* Parse the input string as JSON */
    let parsed: Value = serde_json::from_str(&input.as_str())?;
    let empty_root = String::new();

    match parsed {
        /* If the result is an object, parse it */
        Value::Object(map) => parse_object(empty_root, &map),

        /* If the result is an array, parse it as an array */
        Value::Array(array) => parse_array(empty_root, &array),

        /* If the result is something else, print it */
        _ => parse_other(empty_root, &parsed),
    }

    Ok(())
}

fn parse_object(input_key: String, map: &map::Map<String, Value>) {
    let iter = map.iter();

    /* Iterate through the entries */
    for (key, value) in iter {
        let final_key = format!("{}/{}", input_key, key);
        /* Check the value's type */
        match *value {
            /* If it's an object, recursively call this method on the value */
            Value::Object(ref new_map) => parse_object(final_key, new_map),

            /* If it's an array, call the parse_array method on the value */
            Value::Array(ref array) => parse_array(final_key, array),
            _ => parse_other(final_key, value),
        }
    }
}

fn parse_array(key: String, array: &Vec<Value>) {
    let iter = array.iter();

    /* Iterate through the items */
    for (index, item) in iter.enumerate() {
        let final_key = format!("{}/{}", key, index);

        /* Check the item's type */
        match *item {
            Value::Object(ref map) => parse_object(final_key, map),
            _ => parse_other(final_key, item),
        }
    }
}

fn parse_other(key: String, value: &Value) {
    match *value {
        Value::Null => print(key, "null".to_string()),
        Value::Bool(ref b) => print(key, format!("{}", b)),
        Value::Number(ref n) => print(key, format!("{}", n)),
        Value::String(ref s) => print(key, format!(r#""{}""#, s)),
        _ => (),
    }
}

fn print(key: String, value: String) {
    writeln!(format!("Key: {}, Value: {}", key, value));
}
