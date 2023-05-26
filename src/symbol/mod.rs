#[cfg(test)]
mod tests;

pub struct Symbol {
    pub new_line_carriage_return: &'static str,
    pub new_line: &'static str,
    pub carriage_return: &'static str,
    pub empty_string: &'static str,
    pub whitespace: &'static str,
    pub equals: &'static str,
    pub comma: &'static str,
    pub hyphen: &'static str,
    pub slash: &'static str,
    pub semicolon: &'static str,
    pub colon: &'static str,
    pub number_sign: &'static str,
    pub opening_square_bracket: &'static str,
    pub closing_square_bracket: &'static str,
    pub opening_curly_bracket: &'static str,
    pub closing_curly_bracket: &'static str,
    pub quotation_mark: &'static str,
    pub underscore: &'static str,
    pub single_quote: &'static str,
    pub percent: &'static str,
    pub exclamation_mark: &'static str,
    pub dollar: &'static str,
    pub ampersand: &'static str,
    pub opening_bracket: &'static str,
    pub closing_bracket: &'static str,
    pub asterisk: &'static str,
    pub plus: &'static str,
    pub question_mark: &'static str,
    pub at: &'static str,
}

pub const SYMBOL: Symbol = Symbol {
    new_line: "\n",
    carriage_return: "\r",
    new_line_carriage_return: "\r\n",
    empty_string: "",
    whitespace: " ",
    equals: "=",
    comma: ",",
    hyphen: "-",
    slash: "/",
    semicolon: ";",
    colon: ":",
    number_sign: "#",
    opening_square_bracket: "[",
    closing_square_bracket: "]",
    opening_curly_bracket: "{",
    closing_curly_bracket: "}",
    quotation_mark: "\"",
    underscore: "_",
    single_quote: "'",
    percent: "%",
    exclamation_mark: "!",
    dollar: "$",
    ampersand: "&",
    opening_bracket: "(",
    closing_bracket: ")",
    asterisk: "*",
    plus: "+",
    question_mark: "?",
    at: "@",
};