# exec-target

*exec-target* is simple invoke command.

## Features

```
let oup = exec_target("target/debug/exe-stab-cat", &["-x"]);
```

```
assert_eq!(oup.stderr, "exe-mock-cat: invalid option -- \'x\'\nTry \'exe-mock-cat --help\' for more information.\n");
assert_eq!(oup.stdout, "");
assert_eq!(oup.status.success(), false);
```
