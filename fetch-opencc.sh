#!/bin/bash

uname -a
pwd
ls -la
git branch -a
git config --list
git fetch origin main
git checkout main
git checkout .
ls -la
git log

