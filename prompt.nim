import os
import std/strformat
from std/strutils import rsplit

proc gethostname(name: cstring, len: csize_t): int32
  {.cdecl, importc: "gethostname", header: "<unistd.h>".}

# 255
# let HOST_NAME_MAX {.importc: "HOST_NAME_MAX", header: "<unistd.h>".}: csize_t

proc hostname(): string =
  var buf = cast[cstring](alloc(256))
  if gethostname(buf, 255) == 0:
    result = $buf
  dealloc(buf)

let user = getEnv("USER")
let host = hostname()

var cwd = getCurrentDir()

var gitroot = cwd
var branch = ""
while true:
  let fn = &"{gitroot}/.git/HEAD"
  if fileExists(fn):
    let f = open(fn)
    let l = readLine(f)
    close(f)
    # ref: refs/heads/master
    let tailsplit = rsplit(l, {'/'}, maxsplit = 1)
    branch = tailsplit[1]
    break
  let tailsplit = rsplit(gitroot, {'/'}, maxsplit = 1)
  if tailsplit.len == 1:  # /
    break
  gitroot = tailsplit[0]

let tailsplit = rsplit(cwd, {'/'}, maxsplit = 1)
cwd = tailsplit[1]
if cwd == user:
  cwd = "~"
elif cwd == "":
  if tailsplit[1] == "":
    cwd = "/"

# gave up on detecting changes to git worktree and index
# too much of a perf hit
# https://stackoverflow.com/questions/4075528/what-algorithm-does-git-use-to-detect-changes-on-your-working-tree

# zsh has this vcs_info function which has check-for-changes, but ultimately this just shells out to git.

if branch == "":
  write(stdout, &"{user}@{host} {cwd} $ ")
else:
  write(stdout, &"{user}@{host} {cwd} {branch} $ ")

flushFile(stdout)
