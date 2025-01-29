
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

pub struct Offset<T>(pub T);

impl From<Offset<u64>> for [u8;6] {
    fn from(value: Offset<u64>) -> [u8;6] {
        let bytes_8: [u8;8] = u64::to_be_bytes(value.0);
        let bytes_6: [u8;6] = [bytes_8[2], bytes_8[3], bytes_8[4], bytes_8[5], bytes_8[6], bytes_8[7]];
        bytes_6
    }
}

impl From<Offset<u32>> for [u8;6] {
    fn from(value: Offset<u32>) -> [u8;6] {
        let bytes_4: [u8;4] = u32::to_be_bytes(value.0);
        let bytes_6: [u8;6] = [  0x00, 0x00, bytes_4[0], bytes_4[1], bytes_4[2], bytes_4[3] ];
        bytes_6
    }
}

pub struct Bytes<T>(pub T);

pub mod mmp_errors {
  pub const NO_ERROR: u16 = 0x0000;
  pub const UNIDENTIFIED_COMMAND: u16 = 0xE001;
}

#[derive(Debug)]
pub struct GetResponse {
    pub t_label: u8,
    pub t_code: u8,
    pub r_code: u8,
    pub data: Vec<u8>
}

#[derive(Debug)]
pub struct MetaDataRequest {
    pub source_id: u16,
    pub t_label: u8,
    pub t_code: u8,
}

#[derive(Debug)]
pub struct MetaDataResponse {
    pub source_id: u16,
    pub t_label: u8,
    pub t_code: u8,
    pub r_code: u8,
}

#[derive(Debug)]
pub struct ReadOneResponse {
    pub t_label: u8,
    pub r_code: u8,
    pub t_code: u8,
    pub data: [u8;4],
}

#[derive(Debug)]
pub struct SetResponse {
    pub t_label: u8,
    pub r_code: u8,
    pub t_code: u8,
}

#[derive(Debug)]
pub struct ReadOptions {
    pub source_id: u16,
    pub t_label: u8,
    pub offset: [u8;6],
    pub data_length: u32,
}

#[derive(Debug)]
pub struct ReadOneOptions {
    pub source_id: u16,
    pub t_label: u8,
    pub offset: [u8;6],
}

#[derive(Debug)]
pub struct WriteOptions<'a> {
    pub source_id: u16,
    pub t_label: u8,
    pub offset: [u8;6],
    pub data_length: u16,
    pub data: &'a [u8],
}

#[derive(Debug)]
pub struct WriteOneOptions {
    pub source_id: u16,
    pub t_label: u8,
    pub address: [u8;6],
    pub data: u32,
}

pub fn pack_read_req(options: ReadOptions) -> [u8;16] {
    let destination_id: [u8;2] = [0x00, 0x00];
    let source_id: [u8;2] = options.source_id.to_be_bytes();
    let extended_code: [u8;2] = [0x00, 0x00];
    let len = options.data_length.to_be_bytes();
    let offset = options.offset;
    let meta_data: [u8;16] = [
        destination_id[0], destination_id[1], options.t_label << 2, T_CODE_READ_BLOCK_REQUEST << 4,
        source_id[0], source_id[1], offset[0], offset[1],
        offset[2], offset[3], offset[4], offset[5],
        len[0], len[1], extended_code[0], extended_code[1]
    ];
    meta_data
}

pub fn unpack_read_res(packet: Vec<u8>) -> GetResponse {
    let t_label = packet[2] >> 2;
    let t_code = packet[3] >> 4;
    let r_code = packet[6] >> 4;
    let data = packet[16..].to_vec();
    GetResponse {
        t_label,
        t_code,
        r_code,
        data
    }
}

#[macro_export]
macro_rules! unpack_read_res {
    ($data:expr, $len:expr) => {{
        let data: [u8;$len] = $data[16..16 + $len].try_into().unwrap();
        (
            $crate::utils::MetaDataResponse {
                source_id: u16::from_be_bytes([$data[4], $data[5]]),
                t_label: $data[2] >> 2,
                t_code: $data[3] >> 4,
                r_code: $data[6] >> 4,
            },
            data
        )
    }};
}

pub fn pack_read_quad_req(options: ReadOneOptions) -> [u8;12] {
    let destination_id: [u8;2] = [0x00, 0x00];
    let source_id: [u8;2] = options.source_id.to_be_bytes();
    let t_label = options.t_label << 2;
    let address = options.offset;
    let msg: [u8;12] = [
        destination_id[0], destination_id[1], t_label, T_CODE_READ_QUAD_REQUEST << 4,
        source_id[0], source_id[1], address[0], address[1],
        address[2], address[3], address[4], address[5]
    ];
    msg
}

pub fn unpack_read_quad_res(response: [u8; 16]) -> ReadOneResponse {
    ReadOneResponse {
        t_label: response[2] >> 2,
        t_code: response[3] >> 4,
        r_code: response[6] >> 4,
        data: [response[12], response[13], response[14], response[15]]
    }
}

pub fn pack_write_req(options: WriteOptions) -> Vec<u8> {
    let destination_id: [u8;2] = [0x00, 0x00];
    let source_id: [u8;2] = options.source_id.to_be_bytes();
    let extended_code: [u8;2] = [0x00, 0x00];
    let t_label = options.t_label << 2;
    let len = options.data_length.to_be_bytes();
    let address = options.offset;
    let meta_data: [u8;16] = [
        destination_id[0], destination_id[1], t_label, T_CODE_WRITE_BLOCK_REQUEST << 4,
        source_id[0], source_id[1], address[0], address[1],
        address[2], address[3], address[4], address[5],
        len[0], len[1], extended_code[0], extended_code[1]
    ];
    let mut msg: Vec<u8> = Vec::with_capacity(16 + options.data_length as usize);
    msg.extend_from_slice(&meta_data);
    msg.extend_from_slice(options.data);
    msg
}

pub fn pack_write_quad_req(options: WriteOneOptions) -> [u8;16] {
    let destination_id: [u8;2] = [0x00, 0x00];
    let source_id: [u8;2] = [0x00, 0x00];
    let data = options.data.to_be_bytes();
    let t_label = options.t_label << 2;
    let offset = options.address;
    let msg: [u8;16] = [
        destination_id[0], destination_id[1], t_label, T_CODE_WRITE_QUAD_REQUEST << 4,
        source_id[0], source_id[1], offset[0], offset[1],
        offset[2], offset[3], offset[4], offset[5],
        data[0], data[1], data[2], data[3]
    ];
    msg
}

pub fn unpack_write_res(response: [u8; 12]) -> SetResponse {
    SetResponse {
        t_label: response[2] >> 2,
        t_code: response[3] >> 4,
        r_code: response[6] >> 4
    }
}

pub fn pack_puc_req(source_id: u16, t_label: u8) -> [u8;16] {
    let options = WriteOneOptions {
        source_id,
        t_label,
        address: [0xFF, 0xFF, 0xF0, 0x38, 0x00, 0x00],
        data: 0x0000_0001
    };
    pack_write_quad_req(options)
}

pub fn u64_to_offset(n: u64) -> [u8;6] {
    Offset(n).into()
}

pub fn u32_to_offset(n: u32) -> [u8;6] {
    Offset(n).into()
}
