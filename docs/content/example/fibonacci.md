
Very simple fibonacci sequence proving.

```plonkscript
# k: 4
# in1: 1
# in2: 1

region first_row(a, b, c, in1, in2) {
    a[0] <== in1;
    b[0] <== in2;
    c[0] <== a[0] + b[0];

    [b[0], c[0]]
}

region next_row(a, b, c, last_b, last_c) {
    a[0] <== last_b;
    b[0] <== last_c;
    c[0] <== a[0] + b[0];

    c[0]
}

let N = 10;

pub input in1;
pub input in2;
pub output out;

col advice a;
col advice b;
col advice c;

let fr = first_row(a, b, c, in1, in2);
let last_b = fr[0];
let last_c = fr[1];
for i in 1..N {
    let c = next_row(a, b, c, last_b, last_c);
    last_b = last_c;
    last_c = c;
}

out <== last_c;
```
