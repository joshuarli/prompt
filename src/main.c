#define _POSIX_C_SOURCE 200809L
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <strings.h>
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

int main(int argc, char **argv) {
  const char *user = getenv_default("USER"),
             *style_user = getenv_default("PROMPT_STYLE_USER"),
             *style_hostname = getenv_default("PROMPT_STYLE_HOSTNAME"),
             *style_wd = getenv_default("PROMPT_STYLE_WD"),
             *style_branch = getenv_default("PROMPT_STYLE_BRANCH"),
             *style_reset = getenv_default("PROMPT_STYLE_RESET");

  char cwd[PATH_MAX], hostname[PATH_MAX];
  const char *branch, *wd, *changes = "";

  getcwd(cwd, sizeof(cwd));
  gethostname(hostname, sizeof(hostname));

  get_last_segment(&wd, cwd, '/');

  git_libgit2_init();

  git_buf gitroot = {0};
  if (git_repository_discover(&gitroot, cwd, 0, NULL) != 0) {
    printf(SHORT_FORMAT, SHORT_FORMAT_ARGS);
    return 0;
  }

  git_repository *repo;
  if (git_repository_open(&repo, gitroot.ptr) != 0) {
    printf(SHORT_FORMAT, SHORT_FORMAT_ARGS);
    return 0;
  }
  git_buf_free(&gitroot);

  git_diff_options diffopts = GIT_DIFF_OPTIONS_INIT;
  git_diff *diff;
  diffopts.flags = GIT_DIFF_INCLUDE_UNTRACKED | GIT_DIFF_INCLUDE_TYPECHANGE |
                   GIT_DIFF_ENABLE_FAST_UNTRACKED_DIRS;
  int rv = git_diff_index_to_workdir(&diff, repo, NULL, &diffopts);
  if (rv == CHANGES_FOUND) {
    changes = " *";
  }

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

  printf(LONG_FORMAT, LONG_FORMAT_ARGS);
  return 0;
}
