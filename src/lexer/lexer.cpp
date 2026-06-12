#include <string>

class Lexer {
private:
    std::string source;
    size_t pos = 0;

public:
    Lexer(std::string input) : source(std::move(input)) {}

    char peek() const {
        if (pos >= source.size()) return '\0';
        return source[pos];
    }

    char advance() {
        if (pos >= source.size()) return '\0';
        return source[pos++];
    }

    bool eof() const {
        return pos >= source.size();
    }
};