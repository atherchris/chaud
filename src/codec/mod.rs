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

pub mod wav;
pub mod flac;

pub struct Frame {
	pub channels : usize,
	pub sample_rate : usize,
	pub bits_per_sample : usize,

	pub samples : Vec<i32>,

	pub eof : bool,
	pub error : bool,
}

fn unpack_pcm(data: Vec<u8>, bits_per_sample: usize) -> Vec<i32> {
    let mut pcm : Vec<i32> = Vec::with_capacity(data.len() / (bits_per_sample / 8));

    if bits_per_sample == 8 {
        for x in data {
            pcm.push(x as i32);
        }
    } else if bits_per_sample == 16 {
        for n in 0..(data.len()/2) {
            pcm.push(
                (data[n*2] as i32) |
                (data[n*2+1] as i32) << 8
            );
        }
    } else if bits_per_sample == 24 {
        for n in 0..(data.len()/3) {
            pcm.push(
                (data[n*3] as i32) |
                (data[n*3+1] as i32) << 8 |
                (data[n*3+2] as i32) << 16
            );
        }
    } else if bits_per_sample == 32 {
        for n in 0..(data.len()/4) {
            pcm.push(
                (data[n*4] as i32) |
                (data[n*4+1] as i32) << 8 |
                (data[n*4+2] as i32) << 16 |
                (data[n*4+3] as i32) << 24
            );
        }
    } else {
        panic!("Unsupported bits per sample");
    }

    pcm
}

fn pack_pcm(pcm: Vec<i32>, bits_per_sample: usize) -> Vec<u8> {
    let mut data : Vec<u8> = Vec::with_capacity(pcm.len() * (bits_per_sample / 8));

    if bits_per_sample == 8 {
        for n in pcm {
            data.push(n as u8);
        }
    } else if bits_per_sample == 16 {
        for n in pcm {
            data.push(((n as u32) & 0x000000FF) as u8);
            data.push(((n as u32) & 0x0000FF00 >> 8) as u8);
        }
    } else if bits_per_sample == 24 {
        for n in pcm {
            data.push(((n as u32) & 0x000000FF) as u8);
            data.push(((n as u32) & 0x0000FF00 >> 8) as u8);
            data.push(((n as u32) & 0x00FF0000 >> 16) as u8);
        }
    } else if bits_per_sample == 32 {
        for n in pcm {
            data.push(((n as u32) & 0x000000FF) as u8);
            data.push(((n as u32) & 0x0000FF00 >> 8) as u8);
            data.push(((n as u32) & 0x00FF0000 >> 16) as u8);
            data.push(((n as u32) & 0xFF000000 >> 24) as u8);
        }
    } else {
        panic!("Unsupported bits per sample");
    }

    data
}