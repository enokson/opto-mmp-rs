// pub mod scratch_pad;
pub mod utils;

use std::error::Error;
use std::fmt::Display;
use std::io::prelude::*;
use std::net::TcpStream;

use utils::ReadOneResponse;

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

pub fn send_puc(stream: &mut TcpStream, source_id: u16, t_label: u8) -> Result<ReadOneResponse, AppError> {
    let msg = utils::pack_puc_req(source_id, t_label);
    match stream.write(&msg) {
        Ok(_) => Ok(()),
        Err(e) => Err(AppError::IoError(e)),
    }?;
    let mut buffer = [0; 16];
    match stream.read(&mut buffer) {
        Ok(_) => Ok(()),
        Err(e) => Err(AppError::IoError(e)),
    }?;
    let response = utils::unpack_read_quad_res(buffer);
    Ok(response)
}

pub fn set_one(stream: &mut TcpStream, options: utils::WriteOneOptions) -> Result<utils::SetResponse, AppError> {
    let msg = utils::pack_write_quad_req(options);
    match stream.write(&msg) {
        Ok(_) => Ok(()),
        Err(e) => Err(AppError::IoError(e)),
    }?;
    let mut buffer = [0; 12];
    match stream.read(&mut buffer) {
        Ok(_) => Ok(()),
        Err(e) => Err(AppError::IoError(e)),
    }?;
    let response = utils::unpack_write_res(buffer);
    Ok(response)
}

pub fn set(stream: &mut TcpStream, options: utils::WriteOptions) -> Result<utils::SetResponse, AppError> {
    let msg = utils::pack_write_req(options);
    match stream.write(&msg) {
        Ok(_) => Ok(()),
        Err(e) => Err(AppError::IoError(e)),
    }?;
    let mut buffer = [0;12];
    match stream.read(&mut buffer) {
        Ok(_) => Ok(()),
        Err(e) => Err(AppError::IoError(e)),
    }?;
    Ok(utils::unpack_write_res(buffer))
}

pub fn get_one(stream: &mut TcpStream, options: utils::ReadOneOptions) -> Result<utils::ReadOneResponse, AppError> {
    let msg = utils::pack_read_quad_req(options);
    match stream.write(&msg) {
        Ok(_) => Ok(()),
        Err(e) => Err(AppError::IoError(e)),
    }?;
    let mut buffer = [0; 16];
    match stream.read(&mut buffer) {
        Ok(_) => Ok(()),
        Err(e) => Err(AppError::IoError(e)),
    }?;
    let response = utils::unpack_read_quad_res(buffer);
    Ok(response)
}

pub fn get(stream: &mut TcpStream, options: utils::ReadOptions) -> Result<utils::GetResponse, AppError> {
    let mut buffer = Vec::with_capacity(16 + options.data_length as usize);
    let msg = utils::pack_read_req(options);
    match stream.write(&msg) {
        Ok(_) => Ok(()),
        Err(e) => Err(AppError::IoError(e)),
    }?;
    match stream.read(&mut buffer) {
        Ok(_) => Ok(()),
        Err(e) => Err(AppError::IoError(e)),
    }?;
    let response = utils::unpack_read_res(buffer);
    Ok(response)
}

#[macro_export]
macro_rules! get {
    ($stream:expr, $meta_data:expr, $offset:expr, $data_length:expr) => {{
        let mut buffer: [u8; 16 + $data_length as usize] = [0; 16 + $data_length as usize];
        let options = utils::ReadOptions {
            source_id: $meta_data.source_id,
            t_label: $meta_data.t_label,
            offset: $offset,
            data_length: $data_length
        };
        let msg = utils::pack_read_req(options);
        match $stream.write(&msg) {
            Ok(_) => Ok(()),
            Err(e) => Err(AppError::IoError(e)),
        }?;
        match $stream.read(&mut buffer) {
            Ok(_) => Ok(()),
            Err(e) => Err(AppError::IoError(e)),
        }?;
        let (meta_data, data) = $crate::unpack_read_res!(buffer, $data_length as usize);
        Ok((meta_data, data))
    }};
}

#[cfg(test)]
mod test {

    use std::net::TcpStream;
    use std::io::prelude::*;
    use crate::send_puc;

    use super::*;

    fn get_host_env() -> String {
        std::env::var("MMP_HOST").unwrap()
    }

    #[test]
    fn u64_6bytes() {
        let value: u64 = 0xFFFF_F0D8_1000;
        let bytes: [u8;6] = value.to_be_bytes()[2..].try_into().unwrap();
        assert_eq!(bytes, [0xFF, 0xFF, 0xF0, 0xD8, 0x10, 0x00]);
    }

    #[test]
    fn test_scratch_pad_set_and_get () {
        let mut stream = TcpStream::connect(get_host_env()).unwrap();
        send_puc(&mut stream, 0, 1).unwrap();
        let offset = utils::u64_to_offset(0xFFFF_F0D8_1000);
        let test = 2;
        let options = utils::WriteOneOptions {
            source_id: 0x0001,
            t_label: 0x00,
            address: offset,
            data: test
        };
        let result = set_one(&mut stream, options).unwrap();
        assert_eq!(result.r_code, 0);
        let options = utils::ReadOptions {
            source_id: 0x0001,
            t_label: 0x00,
            offset,
            data_length: 4
        };
        let result = get(&mut stream, options).unwrap();
        dbg!(&result);
        assert_eq!(result.r_code, 0);
        assert_eq!(result.data, u32::to_be_bytes(2));
    }

    #[test]
    fn test_scratch_pad_set_and_get_many () -> Result<(), AppError> {
        let mut stream = TcpStream::connect(get_host_env()).unwrap();
        send_puc(&mut stream, 0, 1).unwrap();
        let offset: [u8;6] = utils::u64_to_offset(0xFFFF_F0D8_1000);
        let test = 3;
        let options = utils::MetaDataRequest {
            source_id: 0x0001,
            t_label: 0x00,
            t_code: 0x00,
        };
        let (response, data) = get!(&mut stream, options, offset, 4)?;
        assert_eq!(response.r_code, 0);
        assert_eq!(data, u32::to_be_bytes(test));
        Ok(())
    }

}
