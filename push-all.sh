#!/bin/bash

# This is a very basic bash script that I will use to push to my Git repos and kepe them in sync.
# It will help me keep the repos in sync since I have the internal Gitea, public Forgejo, and my GitHub instances setup with this.

# This will not be very useful unless you have a fork for the Git repos or something in multiple locations.

# Toggle for the script, by default this script is disabled.
PUSH_TO_REPOS=false

# This makes sure that nothing happens if this script is disabled.
if [ "$PUSH_TO_REPOS" = false ]; then
  echo "Pushing to repos has been disabled, no changes has been made."
  exit 0
fi

# Internal git repo
git push main
# https://github.com/kelson8/Kelsoncraft-Rust
git push github
# https://git.kelsoncraft.net/kelson8/Kelsoncraft-Rust
git push origin
