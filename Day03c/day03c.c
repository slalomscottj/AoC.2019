#include <stdio.h>
#include "move.h"
#include "visit.h"

void process(MOVE * aMoves, MOVE * bMoves) {
    VISIT * aVisits = playMoves(aMoves);
    VISIT * bVisits = playMoves(bMoves);
    VISIT * iVisits = intersection(aVisits, bVisits);

    VISIT * shortest = NULL;
    int shortestDistance;

    VISIT * index = iVisits;
    while(index) {

        int d = distance(index, aVisits) + 1 +
                distance(index, bVisits) + 1;

        if(NULL == shortest || shortestDistance > d) {
                shortest = index;
                shortestDistance = d;
        }
        
        index = (*index).next;
    }
   
    printVisit(shortest);
    printf(" -> %i\n\n", shortestDistance);

    freeVisits(&iVisits);
    freeVisits(&bVisits);
    freeVisits(&aVisits);

    return;
}

void main(int argc, char * argv[]) {
    if(argc < 2)
        return;
    
    FILE * hFile = fopen(argv[1], "rt");

    while(!feof(hFile)) {
        MOVE * aMoves = readMoves(hFile);
        MOVE * bMoves = readMoves(hFile);

        process(aMoves, bMoves);

        freeMoves(&aMoves);
        freeMoves(&bMoves);
    }

    fclose(hFile);
    hFile = NULL;
}