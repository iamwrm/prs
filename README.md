# prs

ps, but with sql syntax


## Examples

```bash
cargo r --release -- -p top10-mem
```

```bash
Query: SELECT * FROM processes ORDER BY vmrss DESC LIMIT 10
----------------
pid, name, uid, vmrss, user, num_threads
102448, rust-analyzer, 1000, 1364740, ubuntu, 12
142185, rust-analyzer, 1000, 792668, ubuntu, 8
102324, node, 1000, 275556, ubuntu, 13
1743, influxd, 1000, 196180, ubuntu, 15
142086, node, 1000, 171748, ubuntu, 13
142760, node, 1000, 159864, ubuntu, 11
103027, node, 1000, 149668, ubuntu, 11
1574, mongod, 999, 121172, lxd, 33
142771, node, 1000, 118772, ubuntu, 8
102439, node, 1000, 116620, ubuntu, 8

```

```bash
cargo r --release -- -s "SELECT user, SUM(vmrss) FROM processes GROUP BY user ORDER BY SUM(vmrss) DESC"
```

```bash
Query: SELECT user, SUM(vmrss) FROM processes GROUP BY user ORDER BY SUM(vmrss) DESC
----------------
user, SUM(vmrss)
ubuntu, 3877620
root, 932344
unknown, 175868
lxd, 121172
systemd-timesync, 13548
messagebus, 7884
landscape, 5876
_apt, 4712
nobody, 4108
systemd-resolve, 756
```