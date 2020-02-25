## prompt

this is my new prompt.

in git repos, my old shell-based prompt shelled out to git 2 times to determine the branch name and whether the index is changed.

i thought i could make it faster, and for this small git repository it is:

    $ ./bench . ./old-prompt.sh
    0m0.009s 0m0.003s
    0m0.901s 0m0.347s

    $ ./bench . ./prompt.sh
    0m0.007s 0m0.003s
    0m0.697s 0m0.225s

but for a particularly large one, performance is heavily degraded right now:

    $ ./bench ~/dev/scratch/sentry ./old-prompt.sh
    0m0.009s 0m0.000s
    0m1.258s 0m1.964s
    
    $ ./bench ~/dev/scratch/sentry ./prompt.sh
    0m0.005s 0m0.005s
    0m3.949s 0m2.759s

...most likely meaning i'm not optimally using git2.


## usage

there's optional colorization via these env vars:

- `PROMPT_STYLE_USER`
- `PROMPT_STYLE_HOSTNAME`
- `PROMPT_STYLE_WD`
- `PROMPT_STYLE_BRANCH`
- `PROMPT_STYLE_RESET`

for example:

```sh
export PROMPT_STYLE_HOSTNAME="$(tput setaf 1)"  # red
export PROMPT_STYLE_WD="$(tput setaf 6)"        # cyan
export PROMPT_STYLE_BRANCH="$(tput setaf 2)"    # green
export PROMPT_STYLE_RESET="$(tput sgr0)"
export PS1='$(prompt)'
```
