#!/usr/bin/env bash

die () { >&2 echo "$1"; exit 1; }

prompt="${PWD}/target/debug/prompt"

assert_prompt () {
    out="$($prompt)"
    [[ "$out" = "$1" ]]
    rc=$?
    if [[ $rc -eq 0 ]]; then
        echo -e "passed!\n"
    else
        echo -e "Expected: '${1}'\nObserved: '${out}'\n"
    fi
    return $rc
}

tmp="$(mktemp -d)" || die 'mktemp failed'
trap "rm -rf ${tmp}" EXIT

(
mkdir "${tmp}/test-1" && cd "${tmp}/test-1"

echo "1.1: non-git dir"
assert_prompt "${USER}@$(hostname) $(basename "$PWD") $ "

echo "1.2: init bare repository"
git init >/dev/null
assert_prompt "${USER}@$(hostname) $(basename "$PWD") (empty branch) $ "

echo "1.3: clean master branch"
git commit --allow-empty -m "foo" >/dev/null
assert_prompt "${USER}@$(hostname) $(basename "$PWD") $(git branch --show-current) $ "

echo "1.4: checkout new clean branch"
git checkout -b foobar >/dev/null 2>&1
assert_prompt "${USER}@$(hostname) $(basename "$PWD") $(git branch --show-current) $ "

echo "1.5: add untracked file"
touch f
assert_prompt "${USER}@$(hostname) $(basename "$PWD") * $(git branch --show-current) $ "

echo "1.6: stage that file"
git add f
assert_prompt "${USER}@$(hostname) $(basename "$PWD") * $(git branch --show-current) $ "

echo "1.7: commit that file (clean branch)"
git commit -m "foo" >/dev/null
assert_prompt "${USER}@$(hostname) $(basename "$PWD") $(git branch --show-current) $ "

echo "1.8: remove tracked file"
rm f
assert_prompt "${USER}@$(hostname) $(basename "$PWD") * $(git branch --show-current) $ "

echo "1.9: undo the removal"
git checkout f >/dev/null 2>&1
assert_prompt "${USER}@$(hostname) $(basename "$PWD") $(git branch --show-current) $ "

# TODO: checkout to detach head. git branch --show-current will be blank, so additionally assert for that.

# TODO: prompt but not inside gitroot
)

(
mkdir "${tmp}/test-2" && cd "${tmp}/test-2"
git init >/dev/null

echo "2.1: set up gitignore for a file"
echo "f" > .gitignore
git add .gitignore
git commit -m "foo" >/dev/null
assert_prompt "${USER}@$(hostname) $(basename "$PWD") $(git branch --show-current) $ "

echo "2.2: add the gitignored file"
echo "data" > f
assert_prompt "${USER}@$(hostname) $(basename "$PWD") $(git branch --show-current) $ "
)
