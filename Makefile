name = prompt
cflags = -std=c99 -O0 -march=native -Wpedantic
cflags_release = $(cflags) -O2
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
