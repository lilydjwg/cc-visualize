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
sha1sum opencc-data/TSCharacters.txt
#wget https://raw.githubusercontent.com/BYVoid/OpenCC/master/data/dictionary/TSCharacters.txt -O opencc-data/TSCharacters.txt
curl https://raw.githubusercontent.com/BYVoid/OpenCC/master/data/dictionary/TSCharacters.txt > opencc-data/TSCharacters.txt


git config user.name bot
git config user.email bot@github.bot.none

sha1sum opencc-data/TSCharacters.txt
git add opencc-data/
git show
git commit -m 'sync opencc'
git log
