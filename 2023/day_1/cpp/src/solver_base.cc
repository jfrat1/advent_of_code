#include "solver_base.h"

SolverBase::SolverBase(const IFileReader* file_reader) {
    this->file_reader = file_reader;
}

std::vector<std::string> SolverBase::ReadLines() const {
    return this->file_reader->ReadLines();
}

int SolverBase::ExtractLineValue(std::string line) const {
    LineDigits<char> digits = this->LineDigitChars(line);

    std::string value_str {digits.first, digits.last};
    int value = std::stoi(value_str);
    return value;
}

LineDigits<char> SolverBase::LineDigitChars(std::string line) const {
    LineDigits<size_t> digit_positions = this->LineDigitPositions(line);

    char first_digit = line[digit_positions.first];
    char last_digit = line[digit_positions.last];

    return LineDigits<char> {first_digit, last_digit};
}

LineDigits<size_t> SolverBase::LineDigitPositions(std::string line) const {
    const char* digits = "1234567890";
    size_t first_digit_pos = line.find_first_of(digits);
    size_t last_digit_pos = line.find_last_of(digits);
    return LineDigits<size_t> {first_digit_pos, last_digit_pos};
}
