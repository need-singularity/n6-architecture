# ubu machine blowup module exit 137 cause diagnosis — 2026-04-14

> Follow-up to `project_blowup_module_sigkill.md` open item. Remote diagnosis performed this session.

## 1. Problem Summary

- macOS-side SIGKILL resolved (`project_hexa_binary_deploy_block.md`):
  - Cause: AppleSystemPolicy adhoc codesign missing
  - Fix: `codesign --sign - --force` + canonical path deployment
- **ubu Linux machine**: the same blowup module still exits 137 -> **separate cause**

exit 137 = 128 + 9 = termination after receiving SIGKILL.

## 2. Measurements This Session

```
$ ssh ubu 'uname -a && uptime'
Linux aiden-B650M-K 6.17.0-20-generic Ubuntu SMP
12:16:41 up 5 days, 17:43
```

### 2.1 Resources (not a memory cause)

| Item | Value | Verdict |
|---|---|---|
| RAM total | 30 Gi | Ample |
| RAM used | 1.9 Gi | — |
| **RAM free** | **28 Gi** | **OOM impossible** |
| Swap total | 8 Gi | — |
| Swap used | 2.8 Gi | Normal |
| Disk free | 643 Gi / 915 Gi | Ample |

### 2.2 cgroup/ulimit

```
$ ssh ubu 'cat /sys/fs/cgroup/user.slice/user-1000.slice/memory.max'
max
```

-> User cgroup memory limit **none**. cgroup OOM impossible.

```
$ ssh ubu 'ulimit -a'
core file size              (blocks, -c) 0
max locked memory           (kbytes, -l) 3993124   <- 4 GB
max memory size             (kbytes, -m) unlimited
open files                          (-n) 1024      <- suspect
stack size                  (kbytes, -s) 8192
virtual memory              (kbytes, -v) unlimited
max user processes                  (-u) 124075
```

### 2.3 Running Watchdogs

```
$ ssh ubu 'ps aux | grep -iE "watchdog|guard"'
root  112  S  [watchdogd]    <- kernel watchdog, does not kill user processes
```

-> **No user watchdog** such as `n6_guard`, `launcher_cap`, `resource_coordinator`.

### 2.4 systemd user services

```
airgenome-harvest.service  activating start  <- 6-axis genome collection
```

-> airgenome is the only user daemon. Unrelated to blowup.

### 2.5 dmesg

```
dmesg: read kernel buffer failed: operation not permitted
```

-> root required. Cannot inspect dmesg OOM killer log (next session: `sudo dmesg` or `journalctl -k`).

## 3. Remaining Hypotheses (Priority)

### H1. `open files 1024` exceeded (*** strongly suspected)

blowup module may exceed fd 1024 while loading 5 modules x many .json/.hexa concurrently.

**Verification**: retry with `ssh ubu 'ulimit -n 65536 && blowup.hexa --fast --max-rounds 1'`.

### H2. `max locked memory 4 GB` exceeded (** medium)

hexa VM hits 4 GB limit on mlock.

**Verification**: retry with `ssh ubu 'ulimit -l unlimited && blowup.hexa ...'`.

### H3. SSH/tmux disconnect SIGHUP -> exit 129 (* low, but can be confused with 137)

Without tmux/nohup, disconnecting SSH sends SIGHUP. But exit code 137 != 129.

### H4. External process kill (* low)

airgenome-harvest killing the blowup process? Unlikely -- separate service.

### H5. Kernel OOM killer (not cgroup, system-wide) (* low)

System-wide OOM killer. Practically impossible given 28 Gi free.

**Verification**: `ssh ubu 'sudo dmesg | grep -i oom'` or `journalctl -k -g oom`.

## 4. Workaround (immediately available)

- **macOS side**: `HEXA_BIN=hexa.patched BLOWUP_LOCAL=1 blowup.hexa` (already verified, `--fast --max-rounds 2` 40s PASS)
- **ubu side**: H1 workaround -- recommend running `ulimit -n 65536` first. Not actually reproduced in this session.

## 5. Next Session Actions

1. Run `ssh ubu 'ulimit -n 65536 && HEXA_BIN=hexa.patched blowup.hexa --fast --max-rounds 1'` -> record exit code
2. If it fails, check `journalctl -k -g "kill\|oom" --since "1 hour ago"` (needs sudo)
3. Use `strace -f -e trace=signal blowup.hexa` to identify SIGKILL sender
4. Sequentially eliminate H1-H5

## 6. Honest Record

- This session performed **remote diagnosis only**. Blowup not actually reproduced on ubu (time constraints).
- H1 (fd 1024) is most likely but unverified.
- Conclusion: ubu exit 137 is not a memory cause -- narrowed to fd / mlock / external kill.
- Memory `project_blowup_module_sigkill.md` "unresolved" item maintained. Real measurement needed next session.
