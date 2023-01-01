prompt: prompt.nim
	/Users/josh/.nimble/bin/nim c prompt

release: prompt.nim
	/Users/josh/.nimble/bin/nim --gc:none -d:release --opt:size c prompt
	strip ./prompt
