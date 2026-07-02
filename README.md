# TO-DO
- test YGOCard constructors
    - fix Han-Shi constructor
        - not properly detecting that this is pendulum
        - given "type" field is just "Spirit Monster", what i expected is found in "humanReadableCardType" instead
    https://db.ygoprodeck.com/api/v7/cardinfo.php?name=Han-Shi%20Kyudo%20Spirit
- fix pend YGOCard constructor for Majespecters
    https://db.ygoprodeck.com/api/v7/cardinfo.php?name=Igknight%20Squire
    https://db.ygoprodeck.com/api/v7/cardinfo.php?name=Majespecter%20Crow%20-%20Yata