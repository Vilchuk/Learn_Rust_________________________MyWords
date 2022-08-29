# MyWords

This is my project where I'm studying making an API part in Rust for my site, which helps to learn foreign words.

###### A total of 3 projects are planned:

-   [x] `In progress` API (Rust)
-   [ ] `Awaiting development` Telegram bot (Rust)
-   [ ] `Awaiting development` Client (ReactJs, maybe VueJs for variety)

###### API Stack:

-   **axum** - web application framework
-   **sqlx** - lib for working with sqlite db
    -   note: for working locally install SQLx CLI `cargo install sqlx-cli`

---

###### Working with db locally from scratch:

```
sqlx db create --database-url sqlite:mywords.db
sqlx migrate run --database-url sqlite:mywords.db
```

for creating new migration:

```
sqlx migrate add init
```
