//! x86_64 syscall ids
//! Reference: https://git.musl-libc.org/cgit/musl/tree/arch/x86_64/bits/syscall.h.in

pub const SYS_READ: usize = 0;
pub const SYS_WRITE: usize = 1;
pub const SYS_OPEN: usize = 2;
pub const SYS_CLOSE: usize = 3;
pub const SYS_STAT: usize = 4;
pub const SYS_FSTAT: usize = 5;
pub const SYS_LSTAT: usize = 6;
pub const SYS_POLL: usize = 7;
pub const SYS_LSEEK: usize = 8;
pub const SYS_MMAP: usize = 9;
pub const SYS_MPROTECT: usize = 10;
pub const SYS_MUNMAP: usize = 11;
pub const SYS_BRK: usize = 12;
pub const SYS_RT_SIGACTION: usize = 13;
pub const SYS_RT_SIGPROCMASK: usize = 14;
pub const SYS_RT_SIGRETURN: usize = 15;
pub const SYS_IOCTL: usize = 16;
pub const SYS_PREAD64: usize = 17;
pub const SYS_PWRITE64: usize = 18;
pub const SYS_READV: usize = 19;
pub const SYS_WRITEV: usize = 20;
pub const SYS_ACCESS: usize = 21;
pub const SYS_PIPE: usize = 22;
pub const SYS_SELECT: usize = 23;
pub const SYS_SCHED_YIELD: usize = 24;
pub const SYS_MREMAP: usize = 25;
pub const SYS_MSYNC: usize = 26;
pub const SYS_MINCORE: usize = 27;
pub const SYS_MADVISE: usize = 28;
pub const SYS_SHMGET: usize = 29;
pub const SYS_SHMAT: usize = 30;
pub const SYS_SHMCTL: usize = 31;
pub const SYS_DUP: usize = 32;
pub const SYS_DUP2: usize = 33;
pub const SYS_PAUSE: usize = 34;
pub const SYS_NANOSLEEP: usize = 35;
pub const SYS_GETITIMER: usize = 36;
pub const SYS_ALARM: usize = 37;
pub const SYS_SETITIMER: usize = 38;
pub const SYS_GETPID: usize = 39;
pub const SYS_SENDFILE: usize = 40;
pub const SYS_SOCKET: usize = 41;
pub const SYS_CONNECT: usize = 42;
pub const SYS_ACCEPT: usize = 43;
pub const SYS_SENDTO: usize = 44;
pub const SYS_RECVFROM: usize = 45;
pub const SYS_SENDMSG: usize = 46;
pub const SYS_RECVMSG: usize = 47;
pub const SYS_SHUTDOWN: usize = 48;
pub const SYS_BIND: usize = 49;
pub const SYS_LISTEN: usize = 50;
pub const SYS_GETSOCKNAME: usize = 51;
pub const SYS_GETPEERNAME: usize = 52;
pub const SYS_SOCKETPAIR: usize = 53;
pub const SYS_SETSOCKOPT: usize = 54;
pub const SYS_GETSOCKOPT: usize = 55;
pub const SYS_CLONE: usize = 56;
pub const SYS_FORK: usize = 57;
pub const SYS_VFORK: usize = 58;
pub const SYS_EXECVE: usize = 59;
pub const SYS_EXIT: usize = 60;
pub const SYS_WAIT4: usize = 61;
pub const SYS_KILL: usize = 62;
pub const SYS_UNAME: usize = 63;
pub const SYS_SEMGET: usize = 64;
pub const SYS_SEMOP: usize = 65;
pub const SYS_SEMCTL: usize = 66;
pub const SYS_SHMDT: usize = 67;
pub const SYS_MSGGET: usize = 68;
pub const SYS_MSGSND: usize = 69;
pub const SYS_MSGRCV: usize = 70;
pub const SYS_MSGCTL: usize = 71;
pub const SYS_FCNTL: usize = 72;
pub const SYS_FLOCK: usize = 73;
pub const SYS_FSYNC: usize = 74;
pub const SYS_FDATASYNC: usize = 75;
pub const SYS_TRUNCATE: usize = 76;
pub const SYS_FTRUNCATE: usize = 77;
pub const SYS_GETDENTS: usize = 78;
pub const SYS_GETCWD: usize = 79;
pub const SYS_CHDIR: usize = 80;
pub const SYS_FCHDIR: usize = 81;
pub const SYS_RENAME: usize = 82;
pub const SYS_MKDIR: usize = 83;
pub const SYS_RMDIR: usize = 84;
pub const SYS_CREAT: usize = 85;
pub const SYS_LINK: usize = 86;
pub const SYS_UNLINK: usize = 87;
pub const SYS_SYMLINK: usize = 88;
pub const SYS_READLINK: usize = 89;
pub const SYS_CHMOD: usize = 90;
pub const SYS_FCHMOD: usize = 91;
pub const SYS_CHOWN: usize = 92;
pub const SYS_FCHOWN: usize = 93;
pub const SYS_LCHOWN: usize = 94;
pub const SYS_UMASK: usize = 95;
pub const SYS_GETTIMEOFDAY: usize = 96;
pub const SYS_GETRLIMIT: usize = 97;
pub const SYS_GETRUSAGE: usize = 98;
pub const SYS_SYSINFO: usize = 99;
pub const SYS_TIMES: usize = 100;
pub const SYS_PTRACE: usize = 101;
pub const SYS_GETUID: usize = 102;
pub const SYS_SYSLOG: usize = 103;
pub const SYS_GETGID: usize = 104;
pub const SYS_SETUID: usize = 105;
pub const SYS_SETGID: usize = 106;
pub const SYS_GETEUID: usize = 107;
pub const SYS_GETEGID: usize = 108;
pub const SYS_SETPGID: usize = 109;
pub const SYS_GETPPID: usize = 110;
pub const SYS_GETPGRP: usize = 111;
pub const SYS_SETSID: usize = 112;
pub const SYS_SETREUID: usize = 113;
pub const SYS_SETREGID: usize = 114;
pub const SYS_GETGROUPS: usize = 115;
pub const SYS_SETGROUPS: usize = 116;
pub const SYS_SETRESUID: usize = 117;
pub const SYS_GETRESUID: usize = 118;
pub const SYS_SETRESGID: usize = 119;
pub const SYS_GETRESGID: usize = 120;
pub const SYS_GETPGID: usize = 121;
pub const SYS_SETFSUID: usize = 122;
pub const SYS_SETFSGID: usize = 123;
pub const SYS_GETSID: usize = 124;
pub const SYS_CAPGET: usize = 125;
pub const SYS_CAPSET: usize = 126;
pub const SYS_RT_SIGPENDING: usize = 127;
pub const SYS_RT_SIGTIMEDWAIT: usize = 128;
pub const SYS_RT_SIGQUEUEINFO: usize = 129;
pub const SYS_RT_SIGSUSPEND: usize = 130;
pub const SYS_SIGALTSTACK: usize = 131;
pub const SYS_UTIME: usize = 132;
pub const SYS_MKNOD: usize = 133;
pub const SYS_USELIB: usize = 134;
pub const SYS_PERSONALITY: usize = 135;
pub const SYS_USTAT: usize = 136;
pub const SYS_STATFS: usize = 137;
pub const SYS_FSTATFS: usize = 138;
pub const SYS_SYSFS: usize = 139;
pub const SYS_GETPRIORITY: usize = 140;
pub const SYS_SETPRIORITY: usize = 141;
pub const SYS_SCHED_SETPARAM: usize = 142;
pub const SYS_SCHED_GETPARAM: usize = 143;
pub const SYS_SCHED_SETSCHEDULER: usize = 144;
pub const SYS_SCHED_GETSCHEDULER: usize = 145;
pub const SYS_SCHED_GET_PRIORITY_MAX: usize = 146;
pub const SYS_SCHED_GET_PRIORITY_MIN: usize = 147;
pub const SYS_SCHED_RR_GET_INTERVAL: usize = 148;
pub const SYS_MLOCK: usize = 149;
pub const SYS_MUNLOCK: usize = 150;
pub const SYS_MLOCKALL: usize = 151;
pub const SYS_MUNLOCKALL: usize = 152;
pub const SYS_VHANGUP: usize = 153;
pub const SYS_MODIFY_LDT: usize = 154;
pub const SYS_PIVOT_ROOT: usize = 155;
pub const SYS__SYSCTL: usize = 156;
pub const SYS_PRCTL: usize = 157;
pub const SYS_ARCH_PRCTL: usize = 158;
pub const SYS_ADJTIMEX: usize = 159;
pub const SYS_SETRLIMIT: usize = 160;
pub const SYS_CHROOT: usize = 161;
pub const SYS_SYNC: usize = 162;
pub const SYS_ACCT: usize = 163;
pub const SYS_SETTIMEOFDAY: usize = 164;
pub const SYS_MOUNT: usize = 165;
pub const SYS_UMOUNT2: usize = 166;
pub const SYS_SWAPON: usize = 167;
pub const SYS_SWAPOFF: usize = 168;
pub const SYS_REBOOT: usize = 169;
pub const SYS_SETHOSTNAME: usize = 170;
pub const SYS_SETDOMAINNAME: usize = 171;
pub const SYS_IOPL: usize = 172;
pub const SYS_IOPERM: usize = 173;
pub const SYS_CREATE_MODULE: usize = 174;
pub const SYS_INIT_MODULE: usize = 175;
pub const SYS_DELETE_MODULE: usize = 176;
pub const SYS_GET_KERNEL_SYMS: usize = 177;
pub const SYS_QUERY_MODULE: usize = 178;
pub const SYS_QUOTACTL: usize = 179;
pub const SYS_NFSSERVCTL: usize = 180;
pub const SYS_GETPMSG: usize = 181;
pub const SYS_PUTPMSG: usize = 182;
pub const SYS_AFS_SYSCALL: usize = 183;
pub const SYS_TUXCALL: usize = 184;
pub const SYS_SECURITY: usize = 185;
pub const SYS_GETTID: usize = 186;
pub const SYS_READAHEAD: usize = 187;
pub const SYS_SETXATTR: usize = 188;
pub const SYS_LSETXATTR: usize = 189;
pub const SYS_FSETXATTR: usize = 190;
pub const SYS_GETXATTR: usize = 191;
pub const SYS_LGETXATTR: usize = 192;
pub const SYS_FGETXATTR: usize = 193;
pub const SYS_LISTXATTR: usize = 194;
pub const SYS_LLISTXATTR: usize = 195;
pub const SYS_FLISTXATTR: usize = 196;
pub const SYS_REMOVEXATTR: usize = 197;
pub const SYS_LREMOVEXATTR: usize = 198;
pub const SYS_FREMOVEXATTR: usize = 199;
pub const SYS_TKILL: usize = 200;
pub const SYS_TIME: usize = 201;
pub const SYS_FUTEX: usize = 202;
pub const SYS_SCHED_SETAFFINITY: usize = 203;
pub const SYS_SCHED_GETAFFINITY: usize = 204;
pub const SYS_SET_THREAD_AREA: usize = 205;
pub const SYS_IO_SETUP: usize = 206;
pub const SYS_IO_DESTROY: usize = 207;
pub const SYS_IO_GETEVENTS: usize = 208;
pub const SYS_IO_SUBMIT: usize = 209;
pub const SYS_IO_CANCEL: usize = 210;
pub const SYS_GET_THREAD_AREA: usize = 211;
pub const SYS_LOOKUP_DCOOKIE: usize = 212;
pub const SYS_EPOLL_CREATE: usize = 213;
pub const SYS_EPOLL_CTL_OLD: usize = 214;
pub const SYS_EPOLL_WAIT_OLD: usize = 215;
pub const SYS_REMAP_FILE_PAGES: usize = 216;
pub const SYS_GETDENTS64: usize = 217;
pub const SYS_SET_TID_ADDRESS: usize = 218;
pub const SYS_RESTART_SYSCALL: usize = 219;
pub const SYS_SEMTIMEDOP: usize = 220;
pub const SYS_FADVISE64: usize = 221;
pub const SYS_TIMER_CREATE: usize = 222;
pub const SYS_TIMER_SETTIME: usize = 223;
pub const SYS_TIMER_GETTIME: usize = 224;
pub const SYS_TIMER_GETOVERRUN: usize = 225;
pub const SYS_TIMER_DELETE: usize = 226;
pub const SYS_CLOCK_SETTIME: usize = 227;
pub const SYS_CLOCK_GETTIME: usize = 228;
pub const SYS_CLOCK_GETRES: usize = 229;
pub const SYS_CLOCK_NANOSLEEP: usize = 230;
pub const SYS_EXIT_GROUP: usize = 231;
pub const SYS_EPOLL_WAIT: usize = 232;
pub const SYS_EPOLL_CTL: usize = 233;
pub const SYS_TGKILL: usize = 234;
pub const SYS_UTIMES: usize = 235;
pub const SYS_VSERVER: usize = 236;
pub const SYS_MBIND: usize = 237;
pub const SYS_SET_MEMPOLICY: usize = 238;
pub const SYS_GET_MEMPOLICY: usize = 239;
pub const SYS_MQ_OPEN: usize = 240;
pub const SYS_MQ_UNLINK: usize = 241;
pub const SYS_MQ_TIMEDSEND: usize = 242;
pub const SYS_MQ_TIMEDRECEIVE: usize = 243;
pub const SYS_MQ_NOTIFY: usize = 244;
pub const SYS_MQ_GETSETATTR: usize = 245;
pub const SYS_KEXEC_LOAD: usize = 246;
pub const SYS_WAITID: usize = 247;
pub const SYS_ADD_KEY: usize = 248;
pub const SYS_REQUEST_KEY: usize = 249;
pub const SYS_KEYCTL: usize = 250;
pub const SYS_IOPRIO_SET: usize = 251;
pub const SYS_IOPRIO_GET: usize = 252;
pub const SYS_INOTIFY_INIT: usize = 253;
pub const SYS_INOTIFY_ADD_WATCH: usize = 254;
pub const SYS_INOTIFY_RM_WATCH: usize = 255;
pub const SYS_MIGRATE_PAGES: usize = 256;
pub const SYS_OPENAT: usize = 257;
pub const SYS_MKDIRAT: usize = 258;
pub const SYS_MKNODAT: usize = 259;
pub const SYS_FCHOWNAT: usize = 260;
pub const SYS_FUTIMESAT: usize = 261;
pub const SYS_NEWFSTATAT: usize = 262;
pub const SYS_UNLINKAT: usize = 263;
pub const SYS_RENAMEAT: usize = 264;
pub const SYS_LINKAT: usize = 265;
pub const SYS_SYMLINKAT: usize = 266;
pub const SYS_READLINKAT: usize = 267;
pub const SYS_FCHMODAT: usize = 268;
pub const SYS_FACCESSAT: usize = 269;
pub const SYS_PSELECT6: usize = 270;
pub const SYS_PPOLL: usize = 271;
pub const SYS_UNSHARE: usize = 272;
pub const SYS_SET_ROBUST_LIST: usize = 273;
pub const SYS_GET_ROBUST_LIST: usize = 274;
pub const SYS_SPLICE: usize = 275;
pub const SYS_TEE: usize = 276;
pub const SYS_SYNC_FILE_RANGE: usize = 277;
pub const SYS_VMSPLICE: usize = 278;
pub const SYS_MOVE_PAGES: usize = 279;
pub const SYS_UTIMENSAT: usize = 280;
pub const SYS_EPOLL_PWAIT: usize = 281;
pub const SYS_SIGNALFD: usize = 282;
pub const SYS_TIMERFD_CREATE: usize = 283;
pub const SYS_EVENTFD: usize = 284;
pub const SYS_FALLOCATE: usize = 285;
pub const SYS_TIMERFD_SETTIME: usize = 286;
pub const SYS_TIMERFD_GETTIME: usize = 287;
pub const SYS_ACCEPT4: usize = 288;
pub const SYS_SIGNALFD4: usize = 289;
pub const SYS_EVENTFD2: usize = 290;
pub const SYS_EPOLL_CREATE1: usize = 291;
pub const SYS_DUP3: usize = 292;
pub const SYS_PIPE2: usize = 293;
pub const SYS_INOTIFY_INIT1: usize = 294;
pub const SYS_PREADV: usize = 295;
pub const SYS_PWRITEV: usize = 296;
pub const SYS_RT_TGSIGQUEUEINFO: usize = 297;
pub const SYS_PERF_EVENT_OPEN: usize = 298;
pub const SYS_RECVMMSG: usize = 299;
pub const SYS_FANOTIFY_INIT: usize = 300;
pub const SYS_FANOTIFY_MARK: usize = 301;
pub const SYS_PRLIMIT64: usize = 302;
pub const SYS_NAME_TO_HANDLE_AT: usize = 303;
pub const SYS_OPEN_BY_HANDLE_AT: usize = 304;
pub const SYS_CLOCK_ADJTIME: usize = 305;
pub const SYS_SYNCFS: usize = 306;
pub const SYS_SENDMMSG: usize = 307;
pub const SYS_SETNS: usize = 308;
pub const SYS_GETCPU: usize = 309;
pub const SYS_PROCESS_VM_READV: usize = 310;
pub const SYS_PROCESS_VM_WRITEV: usize = 311;
pub const SYS_KCMP: usize = 312;
pub const SYS_FINIT_MODULE: usize = 313;
pub const SYS_SCHED_SETATTR: usize = 314;
pub const SYS_SCHED_GETATTR: usize = 315;
pub const SYS_RENAMEAT2: usize = 316;
pub const SYS_SECCOMP: usize = 317;
pub const SYS_GETRANDOM: usize = 318;
pub const SYS_MEMFD_CREATE: usize = 319;
pub const SYS_KEXEC_FILE_LOAD: usize = 320;
pub const SYS_BPF: usize = 321;
pub const SYS_EXECVEAT: usize = 322;
pub const SYS_USERFAULTFD: usize = 323;
pub const SYS_MEMBARRIER: usize = 324;
pub const SYS_MLOCK2: usize = 325;
pub const SYS_COPY_FILE_RANGE: usize = 326;
pub const SYS_PREADV2: usize = 327;
pub const SYS_PWRITEV2: usize = 328;
pub const SYS_PKEY_MPROTECT: usize = 329;
pub const SYS_PKEY_ALLOC: usize = 330;
pub const SYS_PKEY_FREE: usize = 331;
pub const SYS_STATX: usize = 332;
pub const SYS_IO_PGETEVENTS: usize = 333;
pub const SYS_RSEQ: usize = 334;

// custom temporary syscall
pub const SYS_MAP_PCI_DEVICE: usize = 999;
pub const SYS_GET_PADDR: usize = 998;
pub const SYS_REG_IH: usize = 997;

