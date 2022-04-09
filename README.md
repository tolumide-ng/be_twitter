```
cargo run -p twitar
```

| # | Endpoints | Limit (app)  | Limit (user)   | Plan |
|---|-----------|--------|------|-------|
| 1 | Delete Retweet | (a). 50 requests per 15 min (per user) <br /> (b). 300 requests per 3-hour window(per user, per app) <br /> (c). 1000 successful requests/24hrs | | * |
| 2 | Delete Like | (a). 1000 successful requests/24hrs/window per user (this limit is shared by both like a tweet and delete like endpoints) | | |
| 3 | User Tweet Timeline | (a). 1500 requests per 15 minutes (per app) | (a). 900 requests/15 minutes/app | | * |
| 4 | Tweet counts | 300 requests/15m/app | | |
| 5 | Retweets lookup | 75r/15m/app | 75r/15m/app | | |
| 6 | Undo a retweet | 50r/15m/app | | | |