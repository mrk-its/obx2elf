all:	engine.o
	mos-a800xl-clang sprites.c engine.o sprites.S font0.S -o sprites.xex -O3
	mos-a800xl-clang sprites.c engine.o sprites.S font0.S -o sprites.elf -Wl,--oformat=elf -O3
	./labels.sh sprites.elf > sprites.lab

engine.o:
	obx2elf engine.obx engine.o -a 2048

clean:
	rm -f *.o *.elf *.xex *.lab
