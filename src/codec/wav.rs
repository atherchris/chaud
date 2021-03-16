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
	let mut file = File::open(path).unwrap();

	// RIFF Chunk
	let riff_chunk_id = file.read_u32::<BigEndian>().unwrap();
	if riff_chunk_id != RIFF_CHUNK_ID {
		panic!("Bad RIFF ID");
	}
	let _riff_chunk_size = file.read_u32::<LittleEndian>().unwrap();
	let riff_chunk_format = file.read_u32::<BigEndian>().unwrap();
	if riff_chunk_format != RIFF_FORMAT {
		panic!("Bad RIFF format");
	}

	// fmt Chunk
	let fmt_chunk_id = file.read_u32::<BigEndian>().unwrap();
	if fmt_chunk_id != FMT_CHUNK_ID {
		panic!("Bad fmt chunk ID");
	}
	let _fmt_chunk_size = file.read_u32::<LittleEndian>().unwrap();
	let fmt_audiofmt = file.read_u16::<LittleEndian>().unwrap();
	if fmt_audiofmt != 1 {
		panic!("Unknown audio format");
	}
	let fmt_channels : usize = file.read_u16::<LittleEndian>().unwrap() as usize;
	let fmt_samplerate : usize = file.read_u32::<LittleEndian>().unwrap() as usize;
	let _fmt_byterate = file.read_u32::<LittleEndian>().unwrap();
	let _fmt_blockalign = file.read_u16::<LittleEndian>().unwrap();
	let fmt_bitspersample : usize = file.read_u16::<LittleEndian>().unwrap() as usize;

	// data Chunk
	let data_chunk_id = file.read_u32::<BigEndian>().unwrap();
	if data_chunk_id != DATA_CHUNK_ID {
		panic!("Missing data chunk");
	}
    let _data_chunk_size = file.read_u32::<LittleEndian>().unwrap();
    
    // data
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    let frame = Frame {
        channels: fmt_channels,
        sample_rate: fmt_samplerate,
        bits_per_sample: fmt_bitspersample,
        samples: unpack_pcm(data, fmt_bitspersample),
        eof: true,
    };
    tx.send(frame).unwrap();
}

pub fn write_wav(path: &str, rx: mpsc::Receiver<Frame>) {
	let mut file = File::create(path).unwrap();

	let mut frame = rx.recv().unwrap();

	file.write_u32::<BigEndian>(RIFF_CHUNK_ID).unwrap();
	file.write_u32::<LittleEndian>(0x00000000).unwrap();
	file.write_u32::<BigEndian>(RIFF_FORMAT).unwrap();

	file.write_u32::<BigEndian>(FMT_CHUNK_ID).unwrap();
	file.write_u32::<LittleEndian>(0x00000010).unwrap();
	file.write_u16::<LittleEndian>(1).unwrap();
	file.write_u16::<LittleEndian>(frame.channels as u16).unwrap();
	file.write_u32::<LittleEndian>(frame.sample_rate as u32).unwrap();
	file.write_u32::<LittleEndian>(frame.sample_rate as u32 * frame.channels as u32 * frame.bits_per_sample as u32 / 8).unwrap();
	file.write_u16::<LittleEndian>(frame.channels as u16 * frame.bits_per_sample as u16 / 8).unwrap();
	file.write_u16::<LittleEndian>(frame.bits_per_sample as u16).unwrap();

	file.write_u32::<BigEndian>(DATA_CHUNK_ID).unwrap();
	file.write_u32::<LittleEndian>(0x00000000).unwrap();

	let mut data_len = 0;

	while ! frame.eof {
        data_len += frame.samples.len() * (frame.bits_per_sample / 8);
        file.write_all(&pack_pcm(frame.samples, frame.bits_per_sample)).unwrap();
        frame = rx.recv().unwrap();
	}

	if frame.samples.len() > 0 {
        data_len += frame.samples.len() * (frame.bits_per_sample / 8);
        file.write_all(&pack_pcm(frame.samples, frame.bits_per_sample)).unwrap();
	}

	file.seek(SeekFrom::Start(4)).unwrap();
	file.write_u32::<LittleEndian>(36 + data_len as u32).unwrap();

	file.seek(SeekFrom::Start(40)).unwrap();
	file.write_u32::<LittleEndian>(data_len as u32).unwrap();
}
