use super::*;

pub fn try_all<T>(
    parse_functions: Vec<fn(&mut ParseContext, &mut CompileSession) -> ParseResult<T>>,
    context: &mut ParseContext,
    session: &mut CompileSession,
) -> ParseResult<T> {
    for parse_function in parse_functions {
        let mut sandbox = context.create_sandbox();
        let result = parse_function(&mut sandbox, session);
        match &result {
            ParseResult::Fail(true) => {
                context.current += sandbox.current;
                return ParseResult::Fail(true);
            }
            ParseResult::Fail(false) => {}
            _ => {
                context.current += sandbox.current;
                return result;
            }
        }
    }

    context.next();

    ParseResult::Fail(false)
}
