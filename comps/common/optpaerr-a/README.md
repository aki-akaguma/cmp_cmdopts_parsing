# optpaerr

*optpaerr* is error lib for optpa-utils

## Examples

```
let err = OptParseError::missing_argument("<input>");
let thing = format!("{}", err);
let expect = "Missing argument: <input>";
assert_eq!(thing, expect);
```
