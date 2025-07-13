//explain how child is forked from each parent

#include <stdio.h>
#include <sys/types.h>
#include <unistd.h>
#include <stdlib.h>

int main() {
    fork();
    fork();
    fork();
    printf("hello\n");
    return 0;
}

/*
    Process Tree for:

        fork();
        fork();
        fork();
        printf("hello\n");

    Each fork() doubles the number of processes.
    
    Visualization (each → is a fork):

        P0
         ├── P1
         │    ├── P3
         │    │    └── P7
         │    └── P4
         └── P2
              ├── P5
              └── P6

    Total processes after 3 forks: 8

    Each prints:
        hello
        hello
        ...
        (8 times total)

    This grows exponentially with each fork().
*/

