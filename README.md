# bottomtree

git subtree's UX low-key sucks but we have to use it for Rust project.

This is an exploration of whether we can get the same results through other means.

## Experiment 1

I did a fresh clone of `rust-analyzer` into `rust-analyzer-gfr/` directory

Then I ran `git filter-repo --to-subdirectory-filter src/tools/rust-analyzer`.
This rewrites whole history, but it's pretty fast (few seconds).

Then I added this directory as a "path remote" to my `rust/` checkout:

```
git add remote for-sync ../rust-analyzer-gfr
```

I looked at git history on both sides to find the rejoin point `rust/` already
had history for `rust-analyzer` up until commit `43601dd72103f007bec5f35670f54d462ee1ae28`

(Note: that's a "post-rewrite" hash - it doesn't exist in the remote
`rust-analyzer` repo. I'm not sure how to automate this step yet)

I used `git show` to print all commit hashes from that point until the latest
commit, and reversed them with tac:

```
git show --pretty=format:"%H " --no-patch HEAD...43601dd72103f007bec5f35670f54d462ee1ae28 | tac > /tmp/commits.txt
```

I used the code in `src/main.rs` to filter out merge commits, by running `git
show --no-patch --format="%P" <hash>` for every commit hash in order, and
skipping them if they had more than 1 parent.

I stored the resulting list in `/tmp/non-merge-commits`.

Then, from a new branch of `rust/`, I ran:

```
while read i; do git cherry-pick $i; done < /tmp/non-merge-commits
```

And pushed it to my `rust` fork. Here's the result:

  * https://github.com/fasterthanlime/rust/pull/1
