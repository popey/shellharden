/*
 * Copyright 2016 Andreas Nordal
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use ::situation::Situation;
use ::situation::Transition;
use ::situation::WhatNow;
use ::situation::ParseResult;
use ::situation::flush;

use ::sitextent::SitExtent;

pub struct SitStrSqEsc {}

impl Situation for SitStrSqEsc {
	fn whatnow(&mut self, horizon: &[u8], _is_horizon_lengthenable: bool) -> ParseResult {
		for i in 0 .. horizon.len() {
			if horizon[i] == b'\\' {
				let esc = Box::new(SitExtent{len: 1, color: 0x01ff0080, end_insert: None});
				return Ok(WhatNow{tri: Transition::Push(esc), pre: i, len: 1, alt: None});
			}
			if horizon[i] == b'\'' {
				return Ok(WhatNow{tri: Transition::Pop, pre: i, len: 1, alt: None});
			}
		}
		Ok(flush(horizon.len()))
	}
	fn get_color(&self) -> u32{
		0x00ff8000
	}
}
