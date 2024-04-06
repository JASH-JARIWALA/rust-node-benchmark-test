## Node
```
./bombardier -m GET -c 100 -d 50s -r 100 -t 15s -l http://localhost:3000/bubble
Bombarding http://localhost:3000/bubble for 50s using 100 connection(s)
[=================================================================================================================] 50s
Done!
Statistics        Avg      Stdev        Max
  Reqs/sec        12.16      19.78     209.68
  Latency         8.13s     16.55s      1.07m
  Latency Distribution
     50%      1.66s
     75%      1.68s
     90%     41.21s
     95%      0.94m
     99%      1.00m
  HTTP codes:
    1xx - 0, 2xx - 629, 3xx - 0, 4xx - 0, 5xx - 0
    others - 73
  Errors:
    dial tcp [::1]:3000: connect: operation timed out - 68
    the server closed connection before returning the first response byte. Make sure the server returns 'Connection: close' response header before closing the connection - 5
  Throughput:     2.99KB/
```

## Rust
```
./bombardier -m GET -c 100 -d 50s -r 100 -t 15s -l http://localhost:4000/bubble
Bombarding http://localhost:4000/bubble for 50s using 100 connection(s)
[========================================================================================================================] 50s
Done!
Statistics        Avg      Stdev        Max
  Reqs/sec        17.33      89.21    3167.31
  Latency         6.02s      3.66s     18.37s
  Latency Distribution
     50%      5.75s
     75%      8.37s
     90%     10.78s
     95%     13.09s
     99%     15.99s
  HTTP codes:
    1xx - 0, 2xx - 598, 3xx - 0, 4xx - 287, 5xx - 0
    others - 1
  Errors:
    the server closed connection before returning the first response byte. Make sure the server returns 'Connection: close' response header before closing the connection - 1
  Throughput:     3.25KB/s
```