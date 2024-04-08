#include <numeric>

#include "problem_2.h"

int ProblemTwoSolver::Solve() {
    std::vector<std::string> lines = this->ReadLines();

    std::vector<int> line_values {0};
    line_values.resize(lines.size());

    std::transform(
        lines.cbegin(),
        lines.cend(),
        line_values.begin(),
        [this](std::string line) {
            line = this->PreProcessLine(line);
            return this->ExtractLineValue(line);
        }
    );

    int sum_of_values = std::reduce(line_values.cbegin(), line_values.cend());
    return sum_of_values;
}

std::string ProblemTwoSolver::PreProcessLine(std::string line) const {
    line = this->ExpandAllOverlappingSpelledDigits(line);
    line = this->ConvertAllSpelledDigitsToNumbers(line);
    return line;
}

StringMap OverlappingSpelledDigitsExpansionMap() {
    return StringMap {
        {"oneight", "oneeight"},
        {"twone", "twoone"},
        {"threeight", "threeeight"},
        {"fiveight", "fiveeight"},
        {"sevenine", "sevennine"},
        {"eightwo", "eighttwo"},
        {"nineight", "nineeight"},
    };
}

std::string ProblemTwoSolver::ExpandAllOverlappingSpelledDigits(std::string line) const {
    StringMap overlapping_expansion_map = OverlappingSpelledDigitsExpansionMap();

    std::for_each(
        overlapping_expansion_map.cbegin(),
        overlapping_expansion_map.cend(),
        [this, &line](StringMapPair overlapping_expansion_pair) {
            line = this->ExpandOverlappingSpelledDigit(line, overlapping_expansion_pair);
        }
    );
    return line;
}


std::string ProblemTwoSolver::ExpandOverlappingSpelledDigit(std::string line, StringMapPair overlapping_expansion_pair) const {
    std::string overlapping_digits = overlapping_expansion_pair.first;
    std::string expanded_digits = overlapping_expansion_pair.second;

    if (this->IsSubstringInString(line, overlapping_digits)) {
        line = this->ReplaceSubstring(line, overlapping_digits, expanded_digits);

        // Recurse back into this function to convert multiple instances of the spelled digit.
        return this->ConvertSpelledDigitToNumber(line, overlapping_expansion_pair);
    } else {
        return line;
    }
}

StringMap SpelledDigitToNumericStringMap() {
    return StringMap {
        {"one", "1"},
        {"two", "2"},
        {"three", "3"},
        {"four", "4"},
        {"five", "5"},
        {"six", "6"},
        {"seven", "7"},
        {"eight", "8"},
        {"nine", "9"},
        {"zero", "0"},
    };
}

std::string ProblemTwoSolver::ConvertAllSpelledDigitsToNumbers(std::string line) const {
    StringMap spelled_to_numeric_map = SpelledDigitToNumericStringMap();

    std::for_each(
        spelled_to_numeric_map.cbegin(),
        spelled_to_numeric_map.cend(),
        [this, &line](StringMapPair spelled_to_numeric_pair) {
            line = this->ConvertSpelledDigitToNumber(line, spelled_to_numeric_pair);
        }
    );

    return line;
}


std::string ProblemTwoSolver::ConvertSpelledDigitToNumber(std::string line, StringMapPair spelled_to_numeric_pair) const {
    std::string digit_spelled = spelled_to_numeric_pair.first;
    std::string digit_numeric = spelled_to_numeric_pair.second;

    if (this->IsSubstringInString(line, digit_spelled)) {
        line = this->ReplaceSubstring(line, digit_spelled, digit_numeric);

        // Recurse back into this function to convert multiple instances of the spelled digit.
        return this->ConvertSpelledDigitToNumber(line, spelled_to_numeric_pair);
    } else {
        return line;
    }
}

std::string ProblemTwoSolver::ReplaceSubstring(
    std::string string,
    std::string substring,
    std::string replacement
) const {
    if (this->IsSubstringInString(string, substring)) {
        size_t start_pos = this->SubstringStartPos(string, substring);
        string.replace(start_pos, substring.length(), replacement);
        return string;
    } else {
        return string;
    }
}

bool ProblemTwoSolver::IsSubstringInString(std::string string, std::string substring) const {
    return this->SubstringStartPos(string, substring) != std::string::npos;
}

size_t ProblemTwoSolver::SubstringStartPos(std::string string, std::string substring) const {
    return string.find(substring);
}
