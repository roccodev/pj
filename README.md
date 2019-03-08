<!--
 Copyright (C) 2019 RoccoDev
 
 This file is part of pj.
 
 pj is free software: you can redistribute it and/or modify
 it under the terms of the GNU General Public License as published by
 the Free Software Foundation, either version 3 of the License, or
 (at your option) any later version.
 
 pj is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY; without even the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 GNU General Public License for more details.
 
 You should have received a copy of the GNU General Public License
 along with pj.  If not, see <http://www.gnu.org/licenses/>.
-->
# pj
[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg?longCache=true)](https://crates.io/crates/pj)  
![Showcase](https://roccodev.pw/img/pj_terminal.png)
***

`pj` (short for `P`arse `J`SON) is a JSON parser and visualizer for the command line, written in Rust.

# Usage
`pj` reads input from `stdin`. This means you can pipe stuff into it. For example:  
```sh
curl "https://rocco.dev/example.json" | pj
```
## Controls
Use arrow keys (`↑` `↓`) to navigate the keys.  
You can also use `Home` and `End` to quickly go to the top or bottom of an object.

Use `q` to quit.

# Install
## Cross-platform
```sh
cargo install pj
```
## Arch Linux
`pj` is available in the [Arch User Repository](https://aur.archlinux.org/packages/pj/).
## Other distributions
RPM and DEB packages are available in the [Releases](https://github.com/RoccoDev/pj/releases) tab.

# License
This project is licensed under the [GPLv3](LICENSE) license.
