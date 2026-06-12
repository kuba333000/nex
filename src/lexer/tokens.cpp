#include <string>

enum class TokenType {
    Identifier,
    Integer,
    Float,
    Plus,
    Minus,
    Assign,
    Semicolon,
    EndOfFile,
    Invalid
};

struct Token {
    TokenType type;
    std::string value;
};