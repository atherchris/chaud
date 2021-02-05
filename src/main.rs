//
// Copyright (c) 2021 Christopher Atherton <atherchris@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
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
	});

	let enc_path = args[2].clone();
	let enc_thread = thread::spawn(move || {
		codec::wav::write_wav(&enc_path, rx)
	});

	let _dec_result = dec_thread.join();
	let _enc_result = enc_thread.join();

	Ok(())
}
