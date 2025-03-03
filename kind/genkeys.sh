#!/usr/bin/env sh

rm ssh_host_*

ssh-keygen -t ecdsa -C git-server -N "" -f ./ssh_host_ecdsa_key
ssh-keygen -t ed25519 -C git-server -N "" -f ./ssh_host_ed25519_key
ssh-keygen -t rsa -C git-server -N "" -f ./ssh_host_rsa_key

