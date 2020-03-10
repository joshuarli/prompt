#!/usr/bin/env zsh

#fg_green=$'\e[32m'
#fg_cyan=$'\e[36m'
#fg_red_bold=$'\e[1;31m'
#fg_white_bold=$'\e[1;97m'
#reset=$'\e[0m'

prompt_pwd () {
    #[ "$PWD" = "/home/${USER}" ] && wd='~' || wd="${PWD##*/}"
    wd="${PWD##*/}"
    #printf %s "${fg_cyan}${wd}${reset}"
    printf %s "$wd"
}

prompt_git () {
    cur_branch="$(git rev-parse --abbrev-ref HEAD 2>/dev/null)" || return
    [ -z "$(git status --porcelain -unormal)" ] || printf %s ' *'
    [ "$cur_branch" = HEAD ] && cur_branch='(detached HEAD)'
    #printf %s " ${fg_green}${cur_branch}${reset}"
    printf %s " ${cur_branch}"
}

prompt () {
    #echo "${fg_white_bold}${USER}${reset}@${fg_red_bold}${HOST}${reset} $(prompt_pwd)$(prompt_git) $ "
    echo "${USER}@${HOST} $(prompt_pwd)$(prompt_git) $ "
}

prompt
