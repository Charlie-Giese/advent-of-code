#include <iostream>
#include <tuple>
#include <stdlib.h>
#include "../../helper/c++/my-clib/fetch_line/fetch-line.h"
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

std::tuple<int, int> get_size(const char id, Helper helper) {

    FILE* fp;
    int rows = 0;
    int columns = 0;
    int i;

    std::tuple<int, int> result;
    std::make_tuple(rows, columns);

    std::string filepath = helper.get_input_path(id);

    fp = fopen(filepath, "r");
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

void load_grid(const char* filepath, MyGrid g) {
    
    char buf[BUFLEN];
    char *s;
    int lineno = 0;
    while ((s = fetch_line(buf, BUFLEN, stdin, &lineno)) != NULL) {
        printf("trimmed line %3d: %s\n", lineno, s);
    }
}

int main() {
    
    Helper helper;

    std::tuple<int, int> size = get_size("aoc_2024_06", helper);

    //MyGrid grid(std::get<0>(size), std::get<1>(size));


    return 0;
}


