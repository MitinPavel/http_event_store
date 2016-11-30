Environment
===========

```
$ uname -a
Linux blah 4.4.0-47-generic #68-Ubuntu SMP Wed Oct 26 19:39:52 UTC 2016 x86_64 x86_64 x86_64 GNU/Linux

$ rustc --version
rustc 1.13.0 (2c6933acc 2016-11-07)
```

Run test Event Store sever
==========================

```
cd ~/bin/EventStore-OSS-Ubuntu-14.04-v3.8.1
./run-node.sh --db ./ESData --log ./logs
```
