// Just a hello world to see how fork works

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int main(int argc, char *argv[])
{
    printf("hello world (pid:%d)\n", (int)getpid());
    int rc = fork();
    if (rc < 0)
    { // fork failed; exit
        fprintf(stderr, "fork failed\n");
        exit(1);
    }
    else if (rc == 0)
    { // child (new process)
        printf("hello, I am child (pid:%d)\n", (int)getpid());
    }
    else
    { // parent goes down this path (main)
        printf("hello, I am parent of %d (pid:%d)\n",
               rc, (int)getpid());
    }
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