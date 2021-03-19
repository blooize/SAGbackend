# SAGbackend
Backend for the Space Avionics Giessen Website

---
- First you need to have Postgresql installed

- Second you'll need to specify your Postgres Database URL like this in the ```.env``` file:
```
DATABASE_URL=postgres://{USERNAME}:{USER PASSWORD}@{THE DB IP ADRESS i.e localhost}/{YOU DB NAME}

ROCKET_ADDRESS=127.0.0.1 // IP of the Rocket Launch Pad

ROCKET_PORT=3000 // Port on which the Rocket launches
```

- Third you need to execute this command for dependency fetching, compiling and running:
```shell
$ cargo run
```

If you find any problems with my code (of which you'll find many :wink:) feel free to open up an Issue or PR, 
if you feel like it, you can DM me on Discord ```bleuser#0001``` or on Twitter ```@blooizer```

:wave: 