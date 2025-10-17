use thiserror::Error;
use chumsky::{error::Error as ChumskyError, input::ValueInput, label::LabelError, span::SimpleSpan, DefaultExpected};
use crate::lexer::token::Token;

#[derive(Clone, PartialEq, Debug, Error)]
pub enum ParseError {
    #[error("[line {line}] Expected token `{expected:?}`, found `{found:?}`")]
    ExpectedFound {
        expected: Vec<DefaultExpected<'static, Token>>,
        found: Option<String>,
        line: usize,
    },
    #[error("[line {line}] Unknown parse error")]
    Other {
        line: usize
    },
}

impl<'a, I> LabelError<'a, I, DefaultExpected<'a, Token>> for ParseError
where
    I: ValueInput<'a, Token = Token, Span = SimpleSpan>
{
    fn expected_found<E: IntoIterator<Item = DefaultExpected<'a, Token>>>(
            expected: E,
            found: Option<chumsky::util::MaybeRef<'a, <I as chumsky::prelude::Input<'a>>::Token>>,
            span: <I as chumsky::prelude::Input<'a>>::Span,
        ) -> Self {

        let line = span.start;
        let found_str = found.map(|t| format!("{:?}", t));
        ParseError::ExpectedFound {
            expected: expected
                .into_iter()
                .map(|e| e.into_owned())
                .collect(),
            found: found_str,
            line,
        }
    }
}

impl<'a, I> ChumskyError<'a, I> for ParseError
where
    I: ValueInput<'a, Token = Token, Span = SimpleSpan>
{
    fn merge(mut self, mut other: Self) -> Self {
        if let (Self::ExpectedFound { expected, .. }, Self::ExpectedFound { expected: expected_other, .. }) = (
            &mut self,
            &mut other,
        ) {
            expected.append(expected_other);
        }
        self
    }
}
