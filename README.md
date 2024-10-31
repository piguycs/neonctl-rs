# Neonctl-rs

> a blazingly fast cli to manage [neondb](https://neon.tech)

![benchmark_img](./assets/hyperfine.png)

## Usage

Counting features that are paywalled, I have implimented about 40% of the functionality.
This is enough for my use cases, but more will be added over time as I need.
```
$ neonctl-rs
Usage: neonctl-rs <COMMAND>

Commands:
  me
  projects
  branches
  connection-string
  regions            Display neondb regions and their ping from your machine
  psql               Connect to your database using psql
  help               Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## haiatus

I need to use neon for a project, and the free tier only allows one project. So
this has to go on a haiatus, as I wont be able to test it without affecting my
project. I might still add features to this, as I plan on using neonctl-rs instead
of the js version of neonctl, or I might decide that it is worth it to upgrade
to a higher tier just to continue working on an side-project.
