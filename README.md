# play_twitter


Big Credit to [Egg-mode](https://github.com/egg-mode-rs/egg-mode) as a lot of my implementations was inspired by their design

TODOS!
- [ ] All current redis implementations to be migrated Postgres
- [ ] Only Save post ids on redis
- [ ] Only save jwt tokens for local users on redis
- [ ] Migrate route management to routerify
- [ ] Refresh token after every 1hrs 45mins
- [ ] Migrate redis use cases to Postgres
- [x] User lookup
- [x] Tracked actions in the history table (if applicable)
- [ ] Implement app authentication and authorization
- [ ] Separate v2 controllers into different folder
- [x] Send arguments to route controllers as a struct containing http_client, db_client, env_vars (we don't have to call SettingsVars::new() in each controller everytime)
- [ ] Test scenarios where user doesn't have tweets, rts, and likes (how does get_timeline react in these cases)
- [ ] Migrate to Cargo workspaces
- [ ] Add tests CI and code-coverage
- [ ] Abstract the sqlx calls into reusable methods (macros?)