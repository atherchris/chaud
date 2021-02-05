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


use std::fs::File;
use std::io::Write;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::prelude::*;
use std::io::SeekFrom;
use std::sync::mpsc;

use crate::codec::Frame;
use crate::codec::unpack_pcm;
use crate::codec::pack_pcm;

const RIFF_CHUNK_ID : u32 = 0x52494646;
const RIFF_FORMAT : u32 = 0x57415645;
const FMT_CHUNK_ID : u32 = 0x666d7420;
const DATA_CHUNK_ID : u32 = 0x64617461;

pub fn read_wav(path: &str, tx: mpsc::Sender<Frame>) {
	let mut file = File::open(path).expect("Failed to open input");

	// RIFF Chunk
	let riff_chunk_id = file.read_u32::<BigEndian>().unwrap();
	assert_eq!(riff_chunk_id, RIFF_CHUNK_ID, "Bad RIFF ID");
	let _riff_chunk_size = file.read_u32::<LittleEndian>().unwrap();
	let riff_chunk_format = file.read_u32::<BigEndian>().unwrap();
	assert_eq!(riff_chunk_format, RIFF_FORMAT, "Bad RIFF format");

	// fmt Chunk
	let fmt_chunk_id = file.read_u32::<BigEndian>().unwrap();
	assert_eq!(fmt_chunk_id, FMT_CHUNK_ID, "Bad fmt chunk ID");
	let _fmt_chunk_size = file.read_u32::<LittleEndian>().unwrap();
	let fmt_audiofmt = file.read_u16::<LittleEndian>().unwrap();
	assert_eq!(fmt_audiofmt, 1, "Unknown audio format");
	let fmt_channels : usize = file.read_u16::<LittleEndian>().unwrap() as usize;
	let fmt_samplerate : usize = file.read_u32::<LittleEndian>().unwrap() as usize;
	let _fmt_byterate = file.read_u32::<LittleEndian>().unwrap();
	let _fmt_blockalign = file.read_u16::<LittleEndian>().unwrap();
	let fmt_bitspersample : usize = file.read_u16::<LittleEndian>().unwrap() as usize;

	// data Chunk
	let data_chunk_id = file.read_u32::<BigEndian>().unwrap();
	assert_eq!(data_chunk_id, DATA_CHUNK_ID, "Missing data chunk");
    let _data_chunk_size = file.read_u32::<LittleEndian>().unwrap();
    
    // data
    let mut data = Vec::new();
    let _ = file.read_to_end(&mut data);
    let frame = Frame {
        channels: fmt_channels,
        sample_rate: fmt_samplerate,
        bits_per_sample: fmt_bitspersample,
        samples: unpack_pcm(data, fmt_bitspersample),
        eof: true,
        error: false,
    };
    tx.send(frame).unwrap();

    /*
	let mut samples_remaining : usize = (fs::metadata(path).unwrap().len() as usize - 44) / (fmt_bitspersample / 8) / fmt_channels;

	while samples_remaining > 0 {
		let rem = std::cmp::min(BUFFER_SIZE, samples_remaining);
		let mut samples : Vec<i32> = Vec::with_capacity(rem * fmt_channels);
        
		for _n in 0..rem {
            for _ch in 0..fmt_channels {
                match fmt_bitspersample {
                    8 => samples.push(file.read_i8().unwrap().into()),
                    16 => samples.push(file.read_i16::<LittleEndian>().unwrap().into()),
                    24 => samples.push(file.read_i24::<LittleEndian>().unwrap().into()),
                    32 => samples.push(file.read_i32::<LittleEndian>().unwrap().into()),
                    _ => panic!("Incompatible bits per sample"),
                }
            }
        }

        let mut data = [0; BUFFER_SIZE];
        file.by_ref().take(rem).read(&mut data);
		
		samples_remaining -= rem;
		let frame = Frame {
			channels: fmt_channels,
			sample_rate: fmt_samplerate,
			bits_per_sample: fmt_bitspersample,
			samples: unpack_pcm(data.to_vec(), fmt_bitspersample, fmt_channels),
			eof: samples_remaining == 0,
			error: false,
        };

		tx.send(frame).unwrap();
		samples.clear();
    }
    */
}

pub fn write_wav(path: &str, rx: mpsc::Receiver<Frame>) {
	let initial_frame = rx.recv().unwrap();

	let mut file = File::create(path).expect("Failed to open output");

	file.write_u32::<BigEndian>(RIFF_CHUNK_ID).unwrap();
	file.write_u32::<LittleEndian>(0x00000000).unwrap();
	file.write_u32::<BigEndian>(RIFF_FORMAT).unwrap();

	file.write_u32::<BigEndian>(FMT_CHUNK_ID).unwrap();
	file.write_u32::<LittleEndian>(0x00000010).unwrap();
	file.write_u16::<LittleEndian>(1).unwrap();
	file.write_u16::<LittleEndian>(initial_frame.channels as u16).unwrap();
	file.write_u32::<LittleEndian>(initial_frame.sample_rate as u32).unwrap();
	file.write_u32::<LittleEndian>(initial_frame.sample_rate as u32 * initial_frame.channels as u32 * initial_frame.bits_per_sample as u32 / 8).unwrap();
	file.write_u16::<LittleEndian>(initial_frame.channels as u16 * initial_frame.bits_per_sample as u16 / 8).unwrap();
	file.write_u16::<LittleEndian>(initial_frame.bits_per_sample as u16).unwrap();

	file.write_u32::<BigEndian>(DATA_CHUNK_ID).unwrap();
	file.write_u32::<LittleEndian>(0x00000000).unwrap();

    let mut data_len : usize = initial_frame.samples.len() * (initial_frame.bits_per_sample / 8);
    let _ = file.write_all(&pack_pcm(initial_frame.samples, initial_frame.bits_per_sample));

	let mut eof = initial_frame.eof;
	while ! eof {
        let frame = rx.recv().unwrap();
        data_len += frame.samples.len() * (frame.bits_per_sample / 8);
        let _ = file.write_all(&pack_pcm(frame.samples, frame.bits_per_sample));
		eof = frame.eof;
	}

	file.seek(SeekFrom::Start(4)).unwrap();
	file.write_u32::<LittleEndian>(36 + data_len as u32).unwrap();

	file.seek(SeekFrom::Start(40)).unwrap();
	file.write_u32::<LittleEndian>(data_len as u32).unwrap();
}

