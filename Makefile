make:
	gcc -Wall main.c -o pomodoro -lncurses

run:
	gcc -Wall main.c -o pomodoro -lncurses; ./pomodoro

install:
	gcc -Wall main.c -o pomodoro -lncurses; mv pomodoro /usr/local/bin/
