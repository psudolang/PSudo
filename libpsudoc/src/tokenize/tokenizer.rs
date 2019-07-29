use crate::coretypes::{CodeCharacter, LineColumn, Span, Token, TokenCategory};
use crate::tokenize::rules::*;
use crate::tokenize::{RuleCategory, TokenizerCommand};

pub struct Tokenizer {
    characters: Vec<CodeCharacter>,
    offset: usize,
    character_cache: Vec<CodeCharacter>,
    start_offset: LineColumn,
    rule_category: RuleCategory,
}

impl Tokenizer {
    pub fn new(source: Vec<char>) -> Tokenizer {
        Tokenizer {
            characters: source
                .into_iter()
                .map(CodeCharacter::new)
                .chain(vec![CodeCharacter::EOF].into_iter())
                .collect(),
            offset: 0,
            character_cache: Vec::new(),
            start_offset: LineColumn::FIRST_COLUMN,
            rule_category: RuleCategory::Initial,
        }
    }
    fn current_offset(&self) -> LineColumn {
        LineColumn {
            offset: self.offset,
        }
    }

    fn create_token(&self, category: TokenCategory) -> Token {
        Token {
            category,
            span: Span {
                start: self.start_offset.clone(),
                end: self.current_offset(),
            },
        }
    }
    fn prev_cursor(&mut self) {
        if self.characters.get(self.offset).is_some() {
            self.character_cache.pop();
        }
        self.offset -= 1;
    }

    fn next_cursor(&mut self) {
        if let Some(character) = self.characters.get(self.offset) {
            self.character_cache.push(character.clone());
        }
        self.offset += 1;
    }

    fn reset_state(&mut self) {
        self.start_offset = self.current_offset();
        self.rule_category = RuleCategory::Initial;
        self.character_cache.clear();
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result: Option<Self::Item> = None;
        'ret: while let Some(character) = self.characters.get(self.offset).cloned() {
            // do-while pattern
            'consume: while {
                let command = match self.rule_category {
                    RuleCategory::Initial => {
                        RuleInitial::process(&character, &self.character_cache)
                    }
                    RuleCategory::Whitespace => {
                        RuleWhitespace::process(&character, &self.character_cache)
                    }
                    RuleCategory::NewlineCr => {
                        RuleNewlineCr::process(&character, &self.character_cache)
                    }
                    RuleCategory::Identifier => {
                        RuleIdentifier::process(&character, &self.character_cache)
                    }
                    RuleCategory::QuotedString => {
                        RuleQuotedString::process(&character, &self.character_cache)
                    }
                    RuleCategory::Number => RuleNumber::process(&character, &self.character_cache),
                    RuleCategory::NumberDecimal => {
                        RuleNumberDecimal::process(&character, &self.character_cache)
                    }
                    RuleCategory::NumberExponent => {
                        RuleNumberExponent::process(&character, &self.character_cache)
                    }
                    RuleCategory::Punctuation => {
                        RulePunctuation::process(&character, &self.character_cache)
                    }
                    RuleCategory::LineComment => {
                        RuleLineComment::process(&character, &self.character_cache)
                    }
                    RuleCategory::BlockComment => {
                        RuleBlockComment::process(&character, &self.character_cache)
                    }
                };
                let to_consume = command.to_consume();
                if to_consume {
                    self.next_cursor();
                }
                match command {
                    TokenizerCommand::Continue(rule_category, _) => {
                        self.rule_category = rule_category;
                    }
                    TokenizerCommand::Emit(token_category, _) => {
                        result = Some(self.create_token(token_category));
                        self.reset_state();
                        break 'ret;
                    }
                    TokenizerCommand::Ignore(_) => {
                        break 'consume;
                    }
                    TokenizerCommand::MoveCursorPrevious => {
                        self.prev_cursor();
                        break 'consume;
                    }
                }
                !to_consume
            } {}
        }

        result
    }
}
