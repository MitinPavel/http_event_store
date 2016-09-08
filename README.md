Environment
===========

```
$ uname -a
Linux blah 4.2.0-42-generic #49-Ubuntu SMP Tue Jun 28 21:26:26 UTC 2016 x86_64 x86_64 x86_64 GNU/Linux

$ rustc --version
rustc 1.10.0 (cfcb716cf 2016-07-03)
```

Run test Event Store sever
==========================

```
cd ~/bin/EventStore-OSS-Ubuntu-14.04-v3.8.1
./run-node.sh --db ./ESData --log ./logs
```