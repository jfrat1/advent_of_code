#include <gtest/gtest.h>
#include <gmock/gmock.h>

#include "file_reader.h"


TEST(FileManager, ReadData) {
    const char* filename = "data/file_reader_test_data.txt";
    FILE* fp = fopen(filename, "r");

    FileManager manager = FileManager(fp);
    std::string file_data = manager.FileData();

    // This must match data from the file noted above
    std::string expected_data = std::string(
        "54nzzddht8ninelrkkseightseven6\n"
        "54nzzddht8ninelrkkseightseven6\n"
        "54nzzddht8ninelrkkseightseven6\n"
        "54nzzddht8ninelrkkseightseven6"
    );

    ASSERT_EQ(file_data, expected_data);
}


class MockFileManager : public IFileManager {
    public:
        MOCK_METHOD(std::string, FileData, (), (const override));
};


TEST(FileReader, ReadLines) {

    MockFileManager mock_file_manager;
    std::string test_file_string = std::string(
        "54nzzddht8ninelrkkseightseven6\n"
        "54nzzddht8ninelrkkseightseven6\n"
        "54nzzddht8ninelrkkseightseven6\n"
        "54nzzddht8ninelrkkseightseven6"
    );
    EXPECT_CALL(mock_file_manager, FileData())
        .WillOnce(testing::Return(test_file_string));


    FileReader reader = FileReader(&mock_file_manager);
    FileLines lines = reader.ReadLines();

    FileLines expected_lines {
        std::string("54nzzddht8ninelrkkseightseven6"),
        std::string("54nzzddht8ninelrkkseightseven6"),
        std::string("54nzzddht8ninelrkkseightseven6"),
        std::string("54nzzddht8ninelrkkseightseven6"),
    };

    ASSERT_EQ(lines, expected_lines);
}
