#pragma once

// Functions to set stdin to be character based so I can catch inputs
void reset_terminal_mode();
void set_conio_terminal_mode();
int keyboard_hit();
int get_char();