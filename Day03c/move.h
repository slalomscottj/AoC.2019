#ifndef __MOVE__
#define __MOVE__

typedef struct Move {
    char direction;
    int distance;
    struct Move * next;
} MOVE;

MOVE * readMove(FILE * file);
MOVE * readMoves(FILE * file);
void printMoves(MOVE * move);
void freeMoves(MOVE ** move);

#endif