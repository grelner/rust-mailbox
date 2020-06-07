//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

use std::io;
use std::ops::Deref;
use crate::stream::entry::header;
use super::Header;

pub struct Subject(header::Item);

impl Header for Subject {
	#[inline(always)]
	fn name() -> &'static str {
		"Subject"
	}

	#[inline]
	fn parse(values: &[header::Item]) -> io::Result<Self> {
		Ok(Subject(values[0].clone()))
	}
}

impl Deref for Subject {
	type Target = str;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
