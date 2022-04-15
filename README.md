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

### TWEET DELETE PLAN
> Scenario: Assuming 2 users are selected 
<br /> - The Goal is to delete 960 reqs/day | limit - 1000reqs/day
> <br /> - 240 reqs/3hrs | limit - 300reqs/3hrs
> <br /> - 80 reqs/15mins i.e 40reqs/user | limit - 50reqs/15mins
> <br /> - i.e. (960/4)/3

<br />

> Scenario: Assuming 1 user is selected
<br /> - 40r/15mins | 50/15mins
<br /> - Run 7 times in 3hrs - (40*7 = 280r/3hrs) | 300/3hrs
<br /> - Run 3 times a day to delete (900 reqs/day) | 1000/day

```
def basic_del(num_of_users):
    times_to_run = []
    if num_of_users = 2:
        times_to_run = [0min, 15mins, 30mins]
    else:
        times_to_run = [0min, 15mins, 30mins, 45mins, 60mins, 75mins, 90mins]
    
    for time in times_to_run:
        run_at(time) --> 40 * num_of_users
```
> App has a maximum of 300 deletes/3hrs and 1000 deletes/24hrs
<!-- >> 4 hours later -->

```
t1 - 1:00hrs
basic_del() - 240reqs | total 240
t2 - 4:00hrs (rather than 3hrs, add 1 extra hr from the expected limit)
basic_del() - 240reqs | total 480
t3 - 8:00hrs
basic_del() - 240reqs | total 720
t4 - 12:00hrs
basic_del() - 70reqs | total 960
```


```
 basic_del()
```