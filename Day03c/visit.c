#include <stdio.h>
#include "visit.h"

VISIT * internalPlayMoves(MOVE * move, VISIT * lastVisit) {
    int ix = 0;
    int iy = 0;
 
    switch((*move).direction) {
        case 'R':
            ix = 1;
            break;
        case 'U':
            iy = 1;
            break;
        case 'L':
            ix = -1;
            break;
        case 'D':
            iy = -1;
            break;
    }

    VISIT * result = NULL;
    VISIT * thisVisit = NULL;

    int cx = NULL == lastVisit ? 0 : (*lastVisit).x;
    int cy = NULL == lastVisit ? 0 : (*lastVisit).y;    
    int d = (*move).distance;
    
    while(d--) {
        cx += ix;
        cy += iy;

        VISIT * nextVisit = (VISIT *) malloc(sizeof(VISIT));
        (*nextVisit).x = cx;
        (*nextVisit).y = cy;
        (*nextVisit).next = NULL;

        if(NULL == thisVisit)
            thisVisit = nextVisit;
        else {
            (*thisVisit).next = nextVisit;
            thisVisit = nextVisit;
        }

        if(NULL == result)
            result = nextVisit;
    }

    if((*move).next)
        (*thisVisit).next = internalPlayMoves((*move).next, thisVisit);

    return result;
}

VISIT * playMoves(MOVE * moves) {
    return internalPlayMoves(moves, NULL);
}

void printVisit(VISIT * visit) {
    printf("(%i,%i)", (*visit).x, (*visit).y);
}

void internalPrintVisits(VISIT * visit) {

    printVisit(visit);
    
    if(NULL != (*visit).next) {
        fputc(',',stdout);
        internalPrintVisits((*visit).next);
    }
}

// Returns 0 if a and be are equal
int equal(VISIT * a, VISIT * b) {
    return (*a).x == (*b).x && (*a).y == (*b).y;
}

VISIT * shallowCopy(VISIT * visit) {
    VISIT * result = (VISIT *) malloc(sizeof(VISIT));

    (*result).x = (*visit).x;
    (*result).y = (*visit).y;
    (*result).next = NULL;

    return result;
}

VISIT * append(VISIT * visit, VISIT * visits) {
    if(visits) {
        
        // find last item
        VISIT * index = visits;
        while((*index).next) {
            index = (*index).next;
        }

        (*index).next = visit;

        return visits;

    }
    else {
        return visit;
    }
}

int indexOfFirst(VISIT * visit, VISIT * visits) {
    if(visits) {
        int i = 0;
        VISIT * index = visits;

        while(index) {
            if(equal(index, visit))
                return i;

            i++;
            index = (*index).next;
        }
    }

    return -1;
}

VISIT * intersection(VISIT * aVisits, VISIT * bVisits) {
    VISIT * result = NULL;

    VISIT * aIndex = aVisits;
    while(aIndex) {

        VISIT * bIndex = bVisits;
        while(bIndex) {
            if(equal(aIndex, bIndex))
                result = append(shallowCopy(aIndex), result);

            bIndex = (*bIndex).next;
        }

        aIndex = (*aIndex).next;
    }

    return result;
}

void printVisits(VISIT * visit) {
    internalPrintVisits(visit);
    fputc('\n',stdout);
    fputc('\n',stdout);
}


void freeVisits(VISIT ** visit) {
    if(NULL != (*visit) -> next)
        freeVisits(&((*visit) -> next));
    
    free(*visit);
    *visit = NULL;
}

int distance(VISIT * visit, VISIT * visits) {
    int i = 0;
    VISIT * index = visits;
    while(index) {
        if(equal(visit, index))
            return i;

        i++;
        index = (*index).next;
    }

    return -1;
}