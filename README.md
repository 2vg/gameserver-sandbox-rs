# gameserver-sandbox-rs

This is a sandbox repository that implements a very small game server using [actix](https://github.com/actix/actix-web) and embedded DB [sled](https://github.com/spacejam/sled).

This was inspired by [Game Server in 150 lines of Rust](https://medium.com/@buterajay/game-server-in-150-lines-of-rust-ce1782199907).

It is also based on the DDD architecture.

Cuz a very simple implementation, an Entity has only a unique ID and position data.

However, cuz the DDD architecture,  so probably easily extensible.

## contribute

I am still new to Rust and DDD.

so, Any PR is welcome!

## Todo
- create index route
- put html file for View
- Process to actually CRUD the Entity, at this time GameServer does not process the Entity yet
- Refactor(I don't know if this is necessary, the initial goal is to create a small game server, not to write good code.)
