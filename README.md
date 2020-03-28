# Usage Tracker [![Crates.io](https://img.shields.io/crates/l/usage-tracker)](https://crates.io/crates/usage-tracker) [![Build Status](https://drone.tfld.dev/api/badges/tfld/usage-tracker/status.svg)](https://drone.tfld.dev/tfld/usage-tracker)

A simple usage tracker in rust.

## What is this?
This program allows you to keep track on your usage of _things_.

For example, if you want to keep track of how much milk you need, you'd tell the
program to keep track of milk. Whenever you have emptied a can of milk, you tell
it to record a new usage. Later you can see a list of all times when you emptied
a can.

The program can also provide you with an estimate of how much milk you'll need
for a certain amount of time. Please note that these estimates are only a rough
guess and get better with the amount of data provided.

## How to use?
To start tracking a new _thing_:
```sh
$ usage-tracker add thing
```

To get a list of tracked things:
```sh
$ usage-tracker list
```

To record a new usage of _thing_:
```sh
$ usage-tracker use thing
```

To get an estimate on how much instances of _thing_ you need for a given time:
```sh
$ usage-tracker need thing 1 y # 1 year
$ usage-tracker need thing 1 M # 1 month
$ usage-tracker need thing 1 d # 1 day
$ usage-tracker need thing 1 h # 1 hour
$ usage-tracker need thing 1 m # 1 minute
$ usage-tracker need thing 1 s # 1 second
$ usage-tracker need thing 1 w # 1 week
```

To stop tracking a _thing_:
```sh
$ usage-tracker remove thing
```

To stop tracking all things:
```sh
$ usage-tracker clear
```

## How to install?
If you have _cargo_ installed (which probably means your a rust developer), just
type this:
```sh
$ cargo install usage-tracker
```

Otherwise you can go to the [releases](https://github.com/TeFiLeDo/usage-tracker/releases)
page and grab the application for your platform from the latest release. Make
sure to grab the version for the correct platform.

## Details
In the prediction functionality, the existence of leap years is ignored. Also
all months are treated as 30 days long. This is necessary to keep the interface
for users simple. Otherwise they would be required to specify when the usage
starts, which would be a worse user experience.

## How to build from source?
You will need to set up a rust development environment. After that, clone the
repository. Go into the root folder of the repository and run this:
```sh
$ cargo build
```
