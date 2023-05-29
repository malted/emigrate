# emigrate
A little tool to make sure you don't lose any progress on projects when wiping your machine.

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/ma1ted/emigrate/rust.yml)

## Usage
```bash
$ emigrate <PATH>
```

## What it tracks
- Unpushed commits
- Modified but uncommitted files
- Untracked files
- Loose files

## Performance
Approximately 99.75% of the time is spent on `wait4` syscalls waiting for `git` to finish running.
```bash
$ strace -c ./target/release/emigrate /home/malted/code > /dev/null 
```

## Example

![Tool screenshot](https://github.com/ma1ted/emigrate/assets/59726149/49f3181b-e3b6-4eec-b194-c2caf2fa3d25)
