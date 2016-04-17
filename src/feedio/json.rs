// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

//! The feed can be parsed using `FeedReader` and can be written to xml using `FeedWriter`.

use errors;
use rss::{Channel, ChannelBuilder, Category, Cloud, Enclosure, Guid, Image,
    Item, Source, TextInput};
use serde_json;
use std::collections::BTreeMap;

 ///This `FeedReader` struct parses the json feed to the `Channel`.
 pub struct FeedReader {
    channel: Channel,
 }


impl FeedReader {
    /// Construct a new `FeedReader` and return the `Channel`.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::feedio::json::FeedReader;
    ///
    /// let feed_reader = FeedReader::new("String");
    /// ```
    pub fn new(feed: &str) -> FeedReader {

        FeedReader{
            channel: ChannelBuilder::new().finalize(),
        }
    }


    /// Get the `Channel` after parsing.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::feedio::json::FeedReader;
    ///
    /// let feed_reader = FeedReader::new("String");
    /// let channel = feed_reader.channel();
    /// ```
    pub fn channel(self) -> Channel {
        self.channel.clone()
    }
 }

/// This `FeedWriter` struct creates the json from the `Channel`.
#[derive(Default)]
pub struct FeedWriter {
    json: String,
}


impl FeedWriter {
    /// Construct a new `FeedWriter` and return default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::feedio::json::{FeedReader, FeedWriter};
    ///
    /// let feed_reader = FeedReader::new("String");
    /// let channel = feed_reader.channel();
    ///
    /// let feed_writer = FeedWriter::new(channel);
    /// ```
    pub fn new(channel: Channel) -> FeedWriter {
        let xml_str = "xml";
        let mut xml = BTreeMap::new();
        xml.insert("version".to_owned(), "1.0".to_owned());
        xml.insert("encloding".to_owned(), "UTF-8".to_owned());

        let mut rss = BTreeMap::new();
        rss.insert("version".to_owned(), "2.0".to_owned());
        rss.insert("channel".to_owned(), get_channel(channel.clone()));

        let feeds_str = "feed";
        let mut feeds = BTreeMap::new();
        feeds.insert("xml".to_owned(),
                     serde_json::to_string(&xml).expect(&errors::json_to_string_error(xml_str)));

        FeedWriter {
            json: serde_json::to_string(&feeds).expect(&errors::json_to_string_error(feeds_str)),
        }
    }


    /// Convert the `Channel` to JSON.
    ///
    /// # Examples
    ///
    /// ```
    /// use feed::feedio::json::{FeedReader, FeedWriter};
    ///
    /// let feed_reader = FeedReader::new("String");
    /// let channel = feed_reader.channel();
    ///
    /// let json = FeedWriter::new(channel).json();
    /// ```
    pub fn json(&self) -> String {
        self.json.clone()
    }
}

// Retrieve channel converted yo JSON formated String
fn get_channel(channel: Channel) -> String {
    let channel_str = "channel";
    let mut channel_map = BTreeMap::new();
    channel_map.insert("title".to_owned(), channel.title());
    channel_map.insert("link".to_owned(), channel.link());
    channel_map.insert("description".to_owned(), channel.description());
    if channel.language().is_some() {
        channel_map.insert("language".to_owned(), channel.language().unwrap());
    }
    if channel.copyright().is_some() {
        channel_map.insert("copyright".to_owned(), channel.copyright().unwrap());
    }
    if channel.managing_editor().is_some() {
        channel_map.insert("managingEditor".to_owned(),
                           channel.managing_editor().unwrap());
    }
    if channel.web_master().is_some() {
        channel_map.insert("webMaster".to_owned(), channel.web_master().unwrap());
    }
    if channel.pub_date().is_some() {
        channel_map.insert("pubDate".to_owned(),
                           channel.pub_date().unwrap().to_rfc2822());
    }
    if channel.last_build_date().is_some() {
        channel_map.insert("lastBuildDate".to_owned(),
                           channel.last_build_date().unwrap().to_rfc2822());
    }
    if channel.categories().is_some() {
        channel_map.insert("categories".to_owned(),
                           get_categories(channel.categories().unwrap()));
    }
    if channel.generator().is_some() {
        channel_map.insert("generator".to_owned(), channel.generator().unwrap());
    }
    if channel.docs().is_some() {
        channel_map.insert("docs".to_owned(), channel.docs().unwrap());
    }
    if channel.cloud().is_some() {
        channel_map.insert("cloud".to_owned(),
                           get_cloud(channel.cloud().unwrap()));
    }
    if channel.ttl().is_some() {
        channel_map.insert("ttl".to_owned(), channel.ttl().unwrap().to_string());
    }
    if channel.image().is_some() {
        channel_map.insert("image".to_owned(),
                           get_image(channel.image().unwrap()));
    }
    if channel.rating().is_some() {
        channel_map.insert("rating".to_owned(), channel.rating().unwrap());
    }
    if channel.text_input().is_some() {
        channel_map.insert("textInput".to_owned(),
                           get_text_input(channel.text_input().unwrap()));
    }
    if channel.skip_hours().is_some() {
        channel_map.insert("skipHours".to_owned(),
                           get_skip_hours(channel.skip_hours().unwrap()));
    }
    if channel.skip_days().is_some() {
        channel_map.insert("skipDays".to_owned(),
                           get_skip_days(channel.skip_days().unwrap()));
    }
    if channel.items().is_some() {
        channel_map.insert("items".to_owned(),
                           get_items(channel.items().unwrap()));
    }
    serde_json::to_string(&channel_map).expect(&errors::json_to_string_error(channel_str))
}


// Retrieve categories converted to JSON formated String
fn get_categories(categories: Vec<Category>) -> String {
    let categories_str = "categories";
    let mut categories_map = BTreeMap::new();
    for category in categories {
        let category_str = "category";
        let mut category_map = BTreeMap::new();
        category_map.insert(categories_str.to_owned(), category.category());
        if category.domain().is_some() {
            category_map.insert("domain".to_owned(), category.domain().unwrap());
        }
        categories_map.insert(categories_str.to_owned(),
                              serde_json::to_string(&category_map)
                                  .expect(&errors::json_to_string_error(category_str)));
    }
    serde_json::to_string(&categories_map).expect(&errors::json_to_string_error(categories_str))
}


// Retrieve skip days converted to JSON formated String
fn get_skip_days(skip_days: Vec<String>) -> String {
    let skip_days_str = "skipDays";
    let mut skip_days_map = BTreeMap::new();
    for day in skip_days {
        let day_str = "day";
        let mut day_map = BTreeMap::new();
        day_map.insert(day_str.to_owned(), day);
        skip_days_map.insert(skip_days_str.to_owned(),
                              serde_json::to_string(&day_map)
                                  .expect(&errors::json_to_string_error(day_str)));
    }
    serde_json::to_string(&skip_days_map).expect(&errors::json_to_string_error(skip_days_str))
}


// Retrieve skip hours converted to JSON formated String
fn get_skip_hours(skip_hours: Vec<i64>) -> String {
    let skip_hours_str = "skipHours";
    let mut skip_hours_map = BTreeMap::new();
    for hour in skip_hours {
        let hour_str = "hour";
        let mut hour_map = BTreeMap::new();
        hour_map.insert(hour_str.to_owned(), hour);
        skip_hours_map.insert(skip_hours_str.to_owned(),
                              serde_json::to_string(&hour_map)
                                  .expect(&errors::json_to_string_error(hour_str)));
    }
    serde_json::to_string(&skip_hours_map).expect(&errors::json_to_string_error(skip_hours_str))
}


// Retrieve cloud converted to JSON formated String
fn get_cloud(cloud: Cloud) -> String {
    let cloud_str = "cloud";
    let mut cloud_map = BTreeMap::new();
    cloud_map.insert("domain".to_owned(), cloud.domain());
    cloud_map.insert("port".to_owned(), cloud.port().to_string());
    cloud_map.insert("path".to_owned(), cloud.path());
    cloud_map.insert("registerProcedure".to_owned(), cloud.register_procedure());
    cloud_map.insert("protocol".to_owned(), cloud.protocol());

    serde_json::to_string(&cloud_map).expect(&errors::json_to_string_error(cloud_str))
}


// Retrieve image converted to JSON formated String
fn get_image(image: Image) -> String {
    let image_str = "image";
    let mut image_map = BTreeMap::new();
    image_map.insert("title".to_owned(), image.title());
    image_map.insert("link".to_owned(), image.link());
    image_map.insert("width".to_owned(), image.width().to_string());
    image_map.insert("height".to_owned(), image.height().to_string());
    if image.description().is_some() {
        image_map.insert("description".to_owned(), image.description().unwrap());
    }

    serde_json::to_string(&image_map).expect(&errors::json_to_string_error(image_str))
}


// Retrieve text input converted to JSON formated String
fn get_text_input(text_input: TextInput) -> String {
    let text_input_str = "textInput";
    let mut text_input_map = BTreeMap::new();
    text_input_map.insert("title".to_owned(), text_input.title());
    text_input_map.insert("description".to_owned(), text_input.description());
    text_input_map.insert("name".to_owned(), text_input.name());
    text_input_map.insert("link".to_owned(), text_input.link());

    serde_json::to_string(&text_input_map).expect(&errors::json_to_string_error(text_input_str))
}


// Retrieve items converted to JSON formated String
fn get_items(items: Vec<Item>) -> String {
    let items_str = "items";
    let mut items_map = BTreeMap::new();
    for item in items {
        let item_str = "item";
        let mut item_map = BTreeMap::new();
        if item.title().is_some() {
            item_map.insert("title".to_owned(), item.title().unwrap());
        }
        if item.link().is_some() {
            item_map.insert("link".to_owned(), item.link().unwrap());
        }
        if item.description().is_some() {
            item_map.insert("description".to_owned(), item.description().unwrap());
        }
        if item.author().is_some() {
            item_map.insert("author".to_owned(), item.author().unwrap());
        }
        if item.categories().is_some() {
            item_map.insert("categories".to_owned(),
                           get_categories(item.categories().unwrap()));
        }
        if item.comments().is_some() {
            item_map.insert("comments".to_owned(), item.comments().unwrap());
        }
        if item.enclosure().is_some() {
            item_map.insert("enclosure".to_owned(), get_enclosure(item.enclosure().unwrap()));
        }
        if item.guid().is_some() {
            item_map.insert("guid".to_owned(), get_guid(item.guid().unwrap()));
        }
        if item.pub_date().is_some() {
            item_map.insert("pubDate".to_owned(), item.pub_date().unwrap().to_rfc2822());
        }
        if item.source().is_some() {
            item_map.insert("source".to_owned(), get_source(item.source().unwrap()));
        }
        items_map.insert(items_str.to_owned(),
                serde_json::to_string(&item_map).expect(&errors::json_to_string_error(item_str)));
    }
    serde_json::to_string(&items_map).expect(&errors::json_to_string_error(items_str))
}


// Retrieve enclosure converted to JSON formated String
fn get_enclosure(enclosure: Enclosure) -> String {
    let enclosure_str = "enclosure";
    let mut enclosure_map = BTreeMap::new();
    enclosure_map.insert("url".to_owned(), enclosure.url());
    enclosure_map.insert("length".to_owned(), enclosure.length().to_string());
    enclosure_map.insert("type".to_owned(), enclosure.enclosure_type());

    serde_json::to_string(&enclosure_map).expect(&errors::json_to_string_error(enclosure_str))
}


// Retrieve guid converted to JSON formated String
fn get_guid(guid: Guid) -> String {
    let guid_str = "guid";
    let mut guid_map = BTreeMap::new();
    guid_map.insert("permalink".to_owned(), guid.permalink().to_string());
    guid_map.insert("guid".to_owned(), guid.guid());

    serde_json::to_string(&guid_map).expect(&errors::json_to_string_error(guid_str))
}


// Retrieve source converted to JSON formated String
fn get_source(source: Source) -> String {
    let source_str = "source";
    let mut source_map = BTreeMap::new();
    source_map.insert("url".to_owned(), source.url());
    source_map.insert("source".to_owned(), source.source());

    serde_json::to_string(&source_map).expect(&errors::json_to_string_error(source_str))
}
