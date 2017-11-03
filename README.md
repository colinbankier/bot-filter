# Bot Filter

Filter json events for `sessionID` by `ipAddress` given a list of CIDRs.

Usage:
```
bot-filters [content_access_file] [cidr_list]
```
Where the output to stdout is the sessionIDs with IP matching at least one cidr in the list
