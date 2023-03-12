
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_must_use)]
#![allow(unused_assignments)]
#![allow(unused_macros)]

use std::io::prelude::*;
use std::collections::*;

pub mod cli;
pub use cli::*;

pub mod gui;
pub use gui::*;

pub mod util;
pub use util::*;
