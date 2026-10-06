# wat_planner

Cli toolbox for working with WAT schedules

> [!WARNING]
> `wat_planner` is still in beta so it is recommended to cross-check its output with the original schedule.

## Features

- parsing WAT schedules
- generating weekly schedules from [TERA](https://keats.github.io/tera/) template files
- builtin HTML template
- caching parsed schedules

## Installation

For Nix users this repo provides a flake which you can include in your configuration or test it out with `nix shell`.

```sh
$ nix shell github:DiaxuDev/wat_planner
```

If you want to compile it yourself, see [Building](#building).

## Usage

> [!TIP]
> Every command has a `--help` option that explains all parameters

To generate schedule for the current week using the builtin HTML template simply run

```sh
$ wat_planner generate GROUP # replace GROUP with your WAT group identifier
```

This will generate a `schedule.html` file in your current working directory wchich you can open using any browser. If you wish to generate schedule for different week you can pass the desired date like so

```sh
$ wat_planner generate GROUP 2026-12-24 # replace GROUP with your WAT group identifier
```

You can also use `--template` to use different TERA template than the builtin one. For details, see the section on [Templates](#templates).

If at any point your schedule recieved modifications you can force `wat_planner` to renew its cache by running

```sh
$ wat_planner fetch --force GROUP # replace GROUP with your WAT group identifier
```

## Templates

`wat_planner` uses [TERA](https://keats.github.io/tera/) as its templating system. To generate schedule file with a custom template you can pass `--template` to `wat_planner generate` like so

```sh
$ wat_planner generate --template mytemplate.txt GROUP # replace GROUP with your WAT group identifier
```

This will create file named `schedule.txt` (file extension is automatically detected based on the provided template filename) containing the generated schedule.

Template files are expanded with `days` variable containing an array of `day` objects. The structure of a day object looks like follows:

- `weekday` - string containing the name of the current day
- `date` - string with the date of the current day formatted as `dd.mm`
- `classes` - array of either `null` or `class` object

`class` objects have the following structure:

- `hour` - string containing start and end hour of the curent class formatted as `HH:MM - HH:MM`
- `name` - string with the name of the subject
- `kind` - string containing the type of the class
- `color` - hex formatted string containing the color of the subject
- `professors` - array of strings containing potential professors teaching this class
- `info` - array of strings with additional information about the class, usually the room numer

If you want to see an example usage of these variables take a look at how the default template is implemented [here](./include/template.html).

## Building

For Nix users there is a provided devshell which you can use, either with nix-direnv by running `direnv allow` or manually with `nix develop`.

If you don't want to use the devshell you will need rust and cargo installed on your system. To build `wat_planner` locally one must simply run

```sh
$ cargo build
```
