#include <stdio.h>
#include "move.h"

MOVE * readMove(FILE * file) {
    MOVE * result = (MOVE *) malloc(sizeof(MOVE));

    (*result).direction = fgetc(file);
    (*result).distance = 0;
    (*result).next = NULL;
    
    char c;

    while(c = fgetc(file), isdigit(c)) {
        (*result).distance *= 10;
        (*result).distance += c-'0';
    }

    ungetc(c, file);

    return result;
}

MOVE * internalReadMoves(FILE * file, MOVE * last) {
    MOVE * this = readMove(file);

    char c;
    do {
        c = fgetc(file);
    } while(',' == c);

    if(isalpha(c)) {
        ungetc(c, file);
        internalReadMoves(file, this);
    }

    if(NULL == last)
        return this;
    else {
        (*last).next = this;
        return last;
    }
}

MOVE * readMoves(FILE * file) {
    return internalReadMoves(file, NULL);
}

void internalPrintMoves(MOVE * move) {

    printf("%c%i", (*move).direction, (*move).distance);

    if(NULL != (*move).next) {
        fputc(',',stdout);
        internalPrintMoves((*move).next);
    }
}

void printMoves(MOVE * move) {
    internalPrintMoves(move);
    fputc('\n',stdout);
}

void freeMoves(MOVE ** move) {
    if(NULL != (*move) -> next)
        freeMoves(&((*move) -> next));
    
    free(*move);
    *move = NULL;
}
