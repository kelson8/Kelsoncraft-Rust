#!/bin/bash

ENV_FILE=".env"

# Mostly from this Technotim guide
# https://technotim.com/posts/secret-encryption-sops/

# First, make sure age and sops are installed
# TODO Test this later.
# https://linuxize.com/post/bash-check-if-command-exists/
if ! command -v age >/dev/null 2>&1 || ! command -v sops >/dev/null 2>&1; then
  echo "Error, please install Age and Sops."

cat <<EOF
  Error, please install Age and Sops.
  Age: https://github.com/FiloSottile/age
  Sops: https://github.com/getsops/sops
EOF

fi

# Make sure values exist in .zshrc
# This won't validate the items, it'll only check the string.
if [ "$AGE_PUBLIC_KEY" = "" ] || [ "$AGE_KEY_FILE" = "" ] || [ "$SOPS_AGE_KEY_FILE" = "" ]; then

# https://linuxvox.com/blog/linux-cat-eof/
cat <<EOF
  Error, these values need set in either the .bashrc or .zshrc:
  Set this to your Age Public key directly: export AGE_PUBLIC_KEY=
  Set this to the path to your Age private key: export AGE_KEY_FILE=
  Set sops keyfile to previous value: export SOPS_AGE_KEY_FILE=\$AGE_KEY_FILE
EOF

exit 1

fi

# TODO Make this check if the env is already encrypted.
# Sops/Age seem to be able to tell when this is run again so it doesn't overwrite the .env

if [ -f "$ENV_FILE" ]; then
    sops --encrypt --age "$(cat $SOPS_AGE_KEY_FILE |grep -oP "public key: \K(.*)")" -i "$ENV_FILE"
fi
