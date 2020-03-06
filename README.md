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
- ~~Process to actually CRUD the Entity, at this time GameServer does not process the Entity yet~~ done
- Refactor(I don't know if this is necessary, the initial goal is to create a small game server, not to write good code.)

## Simple specifications

This game server can only handle very simple entities.

Entity holds only `id: u32` and `pos: (i32, i32)`.

The `id` is randomly generated and shared as the ws session ID and Entity unique ID.

pos is the position data of the Entity, and the client draws another Entity based on `pos`.


### # when client connected

When the client connects to the game server, the game server creates a new Entity with `pos: (0, 0)`.

Next, register the `id` in the game server session list and send the entity data to all other clients.

### # when entity updated

When the client moves in the game, send the json of `{x: X coordinate, y: Y coordinate}` to the game server.

The game server updates the Entity position based on the received json and sends the updated Entity data to all other clients.

### # when client disconected

When a client disconnects from game server, the game server removes the client's `id` from the session and send all other clients a `{id: client.id, x: -1, y: -1}` json.

The client must implement from this json to stop drawing the entity of the disconnected client.
