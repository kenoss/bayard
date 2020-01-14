use log::*;
use serde_json::Value;
use tantivy::tokenizer::TokenizerManager;

use crate::tokenizer::alpha_num_only_filter_factory::AlphaNumOnlyFilterFactory;
use crate::tokenizer::ascii_folding_filter_factory::AsciiFoldingFilterFactory;
use crate::tokenizer::facet_tokenizer_factory::FacetTokenizerFactory;
use crate::tokenizer::lower_case_filter_factory::LowerCaseFilterFactory;
use crate::tokenizer::ngram_tokenizer_factory::NgramTokenizerFactory;
use crate::tokenizer::raw_tokenizer_factory::RawTokenizerFactory;
use crate::tokenizer::remove_long_filter_factory::RemoveLongFilterFactory;
use crate::tokenizer::simple_tokenizer_factory::SimpleTokenizerFactory;
use crate::tokenizer::stemming_filter_factory::StemmingFilterFactory;
use crate::tokenizer::stop_word_filter_factory::StopWordFilterFactory;

pub struct TokenizerInitializer {
    facet_tokenizer_factory: FacetTokenizerFactory,
    ngram_tokenizer_factory: NgramTokenizerFactory,
    raw_tokenizer_factory: RawTokenizerFactory,
    simple_tokenizer_factory: SimpleTokenizerFactory,

    alpha_num_only_filter_factory: AlphaNumOnlyFilterFactory,
    ascii_folding_filter_factory: AsciiFoldingFilterFactory,
    lower_case_filter_factory: LowerCaseFilterFactory,
    remove_long_filter_factory: RemoveLongFilterFactory,
    stemming_filter_factory: StemmingFilterFactory,
    stop_word_filter_factory: StopWordFilterFactory,
}

impl TokenizerInitializer {
    pub fn new() -> Self {
        TokenizerInitializer {
            facet_tokenizer_factory: FacetTokenizerFactory::new(),
            ngram_tokenizer_factory: NgramTokenizerFactory::new(),
            raw_tokenizer_factory: RawTokenizerFactory::new(),
            simple_tokenizer_factory: SimpleTokenizerFactory::new(),

            alpha_num_only_filter_factory: AlphaNumOnlyFilterFactory::new(),
            ascii_folding_filter_factory: AsciiFoldingFilterFactory::new(),
            lower_case_filter_factory: LowerCaseFilterFactory::new(),
            remove_long_filter_factory: RemoveLongFilterFactory::new(),
            stemming_filter_factory: StemmingFilterFactory::new(),
            stop_word_filter_factory: StopWordFilterFactory::new(),
        }
    }

    pub fn init(&mut self, manager: &TokenizerManager, config: &str) {
        let config_value: Value = serde_json::from_str(config).unwrap();

        let config_map = config_value.as_object().unwrap();
        for (name, tokenizer_config_value) in config_map {
            debug!("name: {}", name);

            let tokenizer_config_map = tokenizer_config_value.as_object().unwrap();

            // tokenizer
            let tokenizer_settings = tokenizer_config_map["tokenizer"].as_object().unwrap();
            debug!("tokenizer_setting: {:?}", tokenizer_settings);

            let tokenizer_name = tokenizer_settings["name"].as_str().unwrap();
            debug!("tokenizer_name: {:?}", tokenizer_name);

            let mut tokenizer_args = String::new();
            if tokenizer_settings.contains_key("args") {
                tokenizer_args = serde_json::to_string(&tokenizer_settings["args"]).unwrap();
            }
            debug!("tokenizer_args: {:?}", tokenizer_args);

            // filters
            // create vector for storing filters
            //let mut filters: Vec<_> = Vec::new();
            if tokenizer_config_map.contains_key("filters") {
                let filters_config_value = tokenizer_config_map["filters"].as_array().unwrap();
                for filter_config_value in filters_config_value {
                    let filter_settings = filter_config_value.as_object().unwrap();
                    debug!("filter_settings: {:?}", filter_settings);

                    let filter_name = filter_settings["name"].as_str().unwrap();
                    debug!("filter_name: {:?}", filter_name);

                    let mut filter_args = String::new();
                    if filter_settings.contains_key("args") {
                        filter_args = serde_json::to_string(&filter_settings["args"]).unwrap();
                    }
                    debug!("filter_args: {:?}", filter_args);

                    // create filter
                    match filter_name {
                        "alpha_num_only" => {
                            let _filter = self.alpha_num_only_filter_factory.clone().create();
                            // push created filter to vector
                            //filters.push(_filter);
                        }
                        "ascii_folding" => {
                            let _filter = self.ascii_folding_filter_factory.clone().create();
                            // push created filter to vector
                            //filters.push(_filter);
                        }
                        "lower_case" => {
                            let _filter = self.lower_case_filter_factory.clone().create();
                        }
                        "remove_long" => {
                            let _filter = self
                                .remove_long_filter_factory
                                .clone()
                                .create(filter_args.as_ref());
                            // push created filter to vector
                            //filters.push(_filter);
                        }
                        "stemming" => {
                            let _filter = self
                                .stemming_filter_factory
                                .clone()
                                .create(filter_args.as_ref());
                            // push created filter to vector
                            //filters.push(_filter);
                        }
                        "stop_word" => {
                            let _filter = self
                                .stop_word_filter_factory
                                .clone()
                                .create(filter_args.as_ref());
                            // push created filter to vector
                            //filters.push(_filter);
                        }
                        _ => {
                            panic!("unknown filter: {}", filter_name);
                        }
                    }
                }
            }

            // create tokenizer
            match tokenizer_name {
                "facet" => {
                    let tokenizer = self.facet_tokenizer_factory.clone().create();
                    // add filters to tokenizer
                    //for filter in filters.iter() {
                    //    tokenizer.filter(filter);
                    //}
                    manager.register(name, tokenizer)
                }
                "ngram" => {
                    let tokenizer = self
                        .ngram_tokenizer_factory
                        .clone()
                        .create(tokenizer_args.as_ref());
                    // add filters to tokenizer
                    //for filter in filters.iter() {
                    //    tokenizer.filter(filter);
                    //}
                    manager.register(name, tokenizer)
                }
                "raw" => {
                    let tokenizer = self.raw_tokenizer_factory.clone().create();
                    // add filters to tokenizer
                    //for filter in filters.iter() {
                    //    tokenizer.filter(filter);
                    //}
                    manager.register(name, tokenizer)
                }
                "simple" => {
                    let tokenizer = self.simple_tokenizer_factory.clone().create();
                    // add filters to tokenizer
                    //for filter in filters.iter() {
                    //    tokenizer.filter(filter);
                    //}
                    manager.register(name, tokenizer)
                }
                _ => {
                    panic!("unknown tokenizer: {}", tokenizer_name);
                }
            }
        }

        debug!("tokenizers are initialized");
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use tantivy::tokenizer::{LowerCaser, SimpleTokenizer, Tokenizer, TokenizerManager};

    use crate::tokenizer::lower_case_filter_factory::LowerCaseFilterFactory;
    use crate::tokenizer::simple_tokenizer_factory::SimpleTokenizerFactory;
    use crate::tokenizer::stop_word_filter_factory::StopWordFilterFactory;
    use crate::tokenizer::tokenizer_initializer::TokenizerInitializer;

    fn read_file(path: &str) -> String {
        fs::read_to_string(path).unwrap()
    }

    #[test]
    fn test_tokenizer() {
        let config = r#"
            {
              "en_text": {
                "tokenizer": {
                  "name": "simple"
                },
                "filters": [
                  {
                    "name": "remove_long",
                    "args": {
                      "length_li mit": 50
                    }
                  },
                  {
                    "name": "lower_case"
                  },
                  {
                    "name": "stemming",
                    "args": {
                      "stemmer_algorithm": "english"
                    }
                  },
                  {
                    "name": "stop_word",
                    "args": {
                      "words": [
                        "a", "an", "and", "are", "as", "at", "be", "but", "by", "for", "if", "in", "into",
                        "is", "it", "no", "not", "of", "on", "or", "such", "that", "the", "their", "then",
                        "there", "these", "they", "this", "to", "was", "will", "with"
                      ]
                    }
                  }
                ]
              }
            }
        "#;

        let manager = TokenizerManager::default();

        let mut initializer = TokenizerInitializer::new();
        initializer.init(&manager, config);

        let tokenizer = manager.get("en_text").unwrap();
        let mut stream = tokenizer.token_stream("HELLO world!");
        {
            let token = stream.next().unwrap();
            assert_eq!(token.text, "hello");
            assert_eq!(token.offset_from, 0);
            assert_eq!(token.offset_to, 5);
        }
        {
            let token = stream.next().unwrap();
            assert_eq!(token.text, "world");
            assert_eq!(token.offset_from, 6);
            assert_eq!(token.offset_to, 11);
        }
        assert!(stream.next().is_none());
    }
}
