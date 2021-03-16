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

pub mod wav;
pub mod flac;
pub mod vorbis;

pub struct Frame {
	pub channels : usize,
	pub sample_rate : usize,
	pub bits_per_sample : usize,

	pub samples : Vec<i32>,

	pub eof : bool,
}

fn unpack_pcm(data: Vec<u8>, bits_per_sample: usize) -> Vec<i32> {
    let mut pcm = Vec::with_capacity(data.len() / (bits_per_sample / 8));

    if bits_per_sample == 8 {
        for n in data {
            pcm.push(n as i32);
        }
    } else if bits_per_sample == 16 {
        for n in 0..(data.len()/2) {
            pcm.push((
                ((data[n*2] as i16) << 0) |
                ((data[n*2+1] as i16) << 8))
                as i32
            );
        }
    } else if bits_per_sample == 24 {
        for n in 0..(data.len()/3) {
            pcm.push((
                ((data[n*3] as i32) << 0) |
                ((data[n*3+1] as i32) << 8) |
                ((data[n*3+2] as i32) << 16))
                as i32
            );
        }
    } else if bits_per_sample == 32 {
        for n in 0..(data.len()/4) {
            pcm.push((
                ((data[n*4] as i32) << 0) |
                ((data[n*4+1] as i32) << 8) |
                ((data[n*4+2] as i32) << 16) |
                ((data[n*4+3] as i32) << 24))
                as i32
            );
        }
    } else {
        panic!("Unsupported bits per sample");
    }

    pcm
}

fn pack_pcm(pcm: Vec<i32>, bits_per_sample: usize) -> Vec<u8> {
    let mut data = Vec::with_capacity(pcm.len() * (bits_per_sample / 8));

    if bits_per_sample == 8 {
        for n in pcm {
            data.push(n as u8);
        }
    } else if bits_per_sample == 16 {
        for n in pcm {
            data.push(((n as u32) & 0x000000FF) as u8);
            data.push((((n as u32) & 0x0000FF00) >> 8) as u8);
        }
    } else if bits_per_sample == 24 {
        for n in pcm {
            data.push(((n as u32) & 0x000000FF) as u8);
            data.push((((n as u32) & 0x0000FF00) >> 8) as u8);
            data.push((((n as u32) & 0x00FF0000) >> 16) as u8);
        }
    } else if bits_per_sample == 32 {
        for n in pcm {
            data.push(((n as u32) & 0x000000FF) as u8);
            data.push((((n as u32) & 0x0000FF00) >> 8) as u8);
            data.push((((n as u32) & 0x00FF0000) >> 16) as u8);
            data.push((((n as u32) & 0xFF000000) >> 24) as u8);
        }
    } else {
        panic!("Unsupported bits per sample");
    }

    data
}
