# Parse Kubectl Time

This is a simple rust program that takes relative time strings in the format output by kubectl and parses them into seconds

For example:

2m10s -> 130
1d2h  -> 93600

I wrote this to use as part of a shell function that sorts my kubectl events by when they last occurred
