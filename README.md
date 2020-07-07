# ICPC Statistic

ICPC Statistic is a set of APIs ought to build uniform and personal [ICPC](https://icpc.global/)-related information for everyone.

## Install
To build ICPC Statistic, you need to install Rust at first, see [official instruction](https://www.rust-lang.org/tools/install) for more details.

You need to install [Redis](https://redis.io/) in your device and open it at default port.

Now change `.env.example` to `.env` with your own settings.

Then excute following commands:

```shell
git clone https://github.com/NJUPT-SAST-Technology-Center/icpc_statistic.git
cd ./icpc-statistic
cargo run
```

It will run service at `localhost:8088`.

## API
Now support:

### /contest/incomming
Type: GET

Description: Return all incomming contests at Multiple OnlineJudges.