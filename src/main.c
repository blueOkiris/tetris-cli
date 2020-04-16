#include <stdio.h>
#include <stdlib.h>

#include <game.h>
#include <key.h>

int main(int argc, char **args) {
    // Get delay time from command line
    char *delay_str;
    long delay_time = DEFAULT_DELAY;
    if(argc > 1)
        delay_time = strtol(args[1], &delay_str, 10);

    while(1) {
        printf("\033[2J");
        printf("Tetris CLI by Dylan Turner cerca 2020.\n\n");
        printf("Controls:\n");
        printf(" - a/d -> move left/right respectively\n");
        printf(" - q/e -> rotate left/right\n");
        printf(" - s   -> drop piece quickly\n");
        printf(" - backspace -> quit game\n");
        printf("\nWhen starting the app, you can give a\n");
        printf("different delay time (in microseconds)\n");
        printf("to adjust for flicker\n");
        printf("\nDefault delay = %d\n", DEFAULT_DELAY);
        printf("Current delay = %ld\n", delay_time);
        printf("Recommended to keep new delay w/in a power of 10\n");
        printf("\nPress enter to begin...\n\n\n\n\n\n\n");

        set_conio_terminal_mode();

        int key = 0;
        while(key != 13) {
            while(!keyboard_hit());
            key = get_char();

            if(key == 127)
                goto quit;
        }

        play(delay_time);
        reset_terminal_mode();
    }

    quit:
        reset_terminal_mode();

    return 0;
}