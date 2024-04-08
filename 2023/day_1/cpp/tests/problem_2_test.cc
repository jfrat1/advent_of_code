#include <gtest/gtest.h>
#include <gmock/gmock.h>

#include <vector>

#include "problem_2.h"


class MockFileReader : public IFileReader {
    public:
        MOCK_METHOD(FileLines, ReadLines, (), (const override));
};


class ProblemTwoTest : ProblemTwoSolver {
    public:
        ProblemTwoTest(const IFileReader* file_reader) : ProblemTwoSolver(file_reader) {};
        std::string PreProcessLine(std::string line) const {
            return ProblemTwoSolver::PreProcessLine(line);
        }
        std::string ExpandAllOverlappingSpelledDigits(std::string line) const {
            return ProblemTwoSolver::ExpandAllOverlappingSpelledDigits(line);
        }
        std::string ExpandOverlappingSpelledDigit(std::string line, StringMapPair overlapping_expansion_pair) const {
            return ProblemTwoSolver::ExpandOverlappingSpelledDigit(line, overlapping_expansion_pair);
        }
        std::string ConvertAllSpelledDigitsToNumbers(std::string line) const {
            return ProblemTwoSolver::ConvertAllSpelledDigitsToNumbers(line);
        }
        std::string ConvertSpelledDigitToNumber(std::string line, StringMapPair spelled_to_numeric_pair) const {
            return ProblemTwoSolver::ConvertSpelledDigitToNumber(line, spelled_to_numeric_pair);
        }
        std::string ReplaceSubstring(std::string string, std::string substring, std::string replacement) const {
            return ProblemTwoSolver::ReplaceSubstring(string, substring, replacement);
        }
        bool IsSubstringInString(std::string string, std::string substring) const {
            return ProblemTwoSolver::IsSubstringInString(string, substring);
        }
        size_t SubstringStartPos(std::string string, std::string substring) const {
            return ProblemTwoSolver::SubstringStartPos(string, substring);
        }
};


TEST(Problem2, PreProcessLineWithOverlappingDigits) {
    MockFileReader mock_file_reader;
    ProblemTwoTest problem_two = ProblemTwoTest(&mock_file_reader);

    std::string line = "xtwone3four";
    std::string converted = problem_two.PreProcessLine(line);
    std::string expected = "x2134";

    ASSERT_EQ(converted, expected);
}

TEST(Problem2, ExpandAllOverlappingSpelledDigits) {
    MockFileReader mock_file_reader;
    ProblemTwoTest problem_two = ProblemTwoTest(&mock_file_reader);

    std::string line = "xtwone3four";
    std::string converted = problem_two.ExpandAllOverlappingSpelledDigits(line);
    std::string expected = "xtwoone3four";

    ASSERT_EQ(converted, expected);
}


TEST(Problem2, ConvertAllSpelledDigitsToNumbers) {
    MockFileReader mock_file_reader;
    ProblemTwoTest problem_two = ProblemTwoTest(&mock_file_reader);

    std::string line = "1two3four5six7eight9";
    std::string converted = problem_two.ConvertAllSpelledDigitsToNumbers(line);
    std::string expected = "123456789";

    ASSERT_EQ(converted, expected);
}

TEST(Problem2, ConvertAllSpelledDigitsToNumbersAllDigitsSpelled) {
    MockFileReader mock_file_reader;
    ProblemTwoTest problem_two = ProblemTwoTest(&mock_file_reader);

    std::string line = "onetwothreefourfivesixseveneightnine";
    std::string converted = problem_two.ConvertAllSpelledDigitsToNumbers(line);
    std::string expected = "123456789";

    ASSERT_EQ(converted, expected);
}

TEST(Problem2, ConvertAllSpelledDigitsToNumbersDuplicateDigits) {
    MockFileReader mock_file_reader;
    ProblemTwoTest problem_two = ProblemTwoTest(&mock_file_reader);

    std::string line = "onetwothreefouronetwothreefour";
    std::string converted = problem_two.ConvertAllSpelledDigitsToNumbers(line);
    std::string expected = "12341234";

    ASSERT_EQ(converted, expected);
}



TEST(Problem2, ConvertSpelledDigitToNumber) {
    MockFileReader mock_file_reader;
    ProblemTwoTest problem_two = ProblemTwoTest(&mock_file_reader);

    std::string line = "1one2";

    std::string converted = problem_two.ConvertSpelledDigitToNumber(
        line,
        StringMapPair {"one", "1"}
    );
    std::string expected = "112";

    ASSERT_EQ(converted, expected);
}


TEST(Problem2, ConvertSpelledDigitToNumberNotFound) {
    MockFileReader mock_file_reader;
    ProblemTwoTest problem_two = ProblemTwoTest(&mock_file_reader);

    std::string line = "1one2";

    std::string converted = problem_two.ConvertSpelledDigitToNumber(
        line,
        StringMapPair {"two", "2"}
    );
    std::string expected = line;  // No changes expected to line.

    ASSERT_EQ(converted, expected);
}

TEST(Problem2, ConvertSpelledDigitToNumberMultipleFound) {
    MockFileReader mock_file_reader;
    ProblemTwoTest problem_two = ProblemTwoTest(&mock_file_reader);

    std::string line = "1one2one3";

    std::string converted = problem_two.ConvertSpelledDigitToNumber(
        line,
        StringMapPair {"one", "1"}
    );
    std::string expected = "11213";

    ASSERT_EQ(converted, expected);
}

TEST(Problem2, ReplaceSubstring) {
    MockFileReader mock_file_reader;
    ProblemTwoTest problem_two = ProblemTwoTest(&mock_file_reader);

    std::string original = "foobarbaz";
    std::string modified = problem_two.ReplaceSubstring(
        original,
        "bar",
        "boo"
    );
    std::string expected = "fooboobaz";

    ASSERT_EQ(modified, expected);
}

TEST(Problem2, IsSubstringInStringTrue) {
    MockFileReader mock_file_reader;
    ProblemTwoTest problem_two = ProblemTwoTest(&mock_file_reader);

    std::string string = "1one2";
    std::string substring = "one";

    bool is_substring_in_string = problem_two.IsSubstringInString(string, substring);

    ASSERT_EQ(is_substring_in_string, true);
}

TEST(Problem2, IsSubstringInStringFalse) {
    MockFileReader mock_file_reader;
    ProblemTwoTest problem_two = ProblemTwoTest(&mock_file_reader);

    std::string string = "1one2";
    std::string substring = "two";

    bool is_substring_in_string = problem_two.IsSubstringInString(string, substring);

    ASSERT_EQ(is_substring_in_string, false);
}


TEST(Problem2, SubstringStartPos) {
    MockFileReader mock_file_reader;
    ProblemTwoTest problem_two = ProblemTwoTest(&mock_file_reader);

    std::string string = "1one2";
    std::string substring = "one";

    size_t position = problem_two.SubstringStartPos(string, substring);
    size_t expected = 1;

    ASSERT_EQ(position, expected);
}

TEST(Problem2, SubstringStartPosWhenSubstringNotFound) {
    MockFileReader mock_file_reader;
    ProblemTwoTest problem_two = ProblemTwoTest(&mock_file_reader);

    std::string string = "1one2";
    std::string substring = "two";

    size_t position = problem_two.SubstringStartPos(string, substring);
    size_t expected = std::string::npos;

    ASSERT_EQ(position, expected);
}

// xtwone3four
// xtwo2twone1one
// -  this clever trick is what I did in rust. replace
//    each instance of a digit spelling with <spelled><digit><spelled>,
//    which leads to retaining any overlap before/after the spelled digit.
//    For example - "one" becomes "one1one", "two" becomes "two2two",
//    and "twone" becomes "two2twone1one"
// - HOWEVER, this doesn't work for my recursive strategy or re-calling
//   the conversion method if an instance of the spelled digit is found.
