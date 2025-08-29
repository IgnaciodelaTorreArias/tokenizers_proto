include!(concat!(env!("OUT_DIR"), "/messages.rs"));
// Error while Converting the defined instance to a concrete instance
pub(crate) type ConversionError = (CallStatus, Option<String>);
pub(crate) mod pipeline_string {
    use super::CallStatus;
    use super::ConversionError;

    include!(concat!(env!("OUT_DIR"), "/messages.pipeline_string.rs"));

    impl Into<tokenizers::pre_tokenizer::PreTokenizedString> for PipelineStringParams {
        fn into(self) -> tokenizers::pre_tokenizer::PreTokenizedString {
            self.content.into()
        }
    }
    impl TryInto<tokenizers::normalizer::OffsetReferential> for OffsetReferential {
        type Error = ConversionError;
        
        fn try_into(self) -> Result<tokenizers::normalizer::OffsetReferential, Self::Error> {
            use tokenizers::normalizer::OffsetReferential;
            Ok(match self {
                Self::UnknownReferential => return Err((CallStatus::UnknownEnumValue, None)),
                Self::Original => OffsetReferential::Original,
                Self::Normalized => OffsetReferential::Normalized,
            })
        }
    }
    impl TryInto<tokenizers::pre_tokenizer::OffsetType> for OffsetType{
        type Error = ConversionError;
        
        fn try_into(self) -> Result<tokenizers::pre_tokenizer::OffsetType, Self::Error> {
            use tokenizers::pre_tokenizer::OffsetType;
            Ok(match self {
                Self::UnknownType => return Err((CallStatus::UnknownEnumValue, None)),
                Self::Byte => OffsetType::Byte,
                Self::Char => OffsetType::Char,
                Self::None => OffsetType::None,
            })
        }
    }
}
pub(crate) mod normalizers {
    use super::CallStatus;
    use super::ConversionError;

    include!(concat!(env!("OUT_DIR"), "/messages.normalizers.rs"));

    impl Into<tokenizers::normalizers::NormalizerWrapper> for BertNormalizer {
        fn into(self) -> tokenizers::normalizers::NormalizerWrapper {
            use tokenizers::normalizers::bert::BertNormalizer;
            let mut d = BertNormalizer::default();
            d.clean_text = self.clean_text.unwrap_or(d.clean_text);
            d.handle_chinese_chars = self.handle_chinese_chars.unwrap_or(d.handle_chinese_chars);
            d.strip_accents = self.strip_accents;
            d.lowercase = self.lowercase.unwrap_or(d.lowercase);
            d.into()
        }
    }
    impl Into<tokenizers::normalizers::NormalizerWrapper> for Nfd {
        fn into(self) -> tokenizers::normalizers::NormalizerWrapper {
            tokenizers::normalizers::unicode::NFD.into()
        }
    }
    impl Into<tokenizers::normalizers::NormalizerWrapper> for Nfkd {
        fn into(self) -> tokenizers::normalizers::NormalizerWrapper {
            tokenizers::normalizers::unicode::NFKD.into()
        }
    }
    impl Into<tokenizers::normalizers::NormalizerWrapper> for Nfc {
        fn into(self) -> tokenizers::normalizers::NormalizerWrapper {
            tokenizers::normalizers::unicode::NFC.into()
        }
    }
    impl Into<tokenizers::normalizers::NormalizerWrapper> for Nfkc {
        fn into(self) -> tokenizers::normalizers::NormalizerWrapper {
            tokenizers::normalizers::unicode::NFKC.into()
        }
    }
    impl Into<tokenizers::normalizers::NormalizerWrapper> for Nmt {
        fn into(self) -> tokenizers::normalizers::NormalizerWrapper {
            tokenizers::normalizers::unicode::Nmt.into()
        }
    }
    impl Into<tokenizers::normalizers::NormalizerWrapper> for StripNormalizer {
        fn into(self) -> tokenizers::normalizers::NormalizerWrapper {
            tokenizers::normalizers::strip::Strip::new(self.strip_left, self.strip_right).into()
        }
    }
    impl Into<tokenizers::normalizers::NormalizerWrapper> for StripAccents {
        fn into(self) -> tokenizers::normalizers::NormalizerWrapper {
            tokenizers::normalizers::strip::StripAccents.into()
        }
    }
    impl Into<tokenizers::normalizers::NormalizerWrapper> for Lowercase {
        fn into(self) -> tokenizers::normalizers::NormalizerWrapper {
            tokenizers::normalizers::utils::Lowercase.into()
        }
    }
    impl Into<tokenizers::normalizers::NormalizerWrapper> for Prepend {
        fn into(self) -> tokenizers::normalizers::NormalizerWrapper {
            tokenizers::normalizers::prepend::Prepend::new(self.prepend).into()
        }
    }
    impl TryInto<tokenizers::normalizers::replace::Replace> for Replace {
        type Error = ConversionError;

        fn try_into(self) -> Result<tokenizers::normalizers::replace::Replace, Self::Error> {
            use tokenizers::normalizers::replace::{Replace, ReplacePattern};
            let pattern = match self.pattern.unwrap() {
                replace::Pattern::StringReplacement(s) => ReplacePattern::String(s),
                replace::Pattern::RegexReplacement(s) => ReplacePattern::Regex(s),
            };
            Ok(match Replace::new(pattern, self.content) {
                Ok(res) => res,
                Err(e) => {
                    return Err((
                        CallStatus::InvalidArgumentsDetails.into(),
                        Some(e.to_string()),
                    ));
                }
            })
        }
    }
    impl TryInto<tokenizers::normalizers::NormalizerWrapper> for Replace {
        type Error = ConversionError;

        fn try_into(self) -> Result<tokenizers::normalizers::NormalizerWrapper, Self::Error> {
            use tokenizers::normalizers::replace::Replace;
            let v: Replace = self.try_into()?;
            Ok(v.into())
        }
    }
    impl TryInto<tokenizers::decoders::DecoderWrapper> for Replace {
        type Error = ConversionError;

        fn try_into(self) -> Result<tokenizers::decoders::DecoderWrapper, Self::Error> {
            use tokenizers::normalizers::replace::Replace;
            let v: Replace = self.try_into()?;
            Ok(v.into())
        }
    }
    impl TryInto<tokenizers::normalizers::NormalizerWrapper> for Precompiled {
        type Error = ConversionError;

        fn try_into(self) -> Result<tokenizers::normalizers::NormalizerWrapper, Self::Error> {
            Ok(
                match tokenizers::normalizers::precompiled::Precompiled::from(
                    &self.precompiled_charsmap,
                ) {
                    Ok(res) => res,
                    Err(e) => {
                        return Err((CallStatus::InvalidArgumentsDetails, Some(e.to_string())));
                    }
                }
                .into(),
            )
        }
    }
    impl Into<tokenizers::normalizers::NormalizerWrapper> for ByteLevel {
        fn into(self) -> tokenizers::normalizers::NormalizerWrapper {
            tokenizers::normalizers::byte_level::ByteLevel.into()
        }
    }
}
pub(crate) mod pre_tokenizers {
    use super::CallStatus;
    use super::ConversionError;

    include!(concat!(env!("OUT_DIR"), "/messages.pre_tokenizers.rs"));

    impl Into<tokenizers::pre_tokenizers::PreTokenizerWrapper> for BertPreTokenizer {
        fn into(self) -> tokenizers::pre_tokenizers::PreTokenizerWrapper {
            tokenizers::pre_tokenizers::bert::BertPreTokenizer.into()
        }
    }
    impl Into<tokenizers::pre_tokenizers::byte_level::ByteLevel> for ByteLevel {
        fn into(self) -> tokenizers::pre_tokenizers::byte_level::ByteLevel {
            use tokenizers::pre_tokenizers::byte_level::ByteLevel;
            let mut d = ByteLevel::default();
            d.add_prefix_space = self.add_prefix_space.unwrap_or(d.add_prefix_space);
            d.trim_offsets = self.trim_offsets.unwrap_or(d.trim_offsets);
            d.use_regex = self.use_regex.unwrap_or(d.use_regex);
            d
        }
    }
    impl Into<tokenizers::pre_tokenizers::PreTokenizerWrapper> for ByteLevel {
        fn into(self) -> tokenizers::pre_tokenizers::PreTokenizerWrapper {
            use tokenizers::pre_tokenizers::byte_level::ByteLevel;
            let v: ByteLevel = self.into();
            v.into()
        }
    }
    impl Into<tokenizers::processors::PostProcessorWrapper> for ByteLevel {
        fn into(self) -> tokenizers::processors::PostProcessorWrapper {
            use tokenizers::pre_tokenizers::byte_level::ByteLevel;
            let v: ByteLevel = self.into();
            v.into()
        }
    }
    impl Into<tokenizers::decoders::DecoderWrapper> for ByteLevel {
        fn into(self) -> tokenizers::decoders::DecoderWrapper {
            use tokenizers::pre_tokenizers::byte_level::ByteLevel;
            let v: ByteLevel = self.into();
            v.into()
        }
    }
    impl TryInto<tokenizers::pre_tokenizers::metaspace::PrependScheme> for PrependScheme {
        type Error = ConversionError;

        fn try_into(
            self,
        ) -> Result<tokenizers::pre_tokenizers::metaspace::PrependScheme, Self::Error> {
            use tokenizers::pre_tokenizers::metaspace::PrependScheme;
            Ok(match self {
                Self::UnknownScheme => return Err((CallStatus::UnknownEnumValue, None)),
                Self::First => PrependScheme::First,
                Self::Never => PrependScheme::Never,
                Self::Always => PrependScheme::Always,
            })
        }
    }
    impl TryInto<tokenizers::pre_tokenizers::metaspace::Metaspace> for Metaspace {
        type Error = ConversionError;

        fn try_into(self) -> Result<tokenizers::pre_tokenizers::metaspace::Metaspace, Self::Error> {
            use tokenizers::pre_tokenizers::metaspace::Metaspace;
            let mut d = Metaspace::default();
            if self.prepend_scheme.is_some() {
                d.prepend_scheme = self.prepend_scheme().try_into()?;
            }
            if let Some(replacement) = self.replacement_char {
                let replacement = match replacement.chars().next() {
                    Some(c) => c,
                    None => {
                        return Err((
                            CallStatus::InvalidArgumentsDetails,
                            Some("Replacement_char cannot be empty".to_string()),
                        ));
                    }
                };
                d.set_replacement(replacement);
            }
            d.split = self.split.unwrap_or(d.split);
            Ok(d)
        }
    }
    impl TryInto<tokenizers::pre_tokenizers::PreTokenizerWrapper> for Metaspace {
        type Error = ConversionError;

        fn try_into(self) -> Result<tokenizers::pre_tokenizers::PreTokenizerWrapper, Self::Error> {
            use tokenizers::pre_tokenizers::metaspace::Metaspace;
            let v: Metaspace = self.try_into()?;
            Ok(v.into())
        }
    }
    impl TryInto<tokenizers::decoders::DecoderWrapper> for Metaspace {
        type Error = ConversionError;

        fn try_into(self) -> Result<tokenizers::decoders::DecoderWrapper, Self::Error> {
            use tokenizers::pre_tokenizers::metaspace::Metaspace;
            let v: Metaspace = self.try_into()?;
            Ok(v.into())
        }
    }
    impl Into<tokenizers::pre_tokenizers::PreTokenizerWrapper> for Whitespace {
        fn into(self) -> tokenizers::pre_tokenizers::PreTokenizerWrapper {
            tokenizers::pre_tokenizers::whitespace::Whitespace.into()
        }
    }
    impl Into<tokenizers::pre_tokenizers::PreTokenizerWrapper> for WhitespaceSplit {
        fn into(self) -> tokenizers::pre_tokenizers::PreTokenizerWrapper {
            tokenizers::pre_tokenizers::whitespace::WhitespaceSplit.into()
        }
    }
    impl TryInto<tokenizers::pre_tokenizers::PreTokenizerWrapper> for Delimiter {
        type Error = ConversionError;

        fn try_into(self) -> Result<tokenizers::pre_tokenizers::PreTokenizerWrapper, Self::Error> {
            use tokenizers::pre_tokenizers::delimiter::CharDelimiterSplit;
            let delimiter = match self.char.chars().next() {
                Some(c) => c,
                None => {
                    return Err((
                        CallStatus::InvalidArgumentsDetails,
                        Some("Char cannot be empty".to_string()),
                    ));
                }
            };
            Ok(CharDelimiterSplit::new(delimiter).into())
        }
    }
    impl TryInto<tokenizers::tokenizer::normalizer::SplitDelimiterBehavior> for SplitDelimiterBehavior {
        type Error = ConversionError;

        fn try_into(
            self,
        ) -> Result<tokenizers::tokenizer::normalizer::SplitDelimiterBehavior, Self::Error>
        {
            use tokenizers::tokenizer::normalizer::SplitDelimiterBehavior;
            Ok(match self {
                Self::UnknownBehavior => return Err((CallStatus::UnknownEnumValue, None)),
                Self::Removed => SplitDelimiterBehavior::Removed,
                Self::Isolated => SplitDelimiterBehavior::Isolated,
                Self::MergedWithPrevious => SplitDelimiterBehavior::MergedWithPrevious,
                Self::MergedWithNext => SplitDelimiterBehavior::MergedWithNext,
                Self::Contiguous => SplitDelimiterBehavior::Contiguous,
            })
        }
    }
    impl TryInto<tokenizers::pre_tokenizers::PreTokenizerWrapper> for Split {
        type Error = ConversionError;

        fn try_into(self) -> Result<tokenizers::pre_tokenizers::PreTokenizerWrapper, Self::Error> {
            use tokenizers::normalizer::SplitDelimiterBehavior;
            use tokenizers::pre_tokenizers::split::{Split, SplitPattern};

            let behavior: SplitDelimiterBehavior = self.behavior().try_into()?;
            let pattern = match self.pattern {
                Some(pattern) => match pattern {
                    split::Pattern::StringSplit(s) => SplitPattern::String(s),
                    split::Pattern::RegexSplit(s) => SplitPattern::Regex(s),
                },
                None => return Err((CallStatus::InvalidArgumentsDetails, Some("Pattern cannot be empty".to_string()))),
            };
            Ok(
                match Split::new(pattern, behavior, self.invert) {
                    Ok(res) => res,
                    Err(e) => {
                        return Err((CallStatus::InvalidArgumentsDetails, Some(e.to_string())));
                    }
                }
                .into(),
            )
        }
    }
    impl TryInto<tokenizers::pre_tokenizers::PreTokenizerWrapper> for Punctuation {
        type Error = ConversionError;

        fn try_into(self) -> Result<tokenizers::pre_tokenizers::PreTokenizerWrapper, Self::Error> {
            use tokenizers::pre_tokenizers::punctuation::Punctuation;
            let mut d = Punctuation::default();
            if self.behavior.is_some() {
                d.behavior = self.behavior().try_into()?;
            }
            Ok(d.into())
        }
    }
    impl Into<tokenizers::pre_tokenizers::PreTokenizerWrapper> for Digits {
        fn into(self) -> tokenizers::pre_tokenizers::PreTokenizerWrapper {
            use tokenizers::pre_tokenizers::digits::Digits;
            let mut d = Digits::default();
            d.individual_digits = self.individual_digits.unwrap_or(d.individual_digits);
            d.into()
        }
    }
    impl Into<tokenizers::pre_tokenizers::PreTokenizerWrapper> for UnicodeScripts {
        fn into(self) -> tokenizers::pre_tokenizers::PreTokenizerWrapper {
            tokenizers::pre_tokenizers::unicode_scripts::UnicodeScripts.into()
        }
    }
    impl Into<tokenizers::pre_tokenizers::PreTokenizerWrapper> for FixedLength {
        fn into(self) -> tokenizers::pre_tokenizers::PreTokenizerWrapper {
            if let Some(length) = self.length {
                tokenizers::pre_tokenizers::fixed_length::FixedLength::new(length as usize)
            } else {
                tokenizers::pre_tokenizers::fixed_length::FixedLength::new(5)
            }
            .into()
        }
    }
}
pub(crate) mod processors {
    use super::CallStatus;
    use super::ConversionError;

    include!(concat!(env!("OUT_DIR"), "/messages.processors.rs"));

    impl Into<tokenizers::processors::PostProcessorWrapper> for RobertaProcessing {
        fn into(self) -> tokenizers::processors::PostProcessorWrapper {
            use tokenizers::processors::roberta::RobertaProcessing;
            let mut d = RobertaProcessing::default();
            d.sep = (
                self.sep_str.unwrap_or(d.sep.0),
                self.sep_id.unwrap_or(d.sep.1),
            );
            d.cls = (
                self.cls_str.unwrap_or(d.cls.0),
                self.cls_id.unwrap_or(d.cls.1),
            );
            d.trim_offsets = self.trim_offsets.unwrap_or(d.trim_offsets);
            d.add_prefix_space = self.add_prefix_space.unwrap_or(d.add_prefix_space);
            d.into()
        }
    }
    impl Into<tokenizers::processors::PostProcessorWrapper> for BertProcessing {
        fn into(self) -> tokenizers::processors::PostProcessorWrapper {
            use tokenizers::processors::bert::BertProcessing;
            let mut d = BertProcessing::default();
            d.sep = (
                self.sep_str.unwrap_or(d.sep.0),
                self.sep_id.unwrap_or(d.sep.1),
            );
            d.cls = (
                self.cls_str.unwrap_or(d.cls.0),
                self.cls_id.unwrap_or(d.cls.1),
            );
            d.into()
        }
    }
    impl TryInto<tokenizers::processors::template::SpecialToken> for SpecialToken {
        type Error = ConversionError;

        fn try_into(self) -> Result<tokenizers::processors::template::SpecialToken, Self::Error> {
            use tokenizers::processors::template::SpecialToken;
            Ok(match SpecialToken::new(self.token, self.ids, self.tokens) {
                Ok(res) => res,
                Err(e) => return Err((CallStatus::InvalidArgumentsDetails, Some(e.to_string()))),
            })
        }
    }
    impl Into<tokenizers::processors::template::SpecialToken> for TokenPair {
        fn into(self) -> tokenizers::processors::template::SpecialToken {
            (self.token, self.token_id).into()
        }
    }
    impl TryInto<tokenizers::processors::template::SpecialToken> for Token {
        type Error = ConversionError;

        fn try_into(self) -> Result<tokenizers::processors::template::SpecialToken, Self::Error> {
            Ok(match self.token.unwrap() {
                token::Token::SpecialToken(special_token) => special_token.try_into()?,
                token::Token::TokenPair(token_pair) => token_pair.into(),
            })
        }
    }
    impl TryInto<tokenizers::processors::template::Tokens> for TokensMap {
        type Error = ConversionError;

        fn try_into(self) -> Result<tokenizers::processors::template::Tokens, Self::Error> {
            use tokenizers::processors::template::{SpecialToken, Tokens};
            let map = self
                .tokens
                .into_iter()
                .map(|(k, v)| -> Result<(String, SpecialToken), Self::Error> {
                    Ok((k, v.try_into()?))
                })
                .collect::<Result<_, _>>()?;
            Ok(Tokens(map))
        }
    }
    impl TryInto<tokenizers::processors::template::Tokens> for Tokens {
        type Error = ConversionError;

        fn try_into(self) -> Result<tokenizers::processors::template::Tokens, Self::Error> {
            use tokenizers::processors::template::SpecialToken;
            let vec: Vec<SpecialToken> = self
                .tokens
                .into_iter()
                .map(|t| -> Result<SpecialToken, Self::Error> { Ok(t.try_into()?) })
                .collect::<Result<_, _>>()?;
            Ok(vec.into())
        }
    }
    impl TryInto<tokenizers::processors::PostProcessorWrapper> for TemplateProcessing {
        type Error = ConversionError;
        fn try_into(self) -> Result<tokenizers::processors::PostProcessorWrapper, Self::Error> {
            use tokenizers::processors::template::{TemplateProcessing, Tokens};
            let mut b = TemplateProcessing::builder();
            if let Some(value) = self.single {
                if let Err(e) = b.try_single(value) {
                    return Err((CallStatus::InvalidArgumentsDetails, Some(e)));
                }
            }
            if let Some(value) = self.pair {
                if let Err(e) = b.try_pair(value) {
                    return Err((CallStatus::InvalidArgumentsDetails, Some(e)));
                }
            }
            let t: Tokens = match self.special_tokens.unwrap() {
                template_processing::SpecialTokens::TokensMap(tokens_map) => {
                    tokens_map.try_into()?
                }
                template_processing::SpecialTokens::Tokens(tokens) => tokens.try_into()?,
            };
            b.special_tokens(t);
            Ok(match b.build() {
                Ok(res) => res,
                Err(e) => return Err((CallStatus::InvalidArgumentsDetails, Some(e.to_string()))),
            }
            .into())
        }
    }
}
pub(crate) mod decoders {
    use super::CallStatus;
    use super::ConversionError;

    include!(concat!(env!("OUT_DIR"), "/messages.decoders.rs"));

    impl Into<tokenizers::decoders::DecoderWrapper> for BpeDecoder {
        fn into(self) -> tokenizers::decoders::DecoderWrapper {
            use tokenizers::decoders::bpe::BPEDecoder;
            if let Some(suffix) = self.suffix {
                BPEDecoder::new(suffix)
            } else {
                BPEDecoder::default()
            }
            .into()
        }
    }
    impl Into<tokenizers::decoders::DecoderWrapper> for WordPiece {
        fn into(self) -> tokenizers::decoders::DecoderWrapper {
            use tokenizers::decoders::wordpiece::WordPiece;
            let mut d = WordPiece::default();
            d.prefix = self.prefix.unwrap_or(d.prefix);
            d.cleanup = self.cleanup.unwrap_or(d.cleanup);
            d.into()
        }
    }
    impl Into<tokenizers::decoders::DecoderWrapper> for Ctc {
        fn into(self) -> tokenizers::decoders::DecoderWrapper {
            use tokenizers::decoders::ctc::CTC;
            let mut d = CTC::default();
            d.pad_token = self.pad_token.unwrap_or(d.pad_token);
            d.word_delimiter_token = self.word_delimiter_token.unwrap_or(d.word_delimiter_token);
            d.cleanup = self.cleanup.unwrap_or(d.cleanup);
            d.into()
        }
    }
    impl Into<tokenizers::decoders::DecoderWrapper> for Fuse {
        fn into(self) -> tokenizers::decoders::DecoderWrapper {
            tokenizers::decoders::fuse::Fuse::new().into()
        }
    }
    impl TryInto<tokenizers::decoders::DecoderWrapper> for Strip {
        type Error = ConversionError;

        fn try_into(self) -> Result<tokenizers::decoders::DecoderWrapper, Self::Error> {
            let content = match self.content.chars().next() {
                Some(c) => c,
                None => {
                    return Err((
                        CallStatus::InvalidArgumentsDetails,
                        Some("Content cannot be empty".to_string()),
                    ));
                }
            };
            let start = self.start as usize;
            let stop = self.stop as usize;
            Ok(tokenizers::decoders::strip::Strip::new(content, start, stop).into())
        }
    }
    impl Into<tokenizers::decoders::DecoderWrapper> for ByteFallback {
        fn into(self) -> tokenizers::decoders::DecoderWrapper {
            tokenizers::decoders::byte_fallback::ByteFallback::new().into()
        }
    }
}
pub(crate) mod trainers {
    use super::CallStatus;
    use super::ConversionError;

    include!(concat!(env!("OUT_DIR"), "/messages.trainers.rs"));

    fn get_added_tokens(special_tokens: Vec<AddedToken>) -> Vec<tokenizers::AddedToken> {
        special_tokens
            .into_iter()
            .map(|at| tokenizers::AddedToken {
                content: at.content,
                single_word: at.single_word,
                lstrip: at.lstrip,
                rstrip: at.rstrip,
                normalized: at.normalized,
                special: at.special,
            })
            .collect()
    }

    impl Into<tokenizers::models::TrainerWrapper> for BpeTrainer {
        fn into(self) -> tokenizers::models::TrainerWrapper {
            use tokenizers::models::bpe::BpeTrainer;
            let mut d = BpeTrainer::default();
            d.min_frequency = self.min_frequency.unwrap_or(d.min_frequency);
            if let Some(vocab_size) = self.vocab_size {
                d.vocab_size = vocab_size as usize;
            };
            d.show_progress = self.show_progress.unwrap_or(d.show_progress);
            d.special_tokens = get_added_tokens(self.special_tokens);
            if let Some(limit_alphabet) = self.limit_alphabet {
                d.limit_alphabet = Some(limit_alphabet as usize);
            };
            if let Some(initial_alphabet) = self.initial_alphabet {
                d.initial_alphabet = initial_alphabet.chars().into_iter().collect();
            };
            d.continuing_subword_prefix = self.continuing_subword_prefix;
            d.end_of_word_suffix = self.end_of_word_suffix;
            if let Some(max_token_length) = self.max_token_length {
                d.max_token_length = Some(max_token_length as usize);
            };
            d.into()
        }
    }
    impl Into<tokenizers::models::TrainerWrapper> for WordPieceTrainer {
        fn into(self) -> tokenizers::models::TrainerWrapper {
            use tokenizers::models::wordpiece::WordPieceTrainer;
            let mut d = WordPieceTrainer::default();
            d.set_min_frequency(self.min_frequency.unwrap_or(d.min_frequency()));
            if let Some(vocab_size) = self.vocab_size {
                d.set_vocab_size(vocab_size as usize);
            };
            d.set_show_progress(self.show_progress.unwrap_or(d.show_progress()));
            d.set_special_tokens(get_added_tokens(self.special_tokens));
            if let Some(limit_alphabet) = self.limit_alphabet {
                d.set_limit_alphabet(Some(limit_alphabet as usize));
            };
            if let Some(initial_alphabet) = self.initial_alphabet {
                d.set_initial_alphabet(initial_alphabet.chars().into_iter().collect());
            };
            if let Some(continuing_subword_prefix) = self.continuing_subword_prefix {
                d.set_continuing_subword_prefix(Some(continuing_subword_prefix));
            }
            if let Some(end_of_word_suffix) = self.end_of_word_suffix {
                d.set_end_of_word_suffix(Some(end_of_word_suffix));
            }
            d.into()
        }
    }
    impl Into<tokenizers::models::TrainerWrapper> for WordLevelTrainer {
        fn into(self) -> tokenizers::models::TrainerWrapper {
            use tokenizers::models::wordlevel::WordLevelTrainer;
            let mut d = WordLevelTrainer::default();
            d.min_frequency = self.min_frequency.unwrap_or(d.min_frequency);
            if let Some(vocab_size) = self.vocab_size {
                d.vocab_size = vocab_size as usize;
            }
            d.show_progress = self.show_progress.unwrap_or(d.show_progress);
            d.special_tokens = get_added_tokens(self.special_tokens);
            d.into()
        }
    }
    impl Into<tokenizers::models::TrainerWrapper> for UnigramTrainer {
        fn into(self) -> tokenizers::models::TrainerWrapper {
            use tokenizers::models::unigram::UnigramTrainer;
            let mut d = UnigramTrainer::default();
            d.show_progress = self.show_progress.unwrap_or(d.show_progress);
            d.vocab_size = self.vocab_size.unwrap_or(d.vocab_size);
            d.n_sub_iterations = self.n_sub_iterations.unwrap_or(d.n_sub_iterations);
            d.shrinking_factor = self.shrinking_factor.unwrap_or(d.shrinking_factor);
            d.special_tokens = get_added_tokens(self.special_tokens);
            if let Some(initial_alphabet) = self.initial_alphabet {
                d.initial_alphabet = initial_alphabet.chars().into_iter().collect();
            }
            d.unk_token = self.unk_token;
            if let Some(max_piece_length) = self.max_piece_length {
                d.max_piece_length = max_piece_length as usize;
            }
            d.into()
        }
    }

    impl TryInto<tokenizers::utils::truncation::TruncationDirection> for TruncationDirection {
        type Error = ConversionError;

        fn try_into(
            self,
        ) -> Result<tokenizers::utils::truncation::TruncationDirection, Self::Error> {
            use tokenizers::utils::truncation::TruncationDirection;
            Ok(match self {
                Self::UnknownTruncationDirection => {
                    return Err((CallStatus::UnknownEnumValue, None));
                }
                Self::LeftTruncation => TruncationDirection::Left,
                Self::RightTruncation => TruncationDirection::Right,
            })
        }
    }
    impl TryInto<tokenizers::utils::truncation::TruncationStrategy> for TruncationStrategy {
        type Error = ConversionError;

        fn try_into(
            self,
        ) -> Result<tokenizers::utils::truncation::TruncationStrategy, Self::Error> {
            use tokenizers::utils::truncation::TruncationStrategy;
            Ok(match self {
                Self::UnknownTruncationStrategy => {
                    return Err((CallStatus::UnknownEnumValue, None));
                }
                Self::LongestFirst => TruncationStrategy::LongestFirst,
                Self::OnlyFirst => TruncationStrategy::OnlyFirst,
                Self::OnlySecond => TruncationStrategy::OnlySecond,
            })
        }
    }
    impl TryInto<tokenizers::utils::truncation::TruncationParams> for TruncationParams {
        type Error = ConversionError;

        fn try_into(self) -> Result<tokenizers::utils::truncation::TruncationParams, Self::Error> {
            use tokenizers::utils::truncation::TruncationParams;
            let mut d = TruncationParams::default();
            if self.direction.is_some() {
                d.direction = self.direction().try_into()?;
            }
            if let Some(max_length) = self.max_length {
                d.max_length = max_length as usize;
            }
            if self.strategy.is_some() {
                d.strategy = self.strategy().try_into()?;
            }
            if let Some(stride) = self.stride {
                d.stride = stride as usize;
            }
            Ok(d)
        }
    }
    impl TryInto<tokenizers::utils::padding::PaddingDirection> for PaddingDirection {
        type Error = ConversionError;

        fn try_into(self) -> Result<tokenizers::utils::padding::PaddingDirection, Self::Error> {
            use tokenizers::utils::padding::PaddingDirection;
            Ok(match self {
                Self::UnknownPaddingDirection => return Err((CallStatus::UnknownEnumValue, None)),
                Self::LeftPadding => PaddingDirection::Left,
                Self::RightPadding => PaddingDirection::Right,
            })
        }
    }
    impl TryInto<tokenizers::utils::padding::PaddingParams> for PaddingParams {
        type Error = ConversionError;

        fn try_into(self) -> Result<tokenizers::utils::padding::PaddingParams, Self::Error> {
            use super::trainers::PaddingStrategy;
            use tokenizers::utils::padding::{PaddingParams, PaddingStrategy as PS};
            let mut d = PaddingParams::default();

            if self.strategy.is_some() {
                d.strategy = match self.strategy() {
                    PaddingStrategy::UnknownPaddingStrategy => return Err((CallStatus::UnknownEnumValue, None)),
                    PaddingStrategy::BatchLongest => PS::BatchLongest,
                    PaddingStrategy::Fixed => {
                        if let Some(fixed_len) = self.fixed_len {
                            PS::Fixed(fixed_len as usize)
                        } else {
                            return Err((CallStatus::InvalidArgumentsDetails, Some("If the padding strategy is FIXED you must also set the field `fixed_len`".to_string())));
                        }
                    }
                }
            }
            if self.direction.is_some() {
                d.direction = self.direction().try_into()?;
            }
            if let Some(pad_to_multiple_of) = self.pad_to_multiple_of {
                d.pad_to_multiple_of = Some(pad_to_multiple_of as usize);
            }
            d.pad_id = self.pad_id.unwrap_or(d.pad_id);
            d.pad_type_id = self.pad_type_id.unwrap_or(d.pad_type_id);
            d.pad_token = self.pad_token.unwrap_or(d.pad_token);
            Ok(d)
        }
    }
}
pub(crate) mod tokenizer {
    include!(concat!(env!("OUT_DIR"), "/messages.tokenizer.rs"));
    pub fn encoding_to_message(
        original: &tokenizers::tokenizer::Encoding,
        include_type_ids: bool,
        include_tokens: bool,
        include_words: bool,
        include_offsets: bool,
        include_special_tokens_mask: bool,
        include_attention_mask: bool,
    ) -> Encoding {
        Encoding {
            ids: original.get_ids().to_vec(),
            type_ids: if include_type_ids {
                original.get_type_ids().to_vec()
            } else {
                vec![]
            },
            tokens: if include_tokens {
                original.get_tokens().to_vec()
            } else {
                vec![]
            },
            words: if include_words {
                original
                    .get_word_ids()
                    .into_iter()
                    .filter_map(|&x| x)
                    .collect()
            } else {
                vec![]
            },
            offsets: if include_offsets {
                original
                    .get_offsets()
                    .into_iter()
                    .map(|&(start, end)| Offsets {
                        start: start as u64,
                        end: end as u64,
                    })
                    .collect()
            } else {
                vec![]
            },
            special_tokens_mask: if include_special_tokens_mask {
                original.get_special_tokens_mask().to_vec()
            } else {
                vec![]
            },
            attention_mask: if include_attention_mask {
                original.get_attention_mask().to_vec()
            } else {
                vec![]
            },
        }
    }
}
