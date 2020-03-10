#define _POSIX_C_SOURCE 200809L
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <strings.h>
#include <sys/time.h>
#include <unistd.h>

#include "git2.h"

#define FORMAT_BASE "%s%s%s@%s%s%s %s%s%s"
#define SHORT_FORMAT FORMAT_BASE " $ \n"
#define SHORT_FORMAT_ARGS                                                      \
  style_user, user, style_reset, style_hostname, hostname, style_reset,        \
      style_wd, wd, style_reset

#define LONG_FORMAT FORMAT_BASE "%s %s%s%s $ \n"
#define LONG_FORMAT_ARGS                                                       \
  SHORT_FORMAT_ARGS, changes, style_branch, branch, style_reset

#define TIMER_INIT() struct timespec _timerstart, _timerend;
#define START_TIMER() clock_gettime(CLOCK_MONOTONIC_RAW, &_timerstart);
#define END_TIMER(name)                                                        \
  {                                                                            \
    clock_gettime(CLOCK_MONOTONIC_RAW, &_timerend);                            \
    uint64_t delta_us = (_timerend.tv_sec - _timerstart.tv_sec) * 1000000 +    \
                        (_timerend.tv_nsec - _timerstart.tv_nsec) / 1000;      \
    fprintf(stderr, name ": %lluus\n", delta_us);                              \
  }

#define CHANGES_FOUND -1
#define CHANGES_NOT_FOUND 1

static const char *getenv_default(const char *var) {
  const char *rv = getenv(var);
  return rv == NULL ? "" : rv;
}

static void get_last_segment(const char **out, const char *s, const char c) {
  // Truncate s to everything after the last / and
  // shove back into out
  int i = 0;
  *out = s;
  while (s[i++] != '\0') {
    if (s[i] == c) {
      *out = &s[i + 1];
    }
  }
}

static int diff_cb(const git_diff *diff_so_far,
                   const git_diff_delta *delta_to_add,
                   const char *matched_pathspec, void *payload) {
  // printf("diff_cb %s\n", matched_pathspec);
  fprintf(stderr,
          "flags=%d status=%d similarity=%d nfiles=%d oldpath=%s newpath=%s\n",
          delta_to_add->flags, delta_to_add->status, delta_to_add->similarity,
          delta_to_add->nfiles, delta_to_add->old_file.path,
          delta_to_add->new_file.path);
  return delta_to_add->status > GIT_DELTA_UNMODIFIED ? CHANGES_FOUND
                                                     : CHANGES_NOT_FOUND;
}

int main(int argc, char **argv) {
  const char *user = getenv_default("USER"),
             *style_user = getenv_default("PROMPT_STYLE_USER"),
             *style_hostname = getenv_default("PROMPT_STYLE_HOSTNAME"),
             *style_wd = getenv_default("PROMPT_STYLE_WD"),
             *style_branch = getenv_default("PROMPT_STYLE_BRANCH"),
             *style_reset = getenv_default("PROMPT_STYLE_RESET");

  char cwd[PATH_MAX], hostname[PATH_MAX];
  const char *branch, *wd, *changes = "";

  TIMER_INIT();

  getcwd(cwd, sizeof(cwd));
  gethostname(hostname, sizeof(hostname));

  get_last_segment(&wd, cwd, '/');

  git_libgit2_init();

  START_TIMER();

  git_buf gitroot = {0};
  if (git_repository_discover(&gitroot, cwd, 0, NULL) != 0) {
    printf(SHORT_FORMAT, SHORT_FORMAT_ARGS);
    return 0;
  }

  // END_TIMER("git_repository_discover");

  START_TIMER();

  git_repository *repo;
  if (git_repository_open(&repo, gitroot.ptr) != 0) {
    printf(SHORT_FORMAT, SHORT_FORMAT_ARGS);
    return 0;
  }
  git_buf_free(&gitroot);

  // END_TIMER("git_repository_open");

  // START_TIMER();
  // git_status_list *statuses = NULL;
  // git_status_options opts = GIT_STATUS_OPTIONS_INIT;
  // opts.flags |= GIT_STATUS_OPT_DEFAULTS | GIT_STATUS_OPT_EXCLUDE_SUBMODULES |
  // GIT_STATUS_OPT_NO_REFRESH;
  // // opts.show = GIT_STATUS_SHOW_WORKDIR_ONLY;
  // printf("%d\n", opts.flags & GIT_STATUS_OPT_NO_REFRESH);
  // git_status_list_new(&statuses, repo, &opts);
  // END_TIMER("git_status_list_new");

  // START_TIMER();
  // size_t count = git_status_list_entrycount(statuses);
  // // END_TIMER("git_status_list_entrycount");

  // START_TIMER();
  // for (int i = 0; i < count; ++i) {
  //   const git_status_entry *entry = git_status_byindex(statuses, i);
  //   if ((entry->status &
  //        (GIT_STATUS_INDEX_NEW | GIT_STATUS_INDEX_MODIFIED |
  //         GIT_STATUS_INDEX_DELETED | GIT_STATUS_INDEX_TYPECHANGE |
  //         GIT_STATUS_WT_NEW | GIT_STATUS_WT_MODIFIED | GIT_STATUS_WT_DELETED
  //         | GIT_STATUS_WT_RENAMED | GIT_STATUS_WT_TYPECHANGE)) > 0) {
  //     changes = " *";
  //     break;
  //   }
  // }
  // git_status_list_free(statuses);
  // END_TIMER("loop over statuses");

  START_TIMER();
  git_diff_options diffopts = GIT_DIFF_OPTIONS_INIT;
  git_diff *diff;
  diffopts.flags = GIT_DIFF_INCLUDE_UNTRACKED | GIT_DIFF_INCLUDE_TYPECHANGE |
                   GIT_DIFF_ENABLE_FAST_UNTRACKED_DIRS;
  diffopts.notify_cb = diff_cb;
  int rv = git_diff_index_to_workdir(&diff, repo, NULL, &diffopts);
  END_TIMER("git_diff_index_to_workdir");
  // printf("rv=%d\n", rv);
  if (rv == CHANGES_FOUND) {
    changes = " *";
  }

  START_TIMER();
  git_reference *ref = NULL;
  if (git_repository_head(&ref, repo) != 0) {
    branch = "(empty branch)";
  } else {
    branch = git_reference_shorthand(ref);

    // XXX: this isn't 100% correct, but it'll do. I don't want to additionally
    // inspect symbolic ref.
    if (strcmp(branch, "HEAD") == 0) {
      branch = "(detached HEAD)";
    }
  }
  git_reference_free(ref);
  // END_TIMER("git_repository_head");

  printf(LONG_FORMAT, LONG_FORMAT_ARGS);
  return 0;
}
