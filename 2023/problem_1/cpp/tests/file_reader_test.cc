#include <gtest/gtest.h>

#include "file_reader.h"

TEST(FileLineTest, Constructor) {
    const char* line = "1abc2";
    FileLine file_line = FileLine(line);
}

TEST(FileLineTest, GetReadStateOk) {
    const char* line = "1abc2";
    FileLine file_line = FileLine(line);

    ASSERT_EQ(file_line.get_read_state(), kLineReadOk);
}

TEST(FileLineTest, GetReadStateError) {
    const char* line =
        "54nzzddht8ninelrkkseightseven6"
        "54nzzddht8ninelrkkseightseven6"
        "54nzzddht8ninelrkkseightseven6"
        "54nzzddht8ninelrkkseightseven6";
    FileLine file_line = FileLine(line);

    ASSERT_EQ(file_line.get_read_state(), kLineReadError);
}

TEST(FileLineTest, NumChars) {
    const char* line = "1abc2";
    FileLine file_line = FileLine(line);

    size_t num_chars = file_line.NumChars();
    ASSERT_EQ(num_chars, 5);
}

TEST(FileLineTest, GetCharsNominal) {
    const char* line = "1abc2";
    FileLine file_line = FileLine(line);

    char chars[FILE_MAX_CHARS_PER_LINE] = {0};
    FileLineGetCharsResult result = file_line.GetChars(chars);

    ASSERT_STREQ(chars, line);
    ASSERT_EQ(result, kGetCharsOk);
}


TEST(FileLineTest, GetCharsStringTooLongReturnsFailedResult) {
    const char* line =
        "54nzzddht8ninelrkkseightseven6"
        "54nzzddht8ninelrkkseightseven6"
        "54nzzddht8ninelrkkseightseven6"
        "54nzzddht8ninelrkkseightseven6";

    FileLine file_line = FileLine(line);

    char chars[FILE_MAX_CHARS_PER_LINE] = {0};
    FileLineGetCharsResult result = file_line.GetChars(chars);

    ASSERT_STREQ(chars, "");
    ASSERT_EQ(result, kGetCharsFailure);
}
