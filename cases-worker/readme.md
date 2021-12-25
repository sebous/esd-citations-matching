## Process

1) defined set of rules - ordered by speed/efficiency
2) iterate over each input file
3) parse file, split to words
4) try each rule in their order
   1) log if match
   2) insert match info to DB
   3) rules can log "interesting" mismatches
