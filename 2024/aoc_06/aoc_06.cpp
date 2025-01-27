#include <iostream>
#include <tuple>
#include <stdlib.h>
#include "../../helper/c++/helper.hpp"

#define BUFLEN 200

struct MyGrid {
    char** grid;
    size_t rows;
    size_t cols;
    std::tuple<int, int> currentPosition;

    MyGrid(size_t r, size_t c) : rows(r), cols(c) {
        grid = new char*[rows];
        for (size_t i = 0; i < rows; i++) {
            grid[i] = new char[cols];
        }
    }

    ~MyGrid() {
        for (size_t i = 0; i < rows; ++i) {
            delete[] grid[i];
        }
        delete[] grid;
    }

};

std::tuple<int, int> get_size(const char* id, Helper helper) {

    FILE* fp;
    int rows = 0;
    int columns = 0;
    int i;

    std::tuple<int, int> result;
    std::make_tuple(rows, columns);

    std::string filepath = helper.get_input_path(id);
    fp = fopen(filepath.c_str(), "r");
    if (fp == NULL) {
        printf("cannot open file\n");
        fclose(fp);
    } else {
        while((i=fgetc(fp))!=EOF) {
            if (i != ' ' && i != '\n')  {
                ++columns;
            }
            if (i == '\n') {
                ++rows;
                std::get<1>(result) = columns;
                columns = 0;
            }
        }
    }
    
    std::get<0>(result) = rows;
   
    return result;
}

MyGrid load_grid(const char* file_id, Helper helper) {
    
    std::tuple<int, int> size = get_size(file_id, helper);

    MyGrid grid(std::get<0>(size), std::get<1>(size));
        
    std::string filepath = helper.get_input_path(file_id);

    char buf[BUFLEN];
    char *s;
    int lineno = 0;
    while (s = helper.fetch_line(filepath.c_str(), &lineno)) {
        printf("trimmed line %3d: %s\n", lineno, s);
    }

    return grid;
}

int main() {
    
    Helper helper;

    MyGrid grid = load_grid("aoc_2024_06", helper);

    return 0;
}


