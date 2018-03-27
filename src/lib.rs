// Copyright 2015  Emmanuele Bassi. All rights reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

#![crate_name = "taglib"]
#![crate_type = "lib"]

extern crate libc;
extern crate taglib_sys as sys;

use libc::{c_char};
use std::ffi::{CString, CStr};

use sys as ll;

fn c_str_to_str(c_str: *const c_char) -> Option<String> {
  if c_str.is_null() {
    return None;
  }

  let bytes = unsafe { CStr::from_ptr(c_str).to_bytes() };
  if bytes.is_empty() {
    None
  } else {
    Some(String::from_utf8_lossy(bytes).to_string())
  }
}

fn u32_to_option(n: u32) -> Option<u32> {
  if n == 0 { None } else { Some(n) }
}

fn str_to_c_str(s: Option<String>) -> *const c_char {
    let s = s.unwrap_or("".into());

    CString::new(s.as_str()).unwrap().as_ptr()
}

/// A representation of an audio file, with meta-data and properties.
pub struct File {
  raw: *mut ll::TagLib_File,
}

/// Represents audio tag metadata.
pub struct Tag {
  /// The title of the track, if available.
  pub title: Option<String>,
  /// The album name, if available.
  pub album: Option<String>,
  /// The artist name, if available.
  pub artist: Option<String>,
  /// An additional comment, if available.
  pub comment: Option<String>,
  /// The genre, if any.
  pub genre: Option<String>,
  /// The year the track was created, if available.
  pub year: Option<u32>,
  /// The track number, if available.
  pub track: Option<u32>,
}

impl Tag {
  unsafe fn new(raw: *const ll::TagLib_Tag) -> Tag {
    Tag {
      title: c_str_to_str(ll::taglib_tag_title(raw)),
      album: c_str_to_str(ll::taglib_tag_album(raw)),
      artist: c_str_to_str(ll::taglib_tag_artist(raw)),
      comment: c_str_to_str(ll::taglib_tag_comment(raw)),
      genre: c_str_to_str(ll::taglib_tag_genre(raw)),
      year: u32_to_option(ll::taglib_tag_year(raw) as u32),
      track: u32_to_option(ll::taglib_tag_track(raw) as u32)
    }
  }
}

/// Common audio file properties.
pub struct AudioProperties {
  /// The length, in seconds, of the track.
  pub length: u32,
  /// The most appropriate bit rate for the track, in KB/s.
  /// For constant bit rate formats, the value is the bit rate of the file;
  /// for variable bit rate formats this is either the average or the nominal
  /// bit rate.
  pub bitrate: u32,
  /// The sample rate in Hz.
  pub samplerate: u32,
  /// The number of audio channels.
  pub channels: u32,
}

impl AudioProperties {
  unsafe fn new(raw: *const ll::TagLib_AudioProperties) -> AudioProperties {
    AudioProperties {
      length: ll::taglib_audioproperties_length(raw) as u32,
      bitrate: ll::taglib_audioproperties_bitrate(raw) as u32,
      samplerate: ll::taglib_audioproperties_samplerate(raw) as u32,
      channels: ll::taglib_audioproperties_channels(raw) as u32,
    }
  }
}

#[derive(Copy, Clone, PartialEq)]
pub enum FileType {
  /// MPEG file
  MPEG = ll::TAGLIB_FILE_MPEG as isize,
  /// Ogg/Vorbis file
  OggVorbis = ll::TAGLIB_FILE_OGG_VORBIS as isize,
  /// FLAC file
  FLAC = ll::TAGLIB_FILE_FLAC as isize,
  /// MPC file
  MPC = ll::TAGLIB_FILE_MPC as isize,
  /// Ogg/FLAC file
  OggFlac = ll::TAGLIB_FILE_OGG_FLAC as isize,
  /// WavPack file
  WavPack = ll::TAGLIB_FILE_WAV_PACK as isize,
  /// Ogg/Speex file
  Speex = ll::TAGLIB_FILE_SPEEX as isize,
  /// TrueAudio file
  TrueAudio = ll::TAGLIB_FILE_TRUE_AUDIO as isize,
  /// MP4 file
  MP4 = ll::TAGLIB_FILE_MP4 as isize,
  /// ASF file
  ASF = ll::TAGLIB_FILE_ASF as isize
}

#[derive(Debug)]
pub enum FileError {
  /// The file is an invalid or an unrecognized audio container
  InvalidFile,
  /// The file name is invalid
  InvalidFileName,
  /// No meta-data is available
  NoAvailableTag,
  /// No audio properties are available
  NoAvailableAudioProperties
}

impl Drop for File {
  fn drop(&mut self) {
    unsafe { ll::taglib_file_free(self.raw); }
  }
}

impl File {
  /// Creates a new `taglib::File` for the given `filename`.
  pub fn new(filename: &str) -> Result<File, FileError> {
    let filename_c =
      match CString::new(filename) {
        Ok(s) => s,
        _ => return Err(FileError::InvalidFileName)
      };

    let filename_c_ptr = filename_c.as_ptr();

    let f = unsafe { ll::taglib_file_new(filename_c_ptr) };
    if f.is_null() {
      return Err(FileError::InvalidFile);
    }

    Ok(File { raw: f })
  }

  /// Creates a new `taglib::File` for the given `filename` and type of file.
  pub fn new_type(filename: &str, filetype: FileType) -> Result<File, FileError> {
    let filename_c =
      match CString::new(filename) {
        Ok(s) => s,
        _ => return Err(FileError::InvalidFileName)
      };

    let filename_c_ptr = filename_c.as_ptr();
    let f = unsafe { ll::taglib_file_new_type(filename_c_ptr, filetype as u32) };
    if f.is_null() {
      return Err(FileError::InvalidFile);
    }

    Ok(File { raw: f })
  }

  /// Returns whether the file is valid.
  pub fn is_valid(&self) -> bool {
    unsafe { ll::taglib_file_is_valid(self.raw) != 0 }
  }

  /// Returns the `taglib::Tag` instance for the given file.
  pub fn tag(&self) -> Result<Tag, FileError> {
    let raw = unsafe { ll::taglib_file_tag(self.raw) };

    if raw.is_null() {
      Err(FileError::NoAvailableTag)
    } else {
      let tag = unsafe { Tag::new(raw) };
      Ok(tag)
    }
  }


  /// Returns the `taglib::AudioProperties` instance for the given file.
  pub fn audioproperties(&self) -> Result<AudioProperties, FileError> {
    let raw = unsafe { ll::taglib_file_audioproperties(self.raw) };

    if raw.is_null() {
      Err(FileError::NoAvailableAudioProperties)
    } else {
      let props = unsafe { AudioProperties::new(raw) };
      Ok(props)
    }
  }

  /// Write the given tag to the audio file.
  pub fn save(&self, tag: Tag) -> bool {
      unsafe {
        let raw_tag = ll::taglib_file_tag(self.raw);
        // if the user managed to get a tag to pass in then this should work
        assert!(!raw_tag.is_null());
        ll::taglib_tag_set_title(raw_tag, str_to_c_str(tag.title));
        ll::taglib_tag_set_album(raw_tag, str_to_c_str(tag.album));
        ll::taglib_tag_set_artist(raw_tag, str_to_c_str(tag.artist));
        ll::taglib_tag_set_genre(raw_tag, str_to_c_str(tag.genre));
        ll::taglib_tag_set_comment(raw_tag, str_to_c_str(tag.comment));
        ll::taglib_tag_set_year(raw_tag, tag.year.unwrap_or(0));
        ll::taglib_tag_set_track(raw_tag, tag.track.unwrap_or(0));
        ll::taglib_file_save(self.raw) != 0
      }
  }
}
