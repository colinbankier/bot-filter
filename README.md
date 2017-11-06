# Bot Filter

Filter tsv events for `sessionID` by `ipAddress` given a list of CIDRs.
e.g.
```
0	1.1.1.1
1	192.0.1.1
2	10.1.1.1
```

Installation:
```
wget https://github.com/colinbankier/bot-filter/raw/treebitmap/target/x86_64-unknown-linux-musl/release/bot-filter
chmod +x bot-filter
```

Usage:
```
./bot-filter [content_access_file] [cidr_list]
```
Where the output to stdout is the sessionIDs with IP matching at least one cidr in the list

### compile static binary
Assumes toolchain installed via rustup.
```
cargo build --release --target=x86_64-unknown-linux-musl
```
