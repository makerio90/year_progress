# this is year_progress, the cool project made by makerio
shows the progress of this year.
# example 
```sh
$ year_progress
[==============>                                   ] 29.858168865590002%
$ year_progress -h
[==============>                                   ]
$ year_progress -t 60
[=================>                                          ] 29.858365713489977%
$ year_progress -s "2021-8-26 0:0:0" -e "2022-6-16 12:59:59" -t 10 -l
[========> ] 80.41234342482606%
```
## installation instructions
simply run `cargo install --git www.github.com/makerio/year_progress`\
make sure ~/.cargo/bin/ is in your $PATH
