all: wslconf

clean: rm wslconf

wslconf: wslconf.c wslconf.h wslu.h
	gcc -o wslconf wslconf.c