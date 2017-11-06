# Bot Filter

Filter json events for `sessionID` by `ipAddress` given a list of CIDRs.

Installation:
```
wget https://github.com/colinbankier/bot-filter/raw/master/target/x86_64-unknown-linux-musl/release/bot-filter
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
