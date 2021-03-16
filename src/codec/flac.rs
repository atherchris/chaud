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

#![allow(non_camel_case_types)]

use std::sync::mpsc;
use std::ffi::CString;
use std::convert::TryInto;

use cty;

use crate::codec::Frame;

type FLAC__int8 = i8;
type FLAC__uint8 = u8;

type FLAC__int16 = i16;
type FLAC__int32 = i32;
type FLAC__int64 = i64;

type FLAC__uint16 = u16;
type FLAC__uint32 = u32;
type FLAC__uint64 = u64;

type FLAC__bool = cty::c_int;
type FLAC__byte = FLAC__uint8;


#[repr(C)]
enum FLAC__StreamDecoderState {
	FLAC__STREAM_DECODER_SEARCH_FOR_METADATA,
	FLAC__STREAM_DECODER_READ_METADATA,
	FLAC__STREAM_DECODER_SEARCH_FOR_FRAME_SYNC,
	FLAC__STREAM_DECODER_READ_FRAME,
	FLAC__STREAM_DECODER_END_OF_STREAM,
	FLAC__STREAM_DECODER_OGG_ERROR,
	FLAC__STREAM_DECODER_SEEK_ERROR,
	FLAC__STREAM_DECODER_ABORTED,
	FLAC__STREAM_DECODER_MEMORY_ALLOCATION_ERROR,
	FLAC__STREAM_DECODER_UNINITIALIZED,
}

#[repr(C)]
#[derive(Debug)]
#[derive(PartialEq)]
enum FLAC__StreamDecoderInitStatus {
	FLAC__STREAM_DECODER_INIT_STATUS_OK = 0,
	FLAC__STREAM_DECODER_INIT_STATUS_UNSUPPORTED_CONTAINER,
	FLAC__STREAM_DECODER_INIT_STATUS_INVALID_CALLBACKS,
	FLAC__STREAM_DECODER_INIT_STATUS_MEMORY_ALLOCATION_ERROR,
	FLAC__STREAM_DECODER_INIT_STATUS_ERROR_OPENING_FILE,
	FLAC__STREAM_DECODER_INIT_STATUS_ALREADY_INITIALIZED,
}

#[repr(C)]
#[derive(Debug)]
#[derive(PartialEq)]
enum FLAC__StreamEncoderInitStatus {
    FLAC__STREAM_ENCODER_INIT_STATUS_OK = 0,
    FLAC__STREAM_ENCODER_INIT_STATUS_ENCODER_ERROR,
    FLAC__STREAM_ENCODER_INIT_STATUS_UNSUPPORTED_CONTAINER,
    FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_CALLBACKS,
    FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_NUMBER_OF_CHANNELS,
    FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_BITS_PER_SAMPLE,
    FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_SAMPLE_RATE,
    FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_BLOCK_SIZE,
    FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_MAX_LPC_ORDER,
    FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_QLP_COEFF_PRECISION,
    FLAC__STREAM_ENCODER_INIT_STATUS_BLOCK_SIZE_TOO_SMALL_FOR_LPC_ORDER,
    FLAC__STREAM_ENCODER_INIT_STATUS_NOT_STREAMABLE,
    FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_METADATA,
    FLAC__STREAM_ENCODER_INIT_STATUS_ALREADY_INITIALIZED
}

#[repr(C)]
enum FLAC__StreamDecoderReadStatus {
	FLAC__STREAM_DECODER_READ_STATUS_CONTINUE,
	FLAC__STREAM_DECODER_READ_STATUS_END_OF_STREAM,
	FLAC__STREAM_DECODER_READ_STATUS_ABORT,
}


#[repr(C)]
enum FLAC__StreamDecoderWriteStatus {
	FLAC__STREAM_DECODER_WRITE_STATUS_CONTINUE,
	FLAC__STREAM_DECODER_WRITE_STATUS_ABORT,
}

#[repr(C)]
enum FLAC__StreamDecoderErrorStatus {
	FLAC__STREAM_DECODER_ERROR_STATUS_LOST_SYNC,
	FLAC__STREAM_DECODER_ERROR_STATUS_BAD_HEADER,
	FLAC__STREAM_DECODER_ERROR_STATUS_FRAME_CRC_MISMATCH,
	FLAC__STREAM_DECODER_ERROR_STATUS_UNPARSEABLE_STREAM,
}

type FLAC__StreamDecoder = cty::c_void;
type FLAC__StreamMetadata = cty::c_void;
type FLAC__Frame = cty::c_void;

type FLAC__StreamEncoder = cty::c_void;

type FLAC__StreamDecoderWriteCallback =  Option<extern "C" fn(*mut FLAC__StreamDecoder, *mut FLAC__Frame, *mut *mut FLAC__int32, tx: *mut cty::c_void) -> FLAC__StreamDecoderWriteStatus>;
type FLAC__StreamDecoderMetadataCallback = Option<extern "C" fn(*mut FLAC__StreamDecoder, *mut FLAC__StreamMetadata, tx: *mut cty::c_void)>;
type FLAC__StreamDecoderErrorCallback = Option<extern "C" fn(*mut FLAC__StreamDecoder, FLAC__StreamDecoderErrorStatus, tx: *mut cty::c_void)>;

type FLAC__StreamEncoderProgressCallback = Option<
	extern "C" fn(encoder: *mut FLAC__StreamEncoder,
    	bytes_writter: FLAC__uint64,
    	samples_written: FLAC__uint64,
    	frames_written: cty::c_uint,
    	total_frames_estimate: cty::c_uint,
    	client_data: *mut cty::c_void )>;

extern "C" {

fn FLAC__stream_decoder_new() -> *mut FLAC__StreamDecoder;
fn FLAC__stream_decoder_delete(decoder: *mut FLAC__StreamDecoder);

fn FLAC__stream_decoder_init_file(decoder: *mut FLAC__StreamDecoder,
	filename: *const cty::c_char,
	write_callback: FLAC__StreamDecoderWriteCallback,
	metadata_callback: FLAC__StreamDecoderMetadataCallback,
	error_callback: FLAC__StreamDecoderErrorCallback,
	client_data: *mut cty::c_void) -> FLAC__StreamDecoderInitStatus;

fn FLAC__stream_decoder_get_state(decoder: *const FLAC__StreamDecoder) -> FLAC__StreamDecoderState;

fn FLAC__stream_decoder_set_md5_checking(decoder: *mut FLAC__StreamDecoder, value: FLAC__bool) -> FLAC__bool;
fn FLAC__stream_decoder_get_channels(decoder: *mut FLAC__StreamDecoder) -> cty::c_uint;
fn FLAC__stream_decoder_get_bits_per_sample(decoder: *mut FLAC__StreamDecoder) -> cty::c_uint;
fn FLAC__stream_decoder_get_sample_rate(decoder: *mut FLAC__StreamDecoder) -> cty::c_uint;
fn FLAC__stream_decoder_get_blocksize(decoder: *mut FLAC__StreamDecoder) -> cty::c_uint;
fn FLAC__stream_decoder_process_until_end_of_stream(decoder: *mut FLAC__StreamDecoder) -> FLAC__bool;

fn FLAC__stream_decoder_finish(decoder: *mut FLAC__StreamDecoder) -> FLAC__bool;

fn FLAC__stream_encoder_new() -> *mut FLAC__StreamEncoder;
fn FLAC__stream_encoder_init_file(encoder: *mut FLAC__StreamEncoder,
    filename: *const cty::c_char,
    progress_callback: FLAC__StreamEncoderProgressCallback,
    client_data: *mut cty::c_void) -> FLAC__StreamEncoderInitStatus;
fn FLAC__stream_encoder_set_channels(encoder: *mut FLAC__StreamEncoder, value: cty::c_uint) -> FLAC__bool;
fn FLAC__stream_encoder_set_bits_per_sample(encoder: *mut FLAC__StreamEncoder, value: cty::c_uint) -> FLAC__bool;
fn FLAC__stream_encoder_set_sample_rate(encoder: *mut FLAC__StreamEncoder, value: cty::c_uint) -> FLAC__bool;
fn FLAC__stream_encoder_process_interleaved(encoder: *mut FLAC__StreamEncoder, buffer: *const FLAC__int32, samples: cty::c_uint) -> FLAC__bool;
fn FLAC__stream_encoder_finish(encoder: *mut FLAC__StreamEncoder) -> FLAC__bool;
fn FLAC__stream_encoder_delete(encoder: *mut FLAC__StreamEncoder);

} 

#[no_mangle]
extern "C" fn write_callback(decoder: *mut FLAC__StreamDecoder, _frame: *mut FLAC__Frame, buffer: *mut *mut FLAC__int32, tx: *mut cty::c_void) -> FLAC__StreamDecoderWriteStatus {
	let channels = unsafe { FLAC__stream_decoder_get_channels(decoder) } as usize;
	let sample_rate = unsafe { FLAC__stream_decoder_get_sample_rate(decoder) } as usize;
	let bits_per_sample = unsafe { FLAC__stream_decoder_get_bits_per_sample(decoder) } as usize;
	let block_size = unsafe { FLAC__stream_decoder_get_blocksize(decoder) } as usize;

	let ch_index = unsafe { std::slice::from_raw_parts(buffer, channels) };
	let mut block_vec : Vec<Vec<FLAC__int32>> = Vec::with_capacity(channels);
	for i in 0..channels {
		block_vec.push( unsafe { std::slice::from_raw_parts(ch_index[i], block_size).to_vec() } );
	}

	let mut packed : Vec<i32> = Vec::with_capacity(block_size * channels);
	for i in 0..block_size {
		for ch in 0..channels {
			packed.push(block_vec[ch][i]);
		}
	}

	let frame = Frame {
		channels: channels,
		sample_rate: sample_rate,
		bits_per_sample: bits_per_sample,
		samples: packed,
		eof: false,
	};

	let p_tx = tx as *mut mpsc::Sender<Frame>;
	unsafe { p_tx.as_ref().unwrap().send(frame).unwrap(); }

	FLAC__StreamDecoderWriteStatus::FLAC__STREAM_DECODER_WRITE_STATUS_CONTINUE
}

#[no_mangle]
extern "C" fn metadata_callback(_decoder: *mut FLAC__StreamDecoder, _metadata: *mut FLAC__StreamMetadata, _tx: *mut cty::c_void) {
}

#[no_mangle]
extern "C" fn error_callback(decoder: *mut FLAC__StreamDecoder, status: FLAC__StreamDecoderErrorStatus, tx: *mut cty::c_void) {
	panic!("Error occured during FLAC decoding");
}

pub fn read_flac(path: &str, tx: mpsc::Sender<Frame>) {
    let decoder = unsafe { FLAC__stream_decoder_new() };
	if decoder.is_null() {
		panic!("Failed to create FLAC decoder");
	}

    let mut my_tx = tx;
    let mut p_tx = &mut my_tx as *mut mpsc::Sender<Frame>;

    let cpath = CString::new(path).unwrap();

    unsafe {
        let md5_ret = FLAC__stream_decoder_set_md5_checking(decoder, 1);
		if md5_ret != 1 {
			panic!("Failed to set FLAC MD5 checksuming");
		}

        let init_ret = FLAC__stream_decoder_init_file(decoder, cpath.as_ptr(), Some(write_callback), Some(metadata_callback), Some(error_callback), p_tx as *mut cty::c_void);
		if init_ret != FLAC__StreamDecoderInitStatus::FLAC__STREAM_DECODER_INIT_STATUS_OK {
			panic!("Failed to initialize FLAC decoder");
		}

        let decode_ret = FLAC__stream_decoder_process_until_end_of_stream(decoder);
		if decode_ret != 1 {
			panic!("Error occurred during decoding FLAC");
		}
	}

	let frame = Frame {
		channels: unsafe { FLAC__stream_decoder_get_channels(decoder) } as usize,
		sample_rate: unsafe { FLAC__stream_decoder_get_sample_rate(decoder) } as usize,
		bits_per_sample: unsafe { FLAC__stream_decoder_get_bits_per_sample(decoder) } as usize,
		samples: Vec::new(),
		eof: true,
	};
	my_tx.send(frame).unwrap();

	unsafe {
		FLAC__stream_decoder_finish(decoder);
		FLAC__stream_decoder_delete(decoder);
    }
}



pub fn write_flac(path: &str, rx: mpsc::Receiver<Frame>) {
    let encoder = unsafe { FLAC__stream_encoder_new() };
	if encoder.is_null() {
		panic!("Failed to create FLAC encoder");
	}

    let cpath = CString::new(path).unwrap();
    let mut frame = rx.recv().unwrap();

    unsafe {
        let channel_ret = FLAC__stream_encoder_set_channels(encoder, frame.channels.try_into().unwrap());
		if channel_ret != 1 {
			panic!("Failed to set FLAC channel count");
		}

        let bitspersample_ret = FLAC__stream_encoder_set_bits_per_sample(encoder, frame.bits_per_sample.try_into().unwrap());
		if bitspersample_ret != 1 {
			panic!("Failed to set FLAC bits per sample");
		}

        let samplerate_ret = FLAC__stream_encoder_set_sample_rate(encoder, frame.sample_rate.try_into().unwrap());
		if samplerate_ret != 1 {
			panic!("Failed to set FLAC sample rate");
		}
        
        let init_ret = FLAC__stream_encoder_init_file(encoder, cpath.as_ptr(), None, std::ptr::null_mut());
		if init_ret != FLAC__StreamEncoderInitStatus::FLAC__STREAM_ENCODER_INIT_STATUS_OK {
			panic!("Failed to initialize FLAC encoder");
		}
    }

	while ! frame.eof {
		let process_ret = unsafe { FLAC__stream_encoder_process_interleaved(encoder, frame.samples.as_ptr(), (frame.samples.len() / frame.channels) as u32) };
		if process_ret != 1 {
			panic!("Error occurred while encoding FLAC");
		}
		frame = rx.recv().unwrap();
	}

	if frame.samples.len() > 0 {
		let process_ret = unsafe { FLAC__stream_encoder_process_interleaved(encoder, frame.samples.as_ptr(), (frame.samples.len() / frame.channels) as u32) };
		if process_ret != 1 {
			panic!("Error occurred while encoding FLAC");
		}
	}

    unsafe {
        let finish_ret = FLAC__stream_encoder_finish(encoder);
		if finish_ret != 1 {
			panic!("Failed to finish encoding FLAC");
		}
        FLAC__stream_encoder_delete(encoder);
    }
}
