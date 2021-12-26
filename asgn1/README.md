# Pass the Pigs

This program is a self-playing implementation of pass the pigs, a game of chance in which up to 10 players stand in a
circle rolling a pig, gaining points until one of them reaches 100 points. Each player may roll the pig until they land
on its side. Point values for each side are as follows

| Position   |   Points |
|:-----------|---------:|
| `SIDE`     | End turn |
| `BACK`     |   10 pts |
| `UPRIGHT`  |   10 pts |
| `SNOUT`    |   15 pts |
| `EARS`     |    5 pts |

## Building and running
This is a standard Cargo project, so `cargo build` and `cargo run` will suffice.