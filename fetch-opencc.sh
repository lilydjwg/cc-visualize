#!/bin/bash

#uname -a
#pwd
#ls -la
#git branch -a
#git config --list
git fetch origin main
git checkout main
git checkout .
git branch -a
#ls -la
#git log
sha1sum opencc-data/JPVariants.txt
wget https://raw.githubusercontent.com/BYVoid/OpenCC/master/data/dictionary/JPVariants.txt -O opencc-data/JPVariants.txt
#curl https://raw.githubusercontent.com/BYVoid/OpenCC/master/data/dictionary/JPVariants.txt > opencc-data/JPVariants.txt


git config user.name bot
git config user.email bot@github.bot.none

sha1sum opencc-data/JPVariants.txt
git add opencc-data/
git show
git commit -m 'sync opencc'
git log
