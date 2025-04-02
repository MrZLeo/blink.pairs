use crate::define_token_enum;

define_token_enum!(LatexToken, {
    delimiters: {
        "(" => ")",
        "[" => "]",
        "{" => "}",
        symmetric "$$" priority = 1,
        symmetric "$" priority = 2,
    },
    line_comment: ["%"],
    block_comment: [],
    string_regex: ["(?&dstring)", "(?&schar)"],
    block_string: []
});
