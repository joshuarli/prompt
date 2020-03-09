name = prompt
cflags = -std=c99 -O2 -march=native -Wpedantic
# XXX: -O0 doesn't currently work under c99, need to put inline funcs in a different compilation unit
cflags_release = $(cflags) -O2 -flto
# TODO: compare -Os vs -O2 -flto size
ldflags = $(shell pkg-config --libs libgit2)

$(name): src/main.c
	clang -v $(ldflags) $(cflags) src/main.c -o $(name)

release: src/main.c
	clang $(cflags_release) $(ldflags) src/main.c -o $(name)

fmt: src/main.c
	clang-format -i src/main.c

test: $(name)
	./test.sh

clean:
	rm -f $(name)
