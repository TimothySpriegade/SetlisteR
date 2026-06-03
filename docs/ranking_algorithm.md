# SetlisteR — Song Ranking Algorithm

How `SetlistDataReducer` assigns a role to each song and orders the final playlist.

## Role Assignment

```mermaid
flowchart TD
    START(["song: SongStats"])
    CHECK_PLAYS{"total_plays\n≥ 3?"}
    REG1(["RoleBucket::Regular\n(too few data points)"])

    CALC["Calculate rates:\nopener_rate  = opener_count  / total_plays\ncloser_rate  = closer_count  / total_plays\nencore_rate  = encore_count  / total_plays"]

    PICK{"Dominant role\n(highest rate)"}
    OPENER_CAND["candidate: Opener"]
    CLOSER_CAND["candidate: Closer"]
    ENCORE_CAND["candidate: Encore"]

    THRESH{"rate ≥ 60%\n(EXCEPTIONAL_ROLE_RATE)?"}
    ASSIGN_ROLE(["Assign candidate role\nOpener / Closer / Encore"])
    REG2(["RoleBucket::Regular\n(rate below threshold)"])

    START --> CHECK_PLAYS
    CHECK_PLAYS -- no --> REG1
    CHECK_PLAYS -- yes --> CALC
    CALC --> PICK
    PICK -- "opener_rate ≥ closer_rate\nAND opener_rate ≥ encore_rate" --> OPENER_CAND
    PICK -- "closer_rate ≥ encore_rate" --> CLOSER_CAND
    PICK -- otherwise --> ENCORE_CAND
    OPENER_CAND --> THRESH
    CLOSER_CAND --> THRESH
    ENCORE_CAND --> THRESH
    THRESH -- yes --> ASSIGN_ROLE
    THRESH -- no --> REG2
```

## Playlist Ordering (within a single artist)

```mermaid
flowchart LR
    subgraph Phase1["Phase 1 — Selection"]
        SEL["Sort all songs by total_plays DESC\nthen by setlist_order (role + mean_pos)\nthen alphabetically\nTake top N = ceil(avg_setlist_length) + 3"]
    end

    subgraph Phase2["Phase 2 — Ranking (sort key priority)"]
        direction TB
        B1["① Role Bucket\n Opener &lt; Regular &lt; Closer &lt; Encore\n(ascending enum order)"]
        B2["② Within Opener\nhighest opener_rate first\nthen lowest mean_position first"]
        B3["③ Within Regular\nlowest mean_position first"]
        B4["④ Within Closer\nhighest closer_rate first\nthen highest mean_position first"]
        B5["⑤ Within Encore\nhighest encore_rate first\nthen highest mean_position first"]
        B6["⑥ Tiebreaker\nhighest total_plays\nthen alphabetical"]
    end

    subgraph Result["Result"]
        OUT["BTreeMap&lt;position, song_name&gt;\nPosition 1 = Opener\nPositions 2..N-2 = Regular songs\nPosition N-1 = Closer\nPosition N = Encore"]
    end

    Phase1 --> Phase2 --> Result
```

## Concurrency Model

```mermaid
flowchart TB
    MAIN["main() — tokio::main"]

    subgraph Async["Async Layer (Tokio)"]
        JS["JoinSet — one task per artist"]
        TASK1["async task: Artist A\nfetch pages 1..N"]
        TASK2["async task: Artist B\nfetch pages 1..N"]
        TASKN["async task: Artist N\nfetch pages 1..N"]
        MUTEX["Arc&lt;Mutex&lt;Instant&gt;&gt;\nglobal rate-limit slot\n(500 ms between requests)"]
    end

    subgraph Blocking["CPU-bound Layer (Rayon par_iter)"]
        PAR1["thread: process Artist A setlists"]
        PAR2["thread: process Artist B setlists"]
        PARN["thread: process Artist N setlists"]
        REDUCE_BLOCK["spawn_blocking → Rayon par_iter\nreduce each artist playlist"]
    end

    MAIN --> JS
    JS --> TASK1 & TASK2 & TASKN
    TASK1 & TASK2 & TASKN <-->|"wait for slot"| MUTEX
    TASK1 --> PAR1
    TASK2 --> PAR2
    TASKN --> PARN
    PAR1 & PAR2 & PARN --> REDUCE_BLOCK
    REDUCE_BLOCK --> MAIN
```
