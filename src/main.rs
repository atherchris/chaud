//
// Copyright (C) 2021 Christopher Atherton <atherchris@gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//

use std::env;

use std::thread;
use std::sync::mpsc;

mod codec;

fn main() -> Result<(), std::io::Error> {
	let args: Vec <_> = env::args().collect();

    let (tx, rx) = mpsc::channel();

	let dec_path = args[1].clone();
	let dec_thread = thread::spawn(move || {
		codec::wav::read_wav(&dec_path, tx)
		//codec::flac::read_flac(&dec_path, tx)
	});

	let enc_path = args[2].clone();
	let enc_thread = thread::spawn(move || {
		//codec::wav::write_wav(&enc_path, rx)
		codec::flac::write_flac(&enc_path, rx)
	});

	let _dec_result = dec_thread.join();
	let _enc_result = enc_thread.join();

	Ok(())
}
