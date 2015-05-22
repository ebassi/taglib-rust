#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_char};
use std::ffi::{CString, CStr};
use std::ptr;

pub mod taglib_sys;

use taglib_sys as ll;

fn c_str_to_str(c_str: *const c_char) -> String {
  if c_str.is_null() {
    String::new()
  }
  else {
    let bytes = unsafe { CStr::from_ptr(c_str).to_bytes() };
    String::from_utf8_lossy(bytes).to_string()
  }
}

pub struct Tag {
  raw: *mut ll::TagLib_Tag,
}

impl Drop for Tag {
  fn drop(&mut self) {
    unsafe { ll::taglib_tag_free_strings() }
  }
}

impl Tag {
  pub fn title(&self) -> String {
    let res = unsafe { ll::taglib_tag_title(self.raw) };
    c_str_to_str(res)
  }

  pub fn artist(&self) -> String {
    let res = unsafe { ll::taglib_tag_artist(self.raw) };
    c_str_to_str(res)
  }

  pub fn album(&self) -> String {
    let res = unsafe { ll::taglib_tag_album(self.raw) };
    c_str_to_str(res)
  }

  pub fn comment(&self) -> String {
    let res = unsafe { ll::taglib_tag_comment(self.raw) };
    c_str_to_str(res)
  }

  pub fn genre(&self) -> String {
    let res = unsafe { ll::taglib_tag_genre(self.raw) };
    c_str_to_str(res)
  }

  pub fn year(&self) -> i32 {
    unsafe { ll::taglib_tag_year(self.raw) as i32 }
  }

  pub fn track(&self) -> i32 {
    unsafe { ll::taglib_tag_track(self.raw) as i32 }
  }
}

pub struct AudioProperties {
  raw: *const ll::TagLib_AudioProperties,
}

impl AudioProperties {
  pub fn length(&self) -> u32 {
    unsafe { ll::taglib_audioproperties_length(self.raw) as u32 }
  }

  pub fn bitrate(&self) -> u32 {
    unsafe { ll::taglib_audioproperties_bitrate(self.raw) as u32 }
  }

  pub fn samplerate(&self) -> u32 {
    unsafe { ll::taglib_audioproperties_samplerate(self.raw) as u32 }
  }

  pub fn channels(&self) -> u32 {
    unsafe { ll::taglib_audioproperties_channels(self.raw) as u32 }
  }
}

#[derive(Copy, Clone, PartialEq)]
pub enum FileType {
  MPEG = ll::TAGLIB_FILE_MPEG as isize,
  OggVorbis = ll::TAGLIB_FILE_OGG_VORBIS as isize,
  FLAC = ll::TAGLIB_FILE_FLAC as isize,
  MPC = ll::TAGLIB_FILE_MPC as isize,
  OggFlac = ll::TAGLIB_FILE_OGG_FLAC as isize,
  WavPack = ll::TAGLIB_FILE_WAV_PACK as isize,
  Speex = ll::TAGLIB_FILE_SPEEX as isize,
  TrueAudio = ll::TAGLIB_FILE_TRUE_AUDIO as isize,
  MP4 = ll::TAGLIB_FILE_MP4 as isize,
  ASF = ll::TAGLIB_FILE_ASF as isize
}

#[derive(Debug)]
pub enum FileError {
  InvalidFile,
  InvalidFileName,
  NoAvailableTag,
  NoAvailableAudioProperties
}

pub struct File {
  raw: *mut ll::TagLib_File,
}

impl Drop for File {
  fn drop(&mut self) {
    unsafe { ll::taglib_file_free(self.raw); }
  }
}

impl File {
  pub fn new(filename: &str) -> Result<File, FileError> {
    let filename_c =
      match CString::new(filename) {
        Ok(s) => s.as_ptr(),
        _ => return Err(FileError::InvalidFileName),
      };

      let f = unsafe { ll::taglib_file_new(filename_c) };
      if f == ptr::null_mut() {
        return Err(FileError::InvalidFile);
      }

      Ok(File { raw: f })
  }

  pub fn tag(&self) -> Result<Tag, FileError> {
    let res = unsafe { ll::taglib_file_tag(self.raw) };

    if res.is_null() {
      Err(FileError::NoAvailableTag)
    }
    else {
      Ok(Tag { raw: res })
    }
  }

  pub fn is_valid(&self) -> bool {
    unsafe { ll::taglib_file_is_valid(self.raw) != 0 }
  }

  pub fn audioproperties(&self) -> Result<AudioProperties, FileError> {
    let res = unsafe { ll::taglib_file_audioproperties(self.raw) };

    if res.is_null() {
      Err(FileError::NoAvailableAudioProperties)
    }
    else {
      Ok(AudioProperties { raw: res })
    }
  }
}

pub fn set_strings_unicode(value: bool) {
  unsafe { ll::taglib_set_strings_unicode(value as ll::TagLib_Bool); }
}
