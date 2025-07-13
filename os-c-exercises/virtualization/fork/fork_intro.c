// Just a hello world to see how fork works

#include <stdio.h>
#include <sys/types.h>
#include <unistd.h>
#include <stdlib.h>

int main() {
    pid_t p = fork();
    if(p<0) {
        perror("fork fail");
        exit(1);
    }

    printf("Hello world!, process_id(pid) = %d\n", getpid());
    return 0;
}

/*
    Visual representation of fork():

              +------------------+
              |  main() starts   |
              +--------+---------+
                       |
                   fork()
                /           \
          fork() == 0     fork() > 0
         (Child process)  (Parent process)
             |                 |
     printf() + PID     printf() + PID

    Both processes continue independently after fork()

    Example Output:
    ----------------
    Hello world!, process_id(pid) = 3456   <-- Parent
    Hello world!, process_id(pid) = 3457   <-- Child
*/