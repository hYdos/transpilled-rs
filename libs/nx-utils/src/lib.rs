use std::io::{Cursor, Read, Seek, SeekFrom};
use std::str::from_utf8;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::NroSegmentType::{DATA, RO, TEXT};

pub enum NroSegmentType {
    TEXT = 0,
    RO = 1,
    DATA = 2,
}

pub struct NroSegment {
    offset: usize,
    size: usize,
}

impl NroSegment {
    pub fn new(reader: &mut Cursor<Vec<u8>>) -> NroSegment {
        NroSegment {
            offset: reader.read_u32::<LittleEndian>().unwrap() as usize,
            size: reader.read_u32::<LittleEndian>().unwrap() as usize,
        }
    }
}

pub struct SwitchExecutable {
    program: Vec<u8>,
    text: Vec<u8>,
    ro: Vec<u8>,
    data: Vec<u8>,
    bss: Vec<u8>,
}

impl SwitchExecutable {
    pub fn read_nro(file_bytes: Vec<u8>) -> SwitchExecutable {
        let mut reader = Cursor::new(file_bytes.clone());
        reader.seek(SeekFrom::Current(4)).unwrap();
        reader.seek(SeekFrom::Current(4)).unwrap(); // mod0: u32
        reader.seek(SeekFrom::Current(8)).unwrap();
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        reader.read_exact(&mut buf).unwrap();
        let magic = from_utf8(&buf).unwrap();
        assert_eq!(magic, "NRO0");
        let _version = reader.read_u32::<LittleEndian>().unwrap();
        let _header_size = reader.read_u32::<LittleEndian>().unwrap();
        reader.seek(SeekFrom::Current(4)).unwrap();
        let sectors = [
            NroSegment::new(&mut reader),
            NroSegment::new(&mut reader),
            NroSegment::new(&mut reader)
        ];
        let bss_size = reader.read_u32::<LittleEndian>().unwrap();
        reader.seek(SeekFrom::Current(4)).unwrap();
        reader.seek(SeekFrom::Current(0x20)).unwrap(); // Build ID
        reader.seek(SeekFrom::Current(0x20)).unwrap();

        // TODO: asset processing?

        let text = file_bytes[sectors[TEXT as usize].offset..][..sectors[TEXT as usize].size].to_vec();
        let ro = file_bytes[sectors[RO as usize].offset..][..sectors[RO as usize].size].to_vec();
        let data = file_bytes[sectors[DATA as usize].offset..][..sectors[DATA as usize].size].to_vec();
        let bss = file_bytes[sectors[DATA as usize].offset + sectors[DATA as usize].size..][..bss_size as usize].to_vec();

        return SwitchExecutable {
            program: file_bytes.clone(),
            text,
            ro,
            data,
            bss,
        };
    }
}
