// SPDX-License-Identifier: MIT
// Copyright 2021 Daniel Vogelbacher <daniel@chaospixel.com>

use super::super::{BoxHeader, FourCC, ReadBox, Result, read_box_header_ext};
use serde::{Deserialize, Serialize};
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct JpegBox {
  pub header: BoxHeader,
  pub version: u8,
  pub flags: u32,
}

impl JpegBox {
  pub const TYP: FourCC = FourCC::with(['J', 'P', 'E', 'G']);
}

impl<R: Read + Seek> ReadBox<&mut R> for JpegBox {
  fn read_box(reader: &mut R, header: BoxHeader) -> Result<Self> {
    let (version, flags) = read_box_header_ext(reader)?;

    reader.seek(SeekFrom::Start(header.end_offset()))?;

    Ok(Self { header, version, flags })
  }
}
