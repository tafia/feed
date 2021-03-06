// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! # feed 1.2.0
//!
//! This Library is for parsing through a rss field and creating a `Feed`
//! struct containing all elements of a `Channel` based on the rss spec.
//!
//! ## Usage
//! Put this in your Cargo.toml:
//!
//! ```Toml
//! [dependencies]
//! feedreader = "1.2.0"
//! ```
//!
//! And put this in your crate root:
//!
//! ```
//! extern crate feed;
//! ```
//!
//! ## Examples
//!
//! ### Reading Feeds
//!
//! ```
//! extern crate feed;
//! extern crate url;
//!
//! use feed::FeedBuilder;
//! use url::Url;
//!
//! fn main() {
//!     let url_str = "http://feeds2.feedburner.com/TheLinuxActionShowOGG.xml";
//!     let url = Url::parse(url_str).expect("Url parse Error");
//!     let feed = FeedBuilder::new().read_from_url(url).finalize();
//!     let channel = feed.channel();
//!     println!("Title: {}", channel.title());
//! }
//! ```
//!
//! ### Writing Feeds
//!
//! ```
//! extern crate feed;
//!
//! use feed::FeedBuilder;
//! use feed::rss::ChannelBuilder;
//!
//! fn main() {
//!
//!     let description = "Ogg Vorbis audio versions of The Linux ".to_owned()
//!         + "Action Show! A show that covers everything geeks care about in "
//!         + "the computer industry. Get a solid dose of Linux, gadgets, news "
//!         + "events and much more!";
//!
//!     let channel = ChannelBuilder::new()
//!             .title("The Linux Action Show! OGG")
//!             .link("http://www.jupiterbroadcasting.com")
//!             .description(description.as_ref())
//!             .finalize();
//!     let feed = FeedBuilder::new().channel(channel).finalize();
//!     let xml = feed.to_xml();
//!     println!("Feed: {:?}", xml);
//! }
//! ```

#![doc(html_root_url = "https://red-oxide.github.io/feed/")]

#![deny(missing_docs)]

pub mod feedio;
pub mod rss;
mod util;
mod errors;

extern crate chrono;
extern crate curl;
extern crate quick_xml;
extern crate url;

#[macro_use]
extern crate log;

use curl::http;
use feedio::{FeedReader, FeedWriter};
use rss::Channel;
use std::str;
use url::Url;

/// This `Feed` struct contains all the items that exist for the feeds.
#[derive(Clone)]
pub struct Feed {
    channel: Channel,
}


impl Feed {
    /// Get the `Channel` that exists under `Feed`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{Feed, FeedBuilder};
    /// use feed::rss::Channel;
    ///
    /// let feed = FeedBuilder::new().finalize();
    /// let channel = feed.channel();
    /// ```
    pub fn channel(self) -> Channel {
        self.channel
    }


    /// Convert the `Feed` to XML.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::{Feed, FeedBuilder};
    /// use feed::rss::Channel;
    ///
    /// let feed = FeedBuilder::new().finalize();
    /// let xml = feed.to_xml();
    /// ```
    pub fn to_xml(&self) -> Vec<u8> {
        FeedWriter::new(self.channel.clone()).xml()
    }
}

/// This `FeedBuilder` struct creates the Feed struct from url, file, or &str.
#[derive(Default)]
pub struct FeedBuilder {
    channel: Channel,
}


impl FeedBuilder {
    /// Construct a new `FeedBuilder` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::FeedBuilder;
    ///
    /// let feed = FeedBuilder::new().finalize();
    /// ```
    pub fn new() -> FeedBuilder {
        FeedBuilder::default()
    }


    /// Construct a new `FeedBuilder` from a `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::FeedBuilder;
    /// use feed::rss::{Channel, ChannelBuilder};
    ///
    /// let channel = ChannelBuilder::new().finalize();
    /// let feed = FeedBuilder::new().channel(channel).finalize();
    /// ```
    pub fn channel(&mut self, channel: Channel) -> &mut FeedBuilder {
        self.channel = channel;
        self
    }


    /// Construct a new `FeedBuilder` from a `Url`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate feed;
    /// extern crate url;
    ///
    /// use feed::FeedBuilder;
    /// use url::Url;
    /// fn main() {
    ///     let url_str = "http://feeds2.feedburner.com/TheLinuxActionShowOGG.xml";
    ///     let url = Url::parse(url_str).expect("Url parse Error");
    ///     let feed = FeedBuilder::new().read_from_url(url).finalize();
    ///     let channel = feed.channel();
    ///
    ///     let description = "Ogg Vorbis audio versions of The Linux ".to_owned()
    ///         + "Action Show! A show that covers everything geeks care about "
    ///         + "in the computer industry. Get a solid dose of Linux, "
    ///         + "gadgets, news events and much more!";
    ///
    ///     let generator = "Feeder 2.5.12(2294); ".to_owned()
    ///         + "Mac OS X Version 10.9.5 (Build 13F34) "
    ///         + "http://reinventedsoftware.com/feeder/";
    ///
    ///     assert_eq!("The Linux Action Show! OGG".to_owned(), channel.title());
    ///     assert_eq!("http://www.jupiterbroadcasting.com".to_owned(), channel.link());
    ///     assert_eq!(description.as_ref(), channel.description());
    ///     assert_eq!(Some(generator), channel.generator());
    ///     assert_eq!(Some("http://blogs.law.harvard.edu/tech/rss".to_owned()), channel.docs());
    ///     assert_eq!(Some("en".to_owned()), channel.language());
    ///     assert!(channel.copyright().is_none());
    ///     assert!(channel.managing_editor().is_none());
    ///     assert!(channel.web_master().is_none());
    /// }
    /// ```
    pub fn read_from_url(&mut self, feed_url: Url) -> &mut FeedBuilder {
        if !feed_url.serialize().as_str().ends_with(".xml") {
            panic!(errors::missing_xml_error());
        }
        let response = http::handle()
                           .get(feed_url.serialize())
                           .exec()
                           .expect(errors::response_error());
        let body = response.get_body();
        let feed_str = str::from_utf8(body).expect(errors::utf8_to_str_error());
        debug!("feed xml:{}", feed_str);
        self.channel = FeedReader::new(feed_str).channel();
        self
    }


    /// Construct the `Feed` from the `FeedBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::FeedBuilder;
    ///
    /// let feed = FeedBuilder::new().finalize();
    /// ```
    pub fn finalize(&self) -> Feed {
        Feed { channel: self.channel.clone() }
    }
}
