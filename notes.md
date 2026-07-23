https://github.com/RicoSuter/NSwag

# Frontend
- top bar
    - home/rules
    - draft
    - cubes
    - decks
    - login

- footer bar
    - (c), link to github, link to creator github
    - bug report link to new github issue
    - thanks/credits to farfa for intro to cube, friends for ideas, ygoprodeck api

- home/rules
    - welcome
    - resume explanation
    - "go to `Draft` now for HS Arena style, choose from `Cubes` otherwise"
    - screenshot

- draft
    - draft area
        - variable num of cards (up to 9? 13?)
        - show card imgs side by side
            - hover for pop-up w/ name, attr, type line, eff, stats
            - click to add to deck
        - once done, download ydk, link to DB/Omega/Edo
            - add `!side` to last line
    - deck panel
        - collapsible + resizeable
            - up to 30-40% of screen
        - [attr] [name] [type] (bg based on card type)
        - 2 sub-headers/sections: main + extra
    - stats panel
        - below draft area
        - pie charts
            - monster/spell/trap
            - attrs
            - fusion/synch/xyz/link
        - bar charts
            - subtypes (tuners, unions, etc)
            - main deck monster levels (1-4, 5+ or 5-6 & 7+)

- cubes
    - dynamic (fill page horizontally, scroll for vert, many pgs?)
    - first "card" is creation button
    - cube creation
        - fields for name, card name art, description
        - search field for cards to add
            - fuzzy search
            - scroll menu for results
            - click card in menu to add
        - card gallery
            - show max of 15 cards (img thumb) per row (centered)
            - hover for pop-up w/ name, attr, type line, eff
            - click on card to remove

    - cube card
        - title (name, creator, num stars/favs)
        - transparent black top half for readable title
        - card art bg
        - rounded rect (& drop shadow??)
        - links to page to view full cube

        | name          stars   |
        | creator               |
        -------------------------
        |       [card art]      |
        |                       |

    - cube page
        - title card
            - name, creator, stars
            - transparent over full card for readability
            - card art bg
        - description blurb
            - char/word lim (100 words?, 300 chars?)
        - card gallery
            - show max of 15 cards (img thumb) per row (centered)
            - hover for pop-up w/ name, attr, type line, eff

- decks
    - if not logged in, show login/signup
    - dynamic (fill page horizontally, scroll for vert, many pgs?)
    - deck card
        - title (name, associated cube, date built)
        - transparent black top half for readable title
        - cube card art bg
        - rounded rect (& drop shadow??)
        - links to page to view full deck

        | name          date    |
        | cube                  |
        -------------------------
        |       [card art]      |
        |                       |

    - deck page
        - title card
            - name, associated cube, date built
            - transparent over full card for readability
            - cube card art bg
            - cube name links to cube page
        - card gallery
            - show max of X cards (img thumb) per row (centered)
                - X :=  40      -> 10 cards in 4 rows
                        41-50   -> 10 cards in 5 rows
                        51-55   -> 11 cards in 5 rows
                        56-60   -> 12 cards in 5 rows
            - hover for pop-up w/ name, attr, type line, eff

- login
    - screen name, email, pw

# Backend
- draft (url/draft?cube=#&session=#)
    - frontend sends user id, choice made
    - set up socket
    - map session id to in-mem editable copy of pool
        - server manages how player selections affect pool (rm from future choices?)

    - user chooses card
    - server rm's from pool if necessary
    - server returns new set to pick from
        - frontend handles closing connection on finish (https://websocket.org/guides/frameworks/react/)

- cube (url/cubes?id=#)
    - get cube from db
    - send json list to front for display

- deck (url/decks?id=#)
    - get deck from db
    - send json list to front for display

- login/auth
    - https://blog.0xshadow.dev/posts/backend-engineering-with-axum/axum-reset-password/
    - https://systemdesign.tech/post/how-to-securely-store-passwords-in-a-database-a-comprehensive-guide


# DB
- SQLite until data >1tb
    - https://docs.rs/sqlx/latest/sqlx/
    - https://www.sqlite.org/lang_keywords.html
    - https://youtu.be/FW4oUXHly8c?si=oeaI1BZBLZcxH0ik  // lgr's "must know" db libs
    - https://youtu.be/bQWvoMwzQN4?si=AdL0nd31yY5pU-wF
    - https://youtu.be/TCERYbgvbq0?si=s7kpK6jvdkpM-vqF  // basic sqlx tutorials
- 5/22/2026
    - [~14372] cards, [~2.6] KB per card in json (used Endymion as worst case)
        - Jul 9 2026 -> [13903] cards (including Skills), [~15.1] MB
    - = [~37] MB db/json data, [~2.5] GB for imgs

- card data (big json file??)
    - save ygopro db ver and/or timestamp (https://db.ygoprodeck.com/api/v7/checkDBVer.php)
    - check on startup & 1 wk??, update if different

    - id (u32 4byte), name (text 60char)
    - frametype (text 20char), race/monster type (text 15char), attribute (enum-u3 | text 6char)
    - level (u4 1nib), atk (u16 2byte), def (u16 2byte)
    - path/to/imgs (text 100char)
- cube data
    - id (u32 4byte | u64 8byte), name (text 60char), creator (text 60char)
    - stars (u16 2byte), path/to/art (text 100char), finite? (u1)
    - list[card id] (separate table)
- deck lists
    - id (u32 4byte | u64 8byte), name (text 60char), creator (text 60char),
    - date (text 10char | 20char), cube id (u32)
    - list[card id] (separate table)
- user data
    - email (text 30char), screen name (text 60char)