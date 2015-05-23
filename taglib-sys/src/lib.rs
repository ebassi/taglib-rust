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

#![allow(non_camel_case_types)]
extern crate libc;

use libc::{c_int, c_uint, c_char, c_void};

// Public types; these are all opaque pointer types
pub type TagLib_File = c_void;
pub type TagLib_Tag = c_void;
pub type TagLib_AudioProperties = c_void;

pub type TagLib_Bool = c_int;
pub type TagLib_FileType = c_uint;

pub const TAGLIB_FILE_MPEG: TagLib_FileType = 0;
pub const TAGLIB_FILE_OGG_VORBIS: TagLib_FileType = 1;
pub const TAGLIB_FILE_FLAC: TagLib_FileType = 2;
pub const TAGLIB_FILE_MPC: TagLib_FileType = 3;
pub const TAGLIB_FILE_OGG_FLAC: TagLib_FileType = 4;
pub const TAGLIB_FILE_WAV_PACK: TagLib_FileType = 5;
pub const TAGLIB_FILE_SPEEX: TagLib_FileType = 6;
pub const TAGLIB_FILE_TRUE_AUDIO: TagLib_FileType = 7;
pub const TAGLIB_FILE_MP4: TagLib_FileType = 8;
pub const TAGLIB_FILE_ASF: TagLib_FileType = 9;

// tag_c.h
extern "C" {
  pub fn taglib_file_new(filename: *const c_char) -> *mut TagLib_File;
  pub fn taglib_file_new_type(filename: *const c_char, filetype: TagLib_FileType) -> *mut TagLib_File;
  pub fn taglib_file_is_valid(file: *mut TagLib_File) -> TagLib_Bool;
  pub fn taglib_file_free(file: *mut TagLib_File);
  pub fn taglib_file_save(file: *mut TagLib_File) -> TagLib_Bool;
  pub fn taglib_file_tag(file: *mut TagLib_File) -> *mut TagLib_Tag;
  pub fn taglib_file_audioproperties(file: *mut TagLib_File) -> *const TagLib_AudioProperties;

  pub fn taglib_tag_title(tag: *const TagLib_Tag) -> *const c_char;
  pub fn taglib_tag_artist(tag: *const TagLib_Tag) -> *const c_char;
  pub fn taglib_tag_album(tag: *const TagLib_Tag) -> *const c_char;
  pub fn taglib_tag_comment(tag: *const TagLib_Tag) -> *const c_char;
  pub fn taglib_tag_genre(tag: *const TagLib_Tag) -> *const c_char;
  pub fn taglib_tag_year(tag: *const TagLib_Tag) -> c_uint;
  pub fn taglib_tag_track(tag: *const TagLib_Tag) -> c_uint;
  pub fn taglib_tag_set_title(tag: *mut TagLib_Tag, title: *const c_char);
  pub fn taglib_tag_set_artist(tag: *mut TagLib_Tag, artist: *const c_char);
  pub fn taglib_tag_set_album(tag: *mut TagLib_Tag, album: *const c_char);
  pub fn taglib_tag_set_comment(tag: *mut TagLib_Tag, comment: *const c_char);
  pub fn taglib_tag_set_genre(tag: *mut TagLib_Tag, genre: *const c_char);
  pub fn taglib_tag_set_year(tag: *mut TagLib_Tag, year: c_uint);
  pub fn taglib_tag_set_track(tag: *mut TagLib_Tag, track: c_uint);
  pub fn taglib_tag_free_strings();

  pub fn taglib_audioproperties_length(properties: *const TagLib_AudioProperties) -> c_int;
  pub fn taglib_audioproperties_bitrate(properties: *const TagLib_AudioProperties) -> c_int;
  pub fn taglib_audioproperties_samplerate(properties: *const TagLib_AudioProperties) -> c_int;
  pub fn taglib_audioproperties_channels(properties: *const TagLib_AudioProperties) -> c_int;
}
