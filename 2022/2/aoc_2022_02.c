#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include "../../helper/c++/my-clib/fetch_line/fetch-line.h"

#define BUFLEN 10

int calc_score(char *input, char *output){
    int score = 0;
    printf("%s\n", input);
    printf("%s", output);
    if(strcmp(input, "A") == 0){
        printf("Opponet Symbol: Rock\n");
        if(strcmp(output, "X\n") == 0){
            printf("Player Symbol: Rock\n");
            score += 0;
            score += 3;
        }
        if(strcmp(output, "Y\n") == 0){
            printf("Player Symbol: Paper\n");
            score += 3;
            score += 1;
        }
        if(strcmp(output, "Z\n") == 0){
            printf("Player Symbol: Scissors\n");
            score += 6;
            score += 2;
        }
        
    }
    if(strcmp(input, "B") == 0){
        printf("Opponet Symbol: Paper\n");
        if(strcmp(output, "X\n") == 0){
            printf("Player Symbol: Rock\n");
            score += 0;
            score += 1;
        }
        if(strcmp(output, "Y\n") == 0){
            printf("Player Symbol: Paper\n");
            score += 3;
            score += 2;
        }
        if(strcmp(output, "Z\n") == 0){
            printf("Player Symbol: Scissors\n");
            score += 6;
            score += 3;
        }
    }
    if(strcmp(input, "C") == 0){
        printf("Opponet Symbol: Scissors\n");
        if(strcmp(output, "X\n") == 0){
            printf("Player Symbol: Rock\n");
            score += 0;
            score += 2;
        }
        if(strcmp(output, "Y\n") == 0){
            printf("Player Symbol: Paper\n");
            score += 3;
            score += 3;
        }
        if(strcmp(output, "Z\n") == 0){
            printf("Player Symbol: Scissors\n");
            score += 6;
            score += 1;
        }
    }
    printf("Score is: %d\n", score);
    return score;
}

//XYZ = rock,paper,scissors
// rock = 1, paper = 2, scissors = 3
//win = 6, draw = 3, lose = 0

int main(void){

    FILE* fp;
    char  line[255];

    int total_score = 0;

    fp = fopen("input.txt" , "r");
    while (fgets(line, sizeof(line), fp) != NULL)
    {
        char* val1 = strtok(line, " ");
        char* val2 = strtok(NULL, " ");
        total_score += calc_score(val1, val2);;
    }

    printf("%d\n", total_score);
    
}
