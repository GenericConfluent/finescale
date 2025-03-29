#![allow(dead_code, unused_variables)]

use core::panic;
use std::collections::VecDeque;
use std::sync::Arc;

use iced_aw::Card;
use iced_aw::widget::modal::Modal;

use anyhow::anyhow;

pub mod ui;
pub mod schedule;