//! # csb_lib
//!
//! csb_lib is a library for reading and writing `commonsoundtable.csb` files from Super Smash Bros. Ultimate.
use std::{
    fs,
    io::{Cursor, Read, Seek, Write},
    path::Path,
};

use binrw::{binrw, BinReaderExt, BinResult, BinWrite};
pub use hash40::Hash40;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The container type for common fighter sounds.
#[binrw]
#[brw(magic = b"CSB\0", little)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug)]
pub struct CsbFile {
    #[br(temp)]
    #[bw(calc = 1u32)]
    unk1: u32,

    #[br(temp)]
    #[bw(calc = entries.get(0).map(|entry| entry.sound_table.len()).unwrap_or(0) as u16)]
    sound_num: u16,

    #[br(temp)]
    #[bw(calc = entries.len() as u16)]
    fighter_num: u16,

    #[br(args { inner: (sound_num,) }, count = fighter_num)]
    pub entries: Vec<CsbEntry>,
}

impl CsbFile {
    /// Reads the data from the given reader.
    pub fn read<R: Read + Seek>(reader: &mut R) -> BinResult<Self> {
        let csb = reader.read_le::<Self>()?;

        Ok(csb)
    }

    /// Reads the data from the given file path.
    pub fn from_file<P: AsRef<Path>>(path: P) -> BinResult<Self> {
        let mut file = Cursor::new(fs::read(path)?);
        let csb = file.read_le::<Self>()?;

        Ok(csb)
    }

    /// Writes the data to the given writer.
    pub fn write<W: Write + Seek>(&self, writer: &mut W) -> BinResult<()> {
        self.write_le(writer)
    }

    /// Writes the data to the given file path.
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> BinResult<()> {
        let mut cursor = Cursor::new(Vec::new());

        self.write_le(&mut cursor)?;
        fs::write(path, cursor.get_mut())?;

        Ok(())
    }
}

/// A collection of common sound labels for a particular fighter.
#[binrw]
#[br(import(sound_num: u16))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug)]
pub struct CsbEntry {
    /// Hashed name of the fighter.
    pub fighter_kind: Hash40,

    /// Collection of common sound labels associated with the fighter.
    #[br(count = sound_num)]
    pub sound_table: Vec<Hash40>,
}
