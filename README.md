## prompt

this is my new prompt.

in git repos, my old shell-based prompt shells out to git 2 times to determine the branch name and whether the index is changed.

i thought i could make it faster, but right now it's pretty much the same speed as my old prompt, which is kind of disappointing...

    $ ./bench . ./old-prompt.sh
    0m0.003s 0m0.007s
    0m1.090s 0m1.153s

    $ ./bench . ./prompt.sh
    0m0.003s 0m0.004s
    0m1.144s 0m0.936s
