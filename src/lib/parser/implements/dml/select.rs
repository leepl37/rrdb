use std::error::Error;

use crate::lib::ast::predule::{SQLStatement, SelectItem, SelectQuery};
use crate::lib::errors::predule::ParsingError;
use crate::lib::lexer::predule::Token;
use crate::lib::parser::predule::{Parser, ParserContext};

impl Parser {
    pub(crate) fn handle_select_query(
        &mut self,
        context: ParserContext,
    ) -> Result<SQLStatement, Box<dyn Error>> {
        if !self.has_next_token() {
            return Err(ParsingError::boxed("E0301: need more tokens"));
        }

        // SELECT 토큰 삼키기
        let current_token = self.get_next_token();

        if current_token != Token::Select {
            return Err(ParsingError::boxed(format!(
                "E0302: expected 'SELECT'. but your input word is '{:?}'",
                current_token
            )));
        }

        if !self.has_next_token() {
            return Err(ParsingError::boxed("E0303: need more tokens"));
        }

        let mut query_builder = SelectQuery::builder();

        // FROM 절이나 세미콜론이 나오기 전까지 select 절 파싱
        loop {
            if !self.has_next_token() {
                break;
            }

            let current_token = self.get_next_token();

            match current_token {
                Token::From => {
                    // from 다시 집어넣고 종료
                    self.unget_next_token(current_token);
                    break;
                }
                Token::SemiColon => {
                    // from 없는 select절로 간주. 종료.
                    return Ok(query_builder.build());
                }
                _ => {
                    self.unget_next_token(current_token);
                    let select_item = self.parse_select_item(context)?;
                    query_builder = query_builder.add_select_item(select_item);
                }
            }
        }

        if !self.has_next_token() {
            return Ok(query_builder.build());
        }

        // FROM 절 파싱
        let current_token = self.get_next_token();

        match current_token {
            Token::From => {
                let table_name = self.parse_table_name()?;
                query_builder = query_builder.set_from_table(table_name);

                if self.next_token_is_as() {
                    let alias = self.parse_alias()?;
                    query_builder = query_builder.set_from_alias(alias);
                }
            }
            Token::LeftParentheses => {
                self.unget_next_token(current_token);
                let subquery = self.parse_subquery(context)?;
                query_builder = query_builder.set_from_subquery(subquery);

                if self.next_token_is_as() {
                    let alias = self.parse_alias()?;
                    query_builder = query_builder.set_from_alias(alias);
                }
            }
            _ => {
                return Err(ParsingError::boxed(format!(
                    "expected 'FROM' clause. but your input word is '{:?}'",
                    current_token
                )));
            }
        }

        // TODO: JOIN 절 파싱

        // TODO: WHERE 절 파싱

        // TODO: Order By 절 파싱

        // TODO: Limit 절 파싱

        // TODO: Offset 절 파싱

        Ok(query_builder.build())
    }

    pub(crate) fn parse_select_item(
        &mut self,
        context: ParserContext,
    ) -> Result<SelectItem, Box<dyn Error>> {
        if !self.has_next_token() {
            return Err(ParsingError::boxed("need more tokens"));
        }

        let select_item = SelectItem::builder();

        // 표현식 파싱
        let select_item = select_item.set_item(self.parse_expression(context)?);

        // 더 없을 경우 바로 반환
        if !self.has_next_token() {
            return Ok(select_item.build());
        }

        let current_token = self.get_next_token();

        match current_token {
            Token::As => {
                // 더 없을 경우 바로 반환
                if !self.has_next_token() {
                    return Err(ParsingError::boxed(format!("expected alias. need more",)));
                }

                let current_token = self.get_next_token();

                match current_token {
                    Token::Identifier(identifier) => {
                        let select_item = select_item.set_alias(identifier);
                        Ok(select_item.build())
                    }
                    _ => Err(ParsingError::boxed(format!(
                        "expected alias, but your input word is '{:?}'",
                        current_token
                    ))),
                }
            }
            Token::Comma => {
                // 현재 select_item은 종료된 것으로 판단.
                Ok(select_item.build())
            }
            _ => Err(ParsingError::boxed(format!(
                "expected expression. but your input word is '{:?}'",
                current_token
            ))),
        }
    }
}
