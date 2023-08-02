# far memory

Currently supports creating block device that connects to one remote memory storage server.

Performance is not good, but it works!

`hdparm -t /dev/nbd1`:
```
/dev/nbd1:
 Timing buffered disk reads:  20 MB in  3.34 seconds =   5.99 MB/sec
```

`ioping /dev/nbd1`:

```
4 KiB <<< /dev/nbd1 (block device 100 MiB): request=1 time=22.3 ms (warmup)
4 KiB <<< /dev/nbd1 (block device 100 MiB): request=2 time=22.6 ms
4 KiB <<< /dev/nbd1 (block device 100 MiB): request=3 time=33.6 ms
4 KiB <<< /dev/nbd1 (block device 100 MiB): request=4 time=34.2 ms
4 KiB <<< /dev/nbd1 (block device 100 MiB): request=5 time=21.1 ms
4 KiB <<< /dev/nbd1 (block device 100 MiB): request=6 time=21.4 ms
4 KiB <<< /dev/nbd1 (block device 100 MiB): request=7 time=36.5 ms
4 KiB <<< /dev/nbd1 (block device 100 MiB): request=8 time=34.4 ms
4 KiB <<< /dev/nbd1 (block device 100 MiB): request=9 time=33.8 ms
4 KiB <<< /dev/nbd1 (block device 100 MiB): request=10 time=33.8 ms
4 KiB <<< /dev/nbd1 (block device 100 MiB): request=11 time=34.3 ms
4 KiB <<< /dev/nbd1 (block device 100 MiB): request=12 time=35.6 ms
4 KiB <<< /dev/nbd1 (block device 100 MiB): request=13 time=33.6 ms
4 KiB <<< /dev/nbd1 (block device 100 MiB): request=14 time=22.2 ms
--- /dev/nbd1 (block device 100 MiB) ioping statistics ---
13 requests completed in 397.0 ms, 52 KiB read, 32 iops, 131.0 KiB/s
generated 14 requests in 13.8 s, 56 KiB, 1 iops, 4.07 KiB/s
min/avg/max/mdev = 21.1 ms / 30.5 ms / 36.5 ms / 5.88 ms
```