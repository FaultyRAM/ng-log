// The MIT License (MIT)
//
// Copyright (c) 2015 FaultyRAM
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

//! ngLog processing utilities.
//!
//! ngLog is a textual file format designed for recording gameplay events. Each
//! line of text in an ngLog-formatted file represents an *event*, consisting
//! of several parameters in the following order, each separated by an ASCII
//! TAB control code:
//!
//! * A *timestamp*: a floating-point number representing the time in seconds
//!   that have elapsed since gameplay began;
//! * An optional *event class*, describing the category to which the event
//!   belongs;
//! * An *event ID*, describing the type of event which occurred;
//! * Zero or more *event parameters*, each representing an arbitrary data
//!   point associated with the event.
//!
//! ngLog was used in conjunction with ngStats and ngWorldStats to provide both
//! local and online statistical analysis and tracking. Supported video games
//! would create two copies of an ngLog file upon gameplay completion: a copy
//! for local processing, and an encoded copy to be sent to a *world server*.
//! This crate provides functionality for processing both forms, from either a
//! `String` or a type that implements `std::io::Read`. For example:
//!
//! ```rust
//! use ng_log::NgLog;
//!
//! use std::fs::OpenOptions;
//! use std::io::Read;
//! use std::string::ToString;
//!
//! let mut file = OpenOptions::new()
//! 	.read(true)
//! 	.open("./tests/ngLog_Example_Log_File.log.txt")
//! 	.unwrap();
//! let log = NgLog::local_from_reader(&mut file).unwrap();
//! println!("{}", log.to_string());
//! ```

use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use std::io::Result as IoResult;
use std::io::Read;
use std::string::ToString;

/// A type representing an ngLog-formatted file.
pub struct NgLog {
	/// A collection of ngLog events.
	pub events: Vec<NgEvent>,
}

/// A type representing an ngLog event.
pub struct NgEvent {
	/// A floating-point value representing the elapsed time since gameplay began.
	pub timestamp:    String,
	/// The category to which this event belongs, if any.
	pub event_class:  Option<String>,
	/// The type of this event.
	pub event_id:     String,
	/// Optional data points associated with this event.
	pub event_params: Vec<String>,
}

impl NgLog {
	/// Constructs a new `NgLog` instance, allocating memory for at least
	/// `capacity` events.
	pub fn new(capacity: usize) -> NgLog {
		NgLog {
			events: Vec::with_capacity(capacity),
		}
	}

	/// Constructs a new `NgLog` instance using data from a type implementing
	/// `std::io::Read`. The data is interpreted as a UTF-8 string.
	///
	/// # Failures
	///
	/// If the input data is either not valid UTF-8 or malformed, this method
	/// returns an `std::io::Error` instance describing the error.
	pub fn local_from_reader<T>(reader: &mut T) -> IoResult<NgLog> where
	T: Read {
		let mut data: Vec<u8> = Vec::with_capacity(0);
		try!(reader.read_to_end(&mut data));
		NgLog::from_string(&try!(String::from_utf8(data).map_err(|e|
			IoError::new(IoErrorKind::InvalidData, format!("{}", e))
		)))
	}

	/// Constructs a new `NgLog` instance using data from a type implementing
	/// `std::io::Error`. The data is fed through a decoding algorithm, then
	/// interpreted as a UTF-8 string.
	///
	/// # Failures
	///
	/// If the input data is either not valid UTF-8 or malformed, this method
	/// returns an `std::io::Error` instance describing the error.
	pub fn world_from_reader<T>(reader: &mut T) -> IoResult<NgLog> where
	T: Read {
		let mut data: Vec<u8> = Vec::with_capacity(0);
		try!(reader.read_to_end(&mut data));
		if data.len() % 2 != 0 {
			return Err(IoError::new(IoErrorKind::InvalidData, "Non-even log length"))
		}
		// TODO: check if this format is used by games other than UT99.
		let mut decoded: Vec<u8> = Vec::with_capacity(data.len() / 2);
		for v in data.chunks(2) {
			decoded.push(v[0] ^ v[1]);
		}
		NgLog::from_string(&try!(String::from_utf8(decoded).map_err(|e|
			IoError::new(IoErrorKind::InvalidData, format!("{}", e))
		)))
	}

	/// Constructs a new `NgLog` instance from the given input string.
	///
	/// # Failures
	///
	/// If the input data is malformed, this method returns an `std::io::Error`
	/// instance describing the error.
	pub fn from_string(s: &String) -> IoResult<NgLog> {
		let mut log = NgLog::new(s.len());
		for line in s.lines() {
			let event = try!(NgEvent::from_string(&String::from(line)));
			log.events.push(event);
		}
		Ok(log)
	}
}

impl ToString for NgLog {
	fn to_string(&self) -> String {
		let mut s = String::with_capacity(0);
		for v in &self.events {
			s = s + &v.to_string() + &"\n";
		}
		s
	}
}

impl NgEvent {
	/// Constructs a new `NgEvent` instance from the given arguments.
	pub fn new(timestamp: String, class: Option<String>, id: String, params: Vec<String>) -> NgEvent {
		NgEvent {
			timestamp:    timestamp,
			event_class:  class,
			event_id:     id,
			event_params: params,
		}
	}

	/// Constructs a new `NgEvent` instance from the given input string.
	///
	/// # Failures
	///
	/// If the given data is malformed, this method returns an `std::io::Error`
	/// instance describing the error.
	pub fn from_string(s: &String) -> IoResult<NgEvent> {
		let mut columns: Vec<String> = s.split('\t').map(|s| String::from(s)).collect();
		if columns.len() < 2 {
			return Err(IoError::new(IoErrorKind::InvalidData, "Bad event string"))
		} else if columns.len() == 2 {
			Ok(NgEvent::new(
				columns.remove(0),
				None,
				columns.remove(0),
				Vec::with_capacity(0)
			))
		} else {
			Ok(NgEvent::new(
				columns.remove(0),
				Some(columns.remove(0)),
				columns.remove(0),
				columns.clone()
			))
		}
	}
}

impl ToString for NgEvent {
	fn to_string(&self) -> String {
		let mut s = String::with_capacity(0);
		s = s + &self.timestamp;
		if let Some(v) = self.event_class.clone() {
			s = s + &"\t" + &v;
		}
		s = s + &"\t" + &self.event_id;
		for v in self.event_params.iter() {
			s = s + &"\t" + &v;
		}
		s
	}
}
