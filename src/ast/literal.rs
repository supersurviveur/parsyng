use crate::error::Diagnostics;
use crate::parse::Parse;

impl Parse for u8 {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        if let Some(literal) = input.literal()
            && let Ok(parsed) = literal.to_string().parse()
        {
            return Ok(parsed);
        }
        Err(Diagnostics::new_error("Expected literal"))
    }
}
