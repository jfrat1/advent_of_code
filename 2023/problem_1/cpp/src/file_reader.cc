#include <cstring>
#include <stdio.h>

#include "file_reader.h"

FileReader::FileReader(FILE* fp) {
    file_pointer = fp;
}

FileReaderGetLinesResult FileReader::GetLines(IFileLine* lines) {
    return kFileReaderGetLinesError;
}

FileLine::FileLine(const char* line) {
    if (strlen(line) > FILE_MAX_CHARS_PER_LINE) {
        this->read_state = kLineReadError;
        strcpy(this->line, "");
    } else {
        this->read_state = kLineReadOk;
        strcpy(this->line, line);
    }
}

FileLineReadState FileLine::get_read_state() {
    return this->read_state;
}

size_t FileLine::NumChars() {
    return strlen(this->line);
}

FileLineGetCharsResult FileLine::GetChars(char* chars) {
    if (this->read_state == kLineReadError) {
        return kGetCharsFailure;
    }

    strcpy(chars, this->line);
    return kGetCharsOk;
}

