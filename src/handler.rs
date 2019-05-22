pub mod task;
pub mod category;
pub mod tasktype;
pub mod achievement;

use actix_web::{error};
use bytes::BytesMut;

const MAX_SIZE: usize = 262_144;

fn read_bytes(mut body: BytesMut, chunk: bytes::Bytes) -> Result<BytesMut, error::Error> {
    if (body.len() + chunk.len()) > MAX_SIZE {
       Err(error::ErrorBadRequest("overflow"))
    } else {
       body.extend_from_slice(&chunk);
       Ok(body)
    }
}