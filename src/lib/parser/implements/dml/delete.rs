use std::error::Error;

use crate::lib::parser::predule::Parser;

use crate::lib::ast::predule::DeleteQuery;
use crate::lib::errors::predule::ParsingError;

impl Parser {
    pub(crate) fn handle_delete_query(&mut self) -> Result<DeleteQuery, Box<dyn Error>> {
        if !self.has_next_token() {
            return Err(ParsingError::boxed("need more tokens"));
        }

        let _current_token = self.get_next_token();

        // TODO: impl

        todo!();
    }
}
