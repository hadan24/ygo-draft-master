# TO-DO
- set up sqlite to store cards
    - plan call stack
        - use `ResponseCard` for db responses too? but then 2 conversions every read
        - separate `YGOCard` constructors for outsiders like db to call + db fn's for "add card" etc
- start writing server

- low prio: find way to auto-check that all variants of ResponseCardName have tests