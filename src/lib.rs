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

fn c_str_to_str(c_str: *const c_char) -> String {
  if c_str.is_null() {
    String::new()
  }
  else {
    let bytes = unsafe { CStr::from_ptr(c_str).to_bytes() };
    String::from_utf8_lossy(bytes).to_string()
  }
}

/// A representation of an audio file, with meta-data and properties.
pub struct File {
  raw: *mut ll::TagLib_File,
}

/// The abstract meta-data container for audio files
///
/// Each `Tag` instance can only be created by the `taglib::File::tag()`
/// method.
#[allow(dead_code)]
pub struct Tag<'a> {
  raw: *mut ll::TagLib_Tag,
  file: &'a File,
}

/// Common audio file properties.
///
/// Instances of `AudioProperties` can only be created through the
/// `taglib::File::audioproperties()` method.
#[allow(dead_code)]
pub struct AudioProperties<'a> {
  raw: *const ll::TagLib_AudioProperties,
  file: &'a File,
}

impl<'a> Tag<'a> {
  /// Returns the track name, or an empty string if no track name is present.
  pub fn title(&self) -> String {
    let res = unsafe { ll::taglib_tag_title(self.raw) };
    c_str_to_str(res)
  }

  /// Sets the track name.
  pub fn set_title(&mut self, title: &str) {
    let s = CString::new(title).unwrap().as_ptr();
    unsafe { ll::taglib_tag_set_title(self.raw, s); }
  }

  /// Returns the artist name, or an empty string if no artist name is present.
  pub fn artist(&self) -> String {
    let res = unsafe { ll::taglib_tag_artist(self.raw) };
    c_str_to_str(res)
  }

  /// Sets the artist name.
  pub fn set_artist(&mut self, artist: &str) {
    let s = CString::new(artist).unwrap().as_ptr();
    unsafe { ll::taglib_tag_set_artist(self.raw, s); }
  }

  /// Returns the album name, or an empty string if no album name is present.
  pub fn album(&self) -> String {
    let res = unsafe { ll::taglib_tag_album(self.raw) };
    c_str_to_str(res)
  }

  /// Sets the album name.
  pub fn set_album(&mut self, album: &str) {
    let s = CString::new(album).unwrap().as_ptr();
    unsafe { ll::taglib_tag_set_album(self.raw, s); }
  }

  /// Returns the track comment, or an empty string if no track comment is
  /// present.
  pub fn comment(&self) -> String {
    let res = unsafe { ll::taglib_tag_comment(self.raw) };
    c_str_to_str(res)
  }

  /// Sets the track comment.
  pub fn set_comment(&mut self, comment: &str) {
    let s = CString::new(comment).unwrap().as_ptr();
    unsafe { ll::taglib_tag_set_comment(self.raw, s); }
  }

  /// Returns the genre name, or an empty string if no genre name is present.
  pub fn genre(&self) -> String {
    let res = unsafe { ll::taglib_tag_genre(self.raw) };
    c_str_to_str(res)
  }

  /// Sets the genre name.
  pub fn set_genre(&mut self, genre: &str) {
    let s = CString::new(genre).unwrap().as_ptr();
    unsafe { ll::taglib_tag_set_genre(self.raw, s); }
  }

  /// Returns the year, or 0 if no year is present.
  pub fn year(&self) -> u32 {
    unsafe { ll::taglib_tag_year(self.raw) as u32 }
  }

  /// Sets the year.
  pub fn set_year(&mut self, year: u32) {
    unsafe { ll::taglib_tag_set_year(self.raw, year); }
  }

  /// Returns the track number, or 0 if no track number is present.
  pub fn track(&self) -> u32 {
    unsafe { ll::taglib_tag_track(self.raw) as u32 }
  }

  /// Sets the track number.
  pub fn set_track(&mut self, track: u32) {
    unsafe { ll::taglib_tag_set_track(self.raw, track); }
  }
}

impl<'a> AudioProperties<'a> {
  /// Returns the length, in seconds, of the track.
  pub fn length(&self) -> u32 {
    unsafe { ll::taglib_audioproperties_length(self.raw) as u32 }
  }

  /// Returns the most appropriate bit rate for the track, in kB/s.
  /// For constant bit rate formats, the returned value is the bit
  /// rate of the file; for variable bit rate formats this is either
  /// the average or the nominal bit rate.
  pub fn bitrate(&self) -> u32 {
    unsafe { ll::taglib_audioproperties_bitrate(self.raw) as u32 }
  }

  /// Returns the sample rate, in Hz.
  pub fn samplerate(&self) -> u32 {
    unsafe { ll::taglib_audioproperties_samplerate(self.raw) as u32 }
  }

  /// Returns the number of audio channels.
  pub fn channels(&self) -> u32 {
    unsafe { ll::taglib_audioproperties_channels(self.raw) as u32 }
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
        Ok(s) => s.as_ptr(),
        _ => return Err(FileError::InvalidFileName)
      };

    let f = unsafe { ll::taglib_file_new(filename_c) };
    if f.is_null() {
      return Err(FileError::InvalidFile);
    }

    Ok(File { raw: f })
  }

  /// Creates a new `taglib::File` for the given `filename` and type of file.
  pub fn new_type(filename: &str, filetype: FileType) -> Result<File, FileError> {
    let filename_c =
      match CString::new(filename) {
        Ok(s) => s.as_ptr(),
        _ => return Err(FileError::InvalidFileName)
      };

    let f = unsafe { ll::taglib_file_new_type(filename_c, filetype as u32) };
    if f.is_null() {
      return Err(FileError::InvalidFile);
    }

    Ok(File { raw: f })
  }

  /// Returns the `taglib::Tag` instance for the given file.
  pub fn tag(&self) -> Result<Tag, FileError> {
    let res = unsafe { ll::taglib_file_tag(self.raw) };

    if res.is_null() {
      Err(FileError::NoAvailableTag)
    }
    else {
      Ok(Tag { raw: res, file: self })
    }
  }

  /// Returns whether the file is valid.
  pub fn is_valid(&self) -> bool {
    unsafe { ll::taglib_file_is_valid(self.raw) != 0 }
  }

  /// Returns the `taglib::AudioProperties` instance for the given file.
  pub fn audioproperties(&self) -> Result<AudioProperties, FileError> {
    let res = unsafe { ll::taglib_file_audioproperties(self.raw) };

    if res.is_null() {
      Err(FileError::NoAvailableAudioProperties)
    }
    else {
      Ok(AudioProperties { raw: res, file: self })
    }
  }

  /// Updates the meta-data of the file.
  pub fn save(&self) -> bool {
    unsafe { ll::taglib_file_save(self.raw) != 0 }
  }
}
