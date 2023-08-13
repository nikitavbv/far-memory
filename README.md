# far memory

Currently supports creating block device that connects to one remote memory storage server.

Performance is not good, but it works!

`hdparm -t /dev/nbd1`:
```
/dev/nbd1:
 Timing buffered disk reads: 100 MB in  1.22 seconds =  82.08 MB/sec
```

`ioping /dev/nbd1`:

```
4 KiB <<< /dev/nbd1 (block device 100 MiB): request=1 time=21.0 ms (warmup)
4 KiB <<< /dev/nbd1 (block device 100 MiB): request=2 time=20.1 ms
4 KiB <<< /dev/nbd1 (block device 100 MiB): request=3 time=21.6 ms
4 KiB <<< /dev/nbd1 (block device 100 MiB): request=4 time=20.7 ms
4 KiB <<< /dev/nbd1 (block device 100 MiB): request=5 time=20.8 ms
4 KiB <<< /dev/nbd1 (block device 100 MiB): request=6 time=22.4 ms
4 KiB <<< /dev/nbd1 (block device 100 MiB): request=7 time=20.0 ms
4 KiB <<< /dev/nbd1 (block device 100 MiB): request=8 time=22.0 ms
4 KiB <<< /dev/nbd1 (block device 100 MiB): request=9 time=21.8 ms
4 KiB <<< /dev/nbd1 (block device 100 MiB): request=10 time=21.4 ms
--- /dev/nbd1 (block device 100 MiB) ioping statistics ---
9 requests completed in 190.8 ms, 36 KiB read, 47 iops, 188.7 KiB/s
generated 10 requests in 9.39 s, 40 KiB, 1 iops, 4.26 KiB/s
min/avg/max/mdev = 20.0 ms / 21.2 ms / 22.4 ms / 791.0 us
```