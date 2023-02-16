/*
 * MIT License
 *
 * termusic - Copyright (c) 2021 Larry Hao
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

pub(crate) mod rust;

use symphonia_core::errors::Result;
use std::{sync::mpsc::{Sender, Receiver, self}, time::Duration};
use crate::{playlist::{Playlist, Status}, config::Config};


pub struct UbiquityPlayer {
    pub message_tx: Sender<PlayerMsg>,
    pub message_rx: Receiver<PlayerMsg>,
    pub playlist: Playlist,
    player: rust::Player,
}

pub enum PlayerMsg {
    Eos,
    AboutToFinish,
    CurrentTrackUpdated,
    Progress(i64, i64),
}

struct PlayTrackOptions {
    track_id: u32,
    seek_ts: u64,
}

impl UbiquityPlayer {
    pub fn new(config: &Config, playlist: Playlist) -> Self {
        let (message_tx, message_rx): (Sender<PlayerMsg>, Receiver<PlayerMsg>) = mpsc::channel();
        let player = rust::Player::new(config, message_tx.clone());

        Self {
            player,
            message_tx,
            message_rx,
            playlist,
        }
    }
    pub fn toggle_gapless(&mut self) -> bool {
        self.player.gapless = !self.player.gapless;
        self.player.gapless
    }

    pub fn start_play(&mut self) {
        if self.playlist.is_stopped() {
            self.playlist.set_status(Status::Running);
            if self.playlist.current_track().is_none() {
                self.playlist.handle_current_track();
            }
        }

        if let Some(file) = self.playlist.get_current_track() {
            if self.playlist.has_next_track() {
                self.playlist.set_next_track(None);
                // eprintln!("next track played");
                #[cfg(not(any(feature = "mpv", feature = "gst")))]
                {
                    self.player.total_duration = Some(self.playlist.next_track_duration());
                    self.player.sink.message_on_end();
                    self.message_tx
                        .send(PlayerMsg::CurrentTrackUpdated)
                        .expect("fail to send track updated signal");
                }
                return;
            }

            self.add_and_play(&file);
            // eprintln!("completely new track added");
            #[cfg(not(any(feature = "mpv", feature = "gst")))]
            {
                self.player.sink.message_on_end();
                self.message_tx
                    .send(PlayerMsg::CurrentTrackUpdated)
                    .expect("fail to send track updated signal");
            }
        }
    }

    pub fn enqueue_next(&mut self) {
        if self.playlist.next_track().is_some() {
            return;
        }

        let track = match self.playlist.fetch_next_track() {
            Some(t) => t.clone(),
            None => return,
        };

        self.playlist.set_next_track(Some(&track));
        if let Some(file) = track.file_path() {
            #[cfg(not(any(feature = "mpv", feature = "gst")))]
            if let Some(d) = self.player.enqueue_next(track.file_path().unwrap()) {
                self.playlist.set_next_track_duration(d);
                // eprintln!("next track queued");
            }
            #[cfg(all(feature = "gst", not(feature = "mpv")))]
            {
                self.player.enqueue_next(file);
                // eprintln!("next track queued");
                self.playlist.set_next_track(None);
                // self.playlist.handle_current_track();
            }

            #[cfg(feature = "mpv")]
            {
                self.player.enqueue_next(file);
                // eprintln!("next track queued");
            }
        }
    }

    pub fn skip(&mut self) {
        if self.playlist.current_track().is_some() {
            self.playlist.set_next_track(None);
            self.player.skip_one();
        } else {
            self.message_tx.send(PlayerMsg::Eos).ok();
        }
    }
}

impl PlayerTrait for UbiquityPlayer {
    fn add_and_play(&mut self, current_track: &str) {
        self.player.add_and_play(current_track);
    }
    fn volume(&self) -> i32 {
        self.player.volume()
    }
    fn volume_up(&mut self) {
        self.player.volume_up();
    }
    fn volume_down(&mut self) {
        self.player.volume_down();
    }
    fn set_volume(&mut self, volume: i32) {
        self.player.set_volume(volume);
    }
    fn pause(&mut self) {
        self.playlist.set_status(Status::Paused);
        self.player.pause();
    }
    fn resume(&mut self) {
        self.playlist.set_status(Status::Running);
        self.player.resume();
    }
    fn is_paused(&self) -> bool {
        self.playlist.is_paused()
    }
    fn seek(&mut self, secs: i64) -> Result<()> {
        self.player.seek(secs)
    }
    fn seek_to(&mut self, last_pos: Duration) {
        self.player.seek_to(last_pos);
    }

    fn set_speed(&mut self, speed: i32) {
        self.player.set_speed(speed);
    }

    fn speed_up(&mut self) {
        self.player.speed_up();
    }

    fn speed_down(&mut self) {
        self.player.speed_down();
    }

    fn speed(&self) -> i32 {
        self.player.speed()
    }

    fn stop(&mut self) {
        self.playlist.set_status(Status::Stopped);
        self.playlist.set_next_track(None);
        self.playlist.set_current_track(None);
        self.player.stop();
    }
}

#[allow(clippy::module_name_repetitions)]
pub trait PlayerTrait {
    fn add_and_play(&mut self, current_track: &str);
    fn volume(&self) -> i32;
    fn volume_up(&mut self);
    fn volume_down(&mut self);
    fn set_volume(&mut self, volume: i32);
    fn pause(&mut self);
    fn resume(&mut self);
    fn is_paused(&self) -> bool;
    fn seek(&mut self, secs: i64) -> Result<()>;
    fn seek_to(&mut self, last_pos: Duration);
    // fn get_progress(&self) -> Result<()>;
    fn set_speed(&mut self, speed: i32);
    fn speed_up(&mut self);
    fn speed_down(&mut self);
    fn speed(&self) -> i32;
    fn stop(&mut self);
}