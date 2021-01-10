use serde::{Deserialize, Serialize};
use serde_json::Result;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Read, Write};

use crate::checksum::CheckSum;
use encodelib::chartable::CharTable;

use encoding::all::WINDOWS_1252;
use encoding::{DecoderTrap, EncoderTrap, Encoding};

#[derive(Serialize, Deserialize)]
pub struct MltBlock {
    pub strings: Vec<String>,
}
impl MltBlock {}

#[derive(Serialize, Deserialize)]
pub struct MltFile {
    pub crc_header: u32,
    pub header: [u8; 8],
    pub crc_body: u32,
    pub body: Vec<MltBlock>,
}
impl MltFile {
    pub fn from(buffer: &[u8]) -> Result<MltFile> {
        let mut rdr = Cursor::new(buffer);
        let crc_1 = rdr.read_u32::<LittleEndian>().unwrap();
        let mut header: [u8; 8] = [0; 8];
        rdr.read_exact(&mut header).unwrap();
        let crc_2 = rdr.read_u32::<LittleEndian>().unwrap();

        let len = buffer.len() as u64;
        let mut data: Vec<MltBlock> = vec![];

        while rdr.position() < len {
            let count = rdr.read_u8().unwrap();
            let mut strings: Vec<String> = vec![];

            for _i in 0..count {
                strings.push(read_null_term_str(&mut rdr));
            }

            let m_block = MltBlock { strings: strings };
            data.push(m_block);
        }

        Ok(MltFile {
            crc_header: crc_1,
            header: header,
            crc_body: crc_2,
            body: data,
        })
    }

    pub fn from_json(json: &str) -> Result<MltFile> {
        serde_json::from_str(json)
    }

    pub fn encode(&self, chartable: &CharTable) -> Vec<u8> {
        let mut body = Cursor::new(vec![]);
        for block in &self.body {
            body.write_u8(block.strings.len() as u8).unwrap();
            for entry in &block.strings {
                let replaced = chartable.replace_letters(&entry);
                body.write(
                    &WINDOWS_1252
                        .encode(&replaced, EncoderTrap::Replace)
                        .unwrap(),
                )
                .unwrap();
                body.write_u8(0).unwrap();
            }
        }

        let mut out: Vec<u8> = vec![];
        out.write_u32::<LittleEndian>(CheckSum::compute_crc(&self.header))
            .unwrap();
        out.extend_from_slice(&self.header);
        out.write_u32::<LittleEndian>(CheckSum::compute_crc(body.get_ref()))
            .unwrap();
        out.extend_from_slice(body.get_ref());

        out
    }
}

fn read_null_term_str(rdr: &mut Cursor<&[u8]>) -> String {
    let mut buf: Vec<u8> = vec![];
    loop {
        let byte = rdr.read_u8().unwrap();
        if byte == 0 {
            break;
        }
        buf.push(byte);
    }

    if buf.len() == 0 {
        return String::with_capacity(0);
    }
    WINDOWS_1252.decode(&buf, DecoderTrap::Strict).unwrap()
}
