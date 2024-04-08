#ifndef FILE_READER_H
#define FILE_READER_H

#include <memory>
#include <stdio.h>
#include <string>
#include <vector>


class IFileManager {
    public:
        virtual ~IFileManager() {};
        virtual std::string FileData() const = 0;
};

class FileManager : public IFileManager {
    public:
        ~FileManager();
        FileManager(FILE* file_pointer);
        std::string FileData() const;

    private:
        FILE* file_pointer;
};

typedef std::vector<std::string> FileLines;

class IFileReader {
    public:
        virtual ~IFileReader() {};
        virtual FileLines ReadLines() const = 0;
};

class FileReader : public IFileReader {
    public:
        FileReader(const IFileManager* file_manager);
        FileLines ReadLines() const;

    private:
        const IFileManager* file_manager;
};

#endif