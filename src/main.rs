/// Transaction Codes (tcode)
pub const T_CODE_WRITE_QUAD_REQUEST: u8     = 0;
pub const T_CODE_WRITE_BLOCK_REQUEST: u8    = 1;
pub const T_CODE_WRITE_RESPONSE: u8         = 2;
pub const T_CODE_READ_QUAD_REQUEST: u8      = 4;
pub const T_CODE_READ_BLOCK_REQUEST: u8     = 5;
pub const T_CODE_READ_QUAD_RESPONSE: u8     = 6;
pub const T_CODE_READ_BLOCK_RESPONSE: u8    = 7;

/// Opto 22 mem-map package sizes
pub const SIZE_WRITE_QUAD_REQUEST: u8       = 16;
pub const SIZE_WRITE_BLOCK_REQUEST: u8      = 16;
pub const SIZE_WRITE_RESPONSE: u8           = 12;
pub const SIZE_READ_QUAD_REQUEST: u8        = 12;
pub const SIZE_READ_BLOCK_REQUEST: u8       = 16;
pub const SIZE_READ_QUAD_RESPONSE: u8       = 16;
pub const SIZE_READ_BLOCK_RESPONSE: u8      = 16;

/// Digital IO area
pub const BASE_D_POINT_WRITE: u32    = 0xF022_0000;
pub const BASE_D_POINT_READ: u32     = 0xF01E_0000;
pub const OFFSET_D_POINT_MOD: u32    = 0x0000_1000;
pub const OFFSET_D_POINT: u32        = 0x0000_0040;

// Analog IO area
pub const BASE_A_POINT_WRITE: u32    = 0xF02A_0000;
pub const BASE_A_POINT_READ: u32     = 0xF026_0000;
pub const OFFSET_A_POINT_MOD: u32    = 0x0000_1000;
pub const OFFSET_A_POINT: u32        = 0x0000_0040;
pub const OFFSET_A_POINT_MIN: u32    = 0x0000_0008;
pub const OFFSET_A_POINT_MAX: u32    = 0x0000_000C;

/// System status area
pub const BASE_IP_ADDRESS_ETH0: u32  = 0xF030_0034;
pub const BASE_MAC_ADDRESS_ETH0: u32 = 0xF030_002E;
pub const BASE_IP_ADDRESS_ETH1: u32  = 0xFFFF_F050;
pub const BASE_MAC_ADDRESS_ETH1: u32 = 0xFFFF_F060;
pub const BASE_FIRMWARE_VERSION: u32 = 0xF030_001C;
pub const BASE_UNIT_DESCRIPTION: u32 = 0xF030_0080;
pub const BASE_LAST_ERROR: u32       = 0xF030_000C;

pub const MODULE_POINT_OFFSET: u32 = 0x0000_1000;
pub const CHANNEL_POINT_OFFSET: u32 = 0x0000_0040;

/// ScratchPad area
/// string
pub const BASE_SCRATCHPAD_STRING: u64      = 0xF0D8_3000;
pub const OFFSET_SCRATCHPAD_STRING: u64    = 0x0000_0082;
pub const MAX_BYTES_STRING: u64            = 0x0000_2080;

/// float
pub const BASE_SCRATCHPAD_FLOAT: u64       = 0xF0D8_2000;
pub const BASE_SCRATCHPAD_FLOAT_1: u64     = 0xF0D8_2000;
pub const BASE_SCRATCHPAD_FLOAT_2: u64     = 0xF0DC_0000;
pub const BASE_SCRATCHPAD_FLOAT_3: u64     = 0xF0DC_2000;
pub const MAX_ELEMENTS_FLOAT_1: u64        = 0x0000_0400;
pub const MAX_ELEMENTS_FLOAT_2: u64        = 0x0000_0800;
pub const MAX_ELEMENTS_FLOAT_3: u64        = 0x0000_1C00;
pub const MAX_ELEMENTS_FLOAT: u64          = MAX_ELEMENTS_FLOAT_1 + MAX_ELEMENTS_FLOAT_2 + MAX_ELEMENTS_FLOAT_3;
pub const MAX_BYTES_FLOAT: u64             = MAX_ELEMENTS_FLOAT * 4;

/// integer
pub const BASE_SCRATCHPAD_INTEGER: u64     = 0xF0D8_1000;
pub const BASE_SCRATCHPAD_INTEGER_1: u64   = 0xF0D8_1000;
pub const BASE_SCRATCHPAD_INTEGER_2: u64   = 0xF0DA_0000;
pub const BASE_SCRATCHPAD_INTEGER_3: u64   = 0xF0DA_2000;

/// integer
pub const MAX_ELEMENTS_INTEGER_1: u64      = 0x0000_0400;
pub const MAX_ELEMENTS_INTEGER_2: u64      = 0x0000_0800;
pub const MAX_ELEMENTS_INTEGER_3: u64      = 0x0000_1C00;
pub const MAX_ELEMENTS_INTEGER: u64        = MAX_ELEMENTS_INTEGER_1 + MAX_ELEMENTS_INTEGER_2 + MAX_ELEMENTS_INTEGER_3;
pub const MAX_BYTES_INTEGER: u64           = MAX_ELEMENTS_INTEGER * 4;


pub mod mmp_errors {
    pub const NO_ERROR: u16 = 0x0000;
    pub const UNIDENTIFIED_COMMAND: u16 = 0xE001;
}



use std::error::Error;
use std::fmt::Display;
use std::io::prelude::*;
use std::net::TcpStream;

#[derive(Debug)]
pub enum AppError {
    IoError(std::io::Error),
    ParseIntError(std::num::ParseIntError),
    InvalidData,
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::IoError(e) => write!(f, "IO Error: {}", e),
            AppError::InvalidData => write!(f, "Invalid data"),
            AppError::ParseIntError(e) => write!(f, "ParseIntError: {}", e),
        }
    }
}

impl Error for AppError {}

pub fn send_puc(stream: &mut TcpStream) -> Result<(), AppError> {
    let msg = build_puc_request();
    match stream.write(&msg) {
        Ok(_) => Ok(()),
        Err(e) => Err(AppError::IoError(e)),
    }?;
    Ok(())
}

pub struct ReadBlockResponse<'a> {
    pub t_label: u8,
    pub t_code: u8,
    pub r_code: u8,
    pub data: &'a [u8]
}

pub struct ReadQuadRes {
    pub t_label: u8,
    pub r_code: u8,
    pub t_code: u8,
    pub data: [u8;4],
}

pub struct WriteResponse {
    pub t_label: u8,
    pub r_code: u8,
    pub t_code: u8,
}

pub fn mk_read_req(t_label: u8, destination_offset: [u8;6], data_length: u16) -> [u8;16] {
    let destination_id: [u8;2] = [0x00, 0x00];
    let source_id: [u8;2] = [0x00, 0x00];
    let extended_code: [u8;2] = [0x00, 0x00];
    let len_byte_array = data_length.to_be_bytes();
    let meta_data: [u8;16] = [
        destination_id[0], destination_id[1], t_label << 2, T_CODE_READ_BLOCK_REQUEST << 4,
        source_id[0], source_id[1], destination_offset[0], destination_offset[1],
        destination_offset[2], destination_offset[3], destination_offset[4], destination_offset[5],
        len_byte_array[0], len_byte_array[1], extended_code[0], extended_code[1]
    ];
    meta_data
}

pub fn mk_read_res(packet: &[u8]) -> ReadBlockResponse {
    let t_label = packet[2] >> 2;
    let t_code = packet[3] >> 4;
    let r_code = packet[6] >> 4;
    let data = &packet[16..];
    ReadBlockResponse {
        t_label,
        t_code,
        r_code,
        data
    }
}

pub fn mk_read_quad_req(t_label: u8, address: [u8;6]) -> [u8;12] {
    let destination_id: [u8;2] = [0x00, 0x00];
    let source_id: [u8;2] = [0x00, 0x00];
    let msg: [u8;12] = [
        destination_id[0], destination_id[1], t_label << 2, T_CODE_READ_QUAD_REQUEST << 4,
        source_id[0], source_id[1], address[0], address[1],
        address[2], address[3], address[4], address[5]
    ];
    msg
}
 
pub fn mk_read_quad_res(response: [u8; 16]) -> ReadQuadRes {
    ReadQuadRes {
        t_label: response[2] >> 2,
        t_code: response[3] >> 4,
        r_code: response[6] >> 4,
        data: [response[12], response[13], response[14], response[15]]
    }
}

pub fn mk_write_req(t_label: u8, address: [u8;6], length: u16, mut data: Vec<u8>) -> Vec<u8> {
    let destination_id: [u8;2] = [0x00, 0x00];
    let source_id: [u8;2] = [0x00, 0x00];
    let extended_code: [u8;2] = [0x00, 0x00];
    let len_byte_array = length.to_be_bytes();
    let meta_data: [u8;16] = [
        destination_id[0], destination_id[1], t_label << 2, T_CODE_WRITE_BLOCK_REQUEST << 4,
        source_id[0], source_id[1], address[0], address[1],
        address[2], address[3], address[4], address[5],
        len_byte_array[0], len_byte_array[1], extended_code[0], extended_code[1]
    ];
    let mut msg: Vec<u8> = Vec::with_capacity(16 + length as usize);
    msg.extend_from_slice(&meta_data);
    msg.append(&mut data);
    msg
}

pub fn mk_write_quad_req(t_label: u8, address: [u8;6], data: u32) -> [u8;16] {
    let destination_id: [u8;2] = [0x00, 0x00];
    let source_id: [u8;2] = [0x00, 0x00];
    let data_array = data.to_be_bytes();
    let msg: [u8;16] = [
        destination_id[0], destination_id[1], t_label << 2, T_CODE_WRITE_QUAD_REQUEST << 4,
        source_id[0], source_id[1], address[0], address[1],
        address[2], address[3], address[4], address[5],
        data_array[0], data_array[1], data_array[2], data_array[3]
    ];
    msg
}

pub fn mk_write_quad_res(response: [u8; 12]) -> WriteResponse {
    WriteResponse {
        t_label: response[2] >> 2,
        t_code: response[3] >> 4,
        r_code: response[6] >> 4
    }
}

pub fn build_puc_request() -> [u8;16] {
    mk_write_quad_req(0x4,  [0xFF, 0xFF, 0xF0, 0x38, 0x00, 0x00], 1)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn unpack_write_response_test() {
        let response: [u8;12] = [
            0b00000000, 0b00000000, 0b00000100, 0b00100000,
            0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000, 0b00000000, 0b00000000
        ];
        let result = mk_write_quad_res(response);
        assert_eq!(result.t_label, 1);
        assert_eq!(result.t_code, 2);
        // assert_eq!(0xFF, 0x1C);
    }

    #[test]
    fn test_shift () {
        let mut response: [u8;4] = [
            0b00000000, 0b00000000, 0b00000100, 0b01100000,
        ];
        response[3] >>= 4;
        assert_eq!(1 << 4, 0b00010000);
    }
}
