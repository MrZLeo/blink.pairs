use crate::define_token_enum;

define_token_enum!(LatexToken, {
    delimiters: {
        "(" => ")",
        "[" => "]",
        "{" => "}",
        symmetric "$$",
        symmetric "$",
    },
    line_comment: ["%"],
    block_comment: [],
    string_regex: ["(?&dstring)", "(?&schar)"],
    block_string: []
});
