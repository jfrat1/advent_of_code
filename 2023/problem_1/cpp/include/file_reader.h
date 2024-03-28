#ifndef FILE_H
#define FILE_H

#include <stdio.h>

#define FILE_MAX_NUM_LINES 1000
#define FILE_MAX_CHARS_PER_LINE 100

enum FileReaderGetLinesResult {
    kFileReaderGetLinesOk = 0,
    kFileReaderGetLinesError = 1,
};

enum FileLineReadState {
    kLineReadOk = 0,
    kLineReadError = 1,
};

enum FileLineGetCharsResult {
    kGetCharsOk = 0,
    kGetCharsFailure = 1,
};

class IFileLine {
    public:
        virtual ~IFileLine() {};
        virtual FileLineGetCharsResult GetChars(char* chars) = 0;
        virtual size_t NumChars() = 0;
};

class FileLine : IFileLine {
    public:
        FileLine(const char* line);
        FileLineReadState get_read_state();
        size_t NumChars();
        FileLineGetCharsResult GetChars(char* chars);


    private:
        char line[FILE_MAX_CHARS_PER_LINE];
        FileLineReadState read_state;
};


class IFileReader {
    public:
        virtual ~IFileReader() {};
        virtual FileReaderGetLinesResult GetLines(IFileLine* lines) = 0;
};

class FileReader : public IFileReader {
    public:
        FileReader(FILE* fp);
        FileReaderGetLinesResult GetLines(IFileLine* lines);

    private:
        FILE* file_pointer;
};


#endif