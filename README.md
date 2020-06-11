# Usage Tracker ![GitHub Workflow Status](https://img.shields.io/github/workflow/status/TeFiLeDo/usage-tracker/Rust) ![GitHub](https://img.shields.io/github/license/TeFiLeDo/usage-tracker) ![GitHub last commit](https://img.shields.io/github/last-commit/TeFiLeDo/usage-tracker)
A simple usage tracker CLI written in rust. Also provides JSON output and a rust
library to easily access the data.

## What is this?
`usage-tracker` is a simple program that allows you to keep track of your usage
of _objects_.

For example, if you want to keep track of how much milk you drink, you'd tell
the program to keep track of a new object that you call "milk". After that,
whenever you've emptied a can of milk, you tell the program to record a new
usage. Later you can access a list of all times when you emptied a can of milk.

`usage-tracker` also provides the functionality to calculate an estimate of how
much cans of milk you'll need in a certain amount of time. Please note that
these estimates are _estimates_. In most cases the accuracy will increase with
the amount of data and the time since the usage first record.

## How to use?
In this section you'll learn how to enact the example from the previous section.

First of all, we need to tell `usage-tracker` to keep track of cans of milk:
```sh
$ usage-tracker add milk
```

Now we want to tell the program that we've emptied a can of milk:
```sh
$ usage-tracker use milk
```

After that, we want to see a list of all times we've emptied a can of milk:
```sh
$ usage-tracker show milk
```

Finally, we want to stop keeping track of milk:
```sh
$ usage-tracker remove milk
```

### Command reference
For further information, you can use the integrated help of the CLI:
```sh
$ usage-tracker help
$ usage-tracker -h
  # These commands will provide you with a brief help message.

$ usage-tracker --help
  # This command will provide you with a longer, more detailed help message.
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
You will need to set up a rust development environment. After that, clone or
download the repository. Go into its root folder and run this command:
```sh
$ cargo build
```
