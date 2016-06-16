/*
 * Copyright 2015-2016 Ben Ashford
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Implementation of ElasticSearch [highlight](https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-highlighting.html)

use std::collections::HashMap;

use serde::ser::{Serialize, Serializer};

macro_rules! serialize_enum {
    ($n:ident) => (
        impl Serialize for $n {
            fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
                where S: Serializer {

                self.to_string().serialize(serializer)
            }
        }
    )
}


#[derive(Debug, Clone)]
pub enum SettingTypes {
    Plain,
    FVH,
    Postings
}

impl ToString for SettingTypes {
    fn to_string(&self) -> String {
        match self {
            &SettingTypes::Plain    => "plain",
            &SettingTypes::FVH      => "fvh",
            &SettingTypes::Postings => "postings"
        }.to_owned()
    }
}

serialize_enum!(SettingTypes);

#[derive(Debug, Clone)]
pub enum IndexOptions {
    Offsets
}

impl ToString for IndexOptions {
    fn to_string(&self) -> String {
        match self {
            &IndexOptions::Offsets => "offsets"
        }.to_owned()
    }
}

serialize_enum!(IndexOptions);

#[derive(Debug, Clone)]
pub enum TermVector {
    WithPositionsOffsets,
    BoundaryChars,
    BoundaryMaxScan,
}

impl ToString for TermVector {
    fn to_string(&self) -> String {
        match self {
            &TermVector::WithPositionsOffsets => "with_positions_offsets",
            &TermVector::BoundaryChars        => "boundary_chars",
            &TermVector::BoundaryMaxScan      => "boundary_max_scan"
        }.to_owned()
    }
}

serialize_enum!(TermVector);

#[derive(Debug, Serialize, Clone)]
pub struct Setting {
    #[serde(rename="type")]
    pub setting_type: Option<SettingTypes>,
    pub index_options: Option<IndexOptions>,
    pub term_vector: Option<TermVector>,
    pub force_source: bool,
    pub fragment_size: u32,
    pub number_of_fragments: u32,
    pub no_match_size: u32,
    pub matched_fields: Vec<String>
}

impl Setting {
    pub fn new() -> Setting {
        Setting {
            setting_type: None,
            index_options: None,
            term_vector: None,
            force_source: false,
            fragment_size: 150,
            number_of_fragments: 5,
            no_match_size: 0,
            matched_fields: vec![]
        }
    }

    pub fn with_type(&mut self, setting_type: SettingTypes) -> &mut Setting {
        self.setting_type = Some(setting_type);
        self
    }

    pub fn with_index_options(&mut self, index_options: IndexOptions) -> &mut Setting {
        self.index_options = Some(index_options);
        self
    }

    pub fn with_term_vector(&mut self, term_vector: TermVector) -> &mut Setting {
        self.term_vector = Some(term_vector);
        self
    }

    pub fn with_force_source(&mut self, force_source: bool) -> &mut Setting {
        self.force_source = force_source;
        self
    }

    pub fn with_fragment_size(&mut self, fragment_size: u32) -> &mut Setting {
        self.fragment_size = fragment_size;
        self
    }

    pub fn with_number_of_fragments(&mut self, number_of_fragments: u32) -> &mut Setting {
        self.number_of_fragments = number_of_fragments;
        self
    }

    pub fn with_no_match_size(&mut self, no_match_size: u32) -> &mut Setting {
        self.no_match_size = no_match_size;
        self
    }

    pub fn with_matched_fields<'a>(&mut self, matched_fields: Vec<String>) -> &mut Setting {
        self.matched_fields = matched_fields;
        self
    }
}

#[derive(Debug, Serialize)]
pub struct Highlight {
    pub fields: HashMap<String, Setting>
}

impl Highlight {
    /// Create an Highlight entity without any field or setting
    /// specified as they are supposed to be added via the `add`
    /// method.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_es::operations::search::highlight::{Highlight, Setting, SettingTypes};
    ///
    /// let mut highlight = Highlight::new();
    /// let setting = Setting::new().with_type(SettingTypes::Plain).to_owned();
    /// highlight.add("first_name".to_owned(), setting);
    /// ```
    pub fn new() -> Highlight {
        Highlight { fields: HashMap::new() }
    }

    /// Add a field to highlight to the set
    pub fn add(&mut self, name: String, setting: Setting) {
        self.fields.insert(name, setting);
    }
}

/// The fields containing found terms
pub type HighlightResult = HashMap<String, Vec<String>>;
