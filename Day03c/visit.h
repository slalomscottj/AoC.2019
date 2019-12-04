#ifndef __VISIT__
#define __VISIT__

#include "move.h"

typedef struct Visit {
    int x;
    int y;
    struct Visit * next;
} VISIT;

VISIT * playMoves(MOVE * moves);
void printVisit(VISIT * visit);
void printVisits(VISIT * visit);
void freeVisits(VISIT ** visit);

VISIT * intersection(VISIT * aVisits, VISIT * bVIsits);
int distance(VISIT * visit, VISIT * visits);

#endif