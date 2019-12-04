#include <stdio.h>

void main() {
    char buffer[7];

    int count = 0;
    
    for(int i = 124075; i <= 580769; i++) {
        sprintf(buffer,"%i", i);

        // Day04a
        int duplicates = 0;
        int descending = 0;
        // Day04b
        int pairs = 0;

        for(int j = 0; j < 5; j++) {
            // Day04a
            if(buffer[j] == buffer[j+1])
                duplicates++;

            if(buffer[j] > buffer[j+1])
                descending++;

            // Day04b
            switch(j) {
                case 0:
                    if(buffer[j] == buffer[j+1] && buffer[j] != buffer[j+2])
                        pairs++;
                    break;
                case 5:
                    if(buffer[j] == buffer[j+1] && buffer[j-1] != buffer[j])
                        pairs++;
                    break;
                default:
                    if(buffer[j] == buffer[j+1] && buffer[j-1] != buffer[j] && buffer[j+2] != buffer[j])
                        pairs++;
                    break;
            }
        }

        if(duplicates > 0 && descending == 0 // Day04a
            && pairs > 0)                    // Day04b
            count++;    
    }

    printf("%i", count);
}