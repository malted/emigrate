# emigrate
A little tool to make sure you don't lose any progress on projects when wiping your machine.

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/ma1ted/emigrate/rust.yml)

## Usage
```bash
$ emigrate <DIR_PATH>
```

## What it tracks
- Unpushed commits
- Modified but uncommitted files
- Untracked files
- Loose files

## Example

![Tool screenshot](https://github.com/ma1ted/emigrate/assets/59726149/49f3181b-e3b6-4eec-b194-c2caf2fa3d25)

## Performance
Approximately 99.85% of the time is spent on `wait4` syscalls waiting for `git` to finish running.
```bash
$ strace -c ./target/release/emigrate /home/malted/code > /dev/null
% time     seconds  usecs/call     calls    errors syscall
------ ----------- ----------- --------- --------- ------------------
 99.85    1.753982       31890        55           wait4
  0.04    0.000624          11        55           clone3
  0.02    0.000320           1       280           close
  0.02    0.000292           0       332           prlimit64
  0.01    0.000249           0       251         5 read
  0.01    0.000233           2       110           pipe2
  0.01    0.000148           0       165           ioctl
  0.01    0.000130           2        60           openat
  0.01    0.000126           2        57           munmap
  0.01    0.000111           1        70           statx
  0.01    0.000102           1        61           poll
  0.01    0.000096           0       110           rt_sigprocmask
  0.00    0.000087           1        68           mmap
  0.00    0.000080           1        72           write
  0.00    0.000051           1        35           clock_gettime
  0.00    0.000002           1         2           getdents64
  0.00    0.000001           0         3           sigaltstack
  0.00    0.000000           0         5           mprotect
  0.00    0.000000           0         5           brk
  0.00    0.000000           0         5           rt_sigaction
  0.00    0.000000           0         2           pread64
  0.00    0.000000           0         1         1 access
  0.00    0.000000           0         1           execve
  0.00    0.000000           0         2         1 arch_prctl
  0.00    0.000000           0         1           sched_getaffinity
  0.00    0.000000           0         1           set_tid_address
  0.00    0.000000           0         5           newfstatat
  0.00    0.000000           0         1           set_robust_list
  0.00    0.000000           0         2           getrandom
  0.00    0.000000           0         1           rseq
------ ----------- ----------- --------- --------- ------------------
100.00    1.756634         966      1818         7 total
```
