# Agent Team Report
Generated: Fri Apr 24 03:49:42 UTC 2026

## Test Results
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

## Mutation Testing
MISSED   src/error.rs:10:9: replace <impl fmt::Display for SeymourError>::fmt -> fmt::Result with Ok(Default::default()) in 0s build + 0s test
::warning file=src/error.rs,line=10,col=9,endLine=12,endCol=10,title=Missed mutant:: replace <impl fmt::Display for SeymourError>::fmt -> fmt::Result with Ok(Default::default())
MISSED   src/validator.rs:37:27: replace < with <= in validate in 0s build + 0s test
::warning file=src/validator.rs,line=37,col=27,endLine=37,endCol=28,title=Missed mutant:: replace < with <= in validate
7 mutants tested in 14s: 2 missed, 3 caught, 2 unviable

## Coverage

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Filename                      Regions    Missed Regions     Cover   Functions  Missed Functions  Executed       Lines      Missed Lines     Cover    Branches   Missed Branches     Cover
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
error.rs                            6                 6     0.00%           1                 1     0.00%           4                 4     0.00%           0                 0         -
parser.rs                          11                 3    72.73%           2                 1    50.00%           6                 1    83.33%           0                 0         -
validator.rs                      172                13    92.44%          11                 0   100.00%         124                11    91.13%           0                 0         -
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
TOTAL                             189                22    88.36%          14                 2    85.71%         134                16    88.06%           0                 0         -

## Lint
    Checking ryu v1.0.23
    Checking dyn-clone v1.0.20
    Checking napi-sys v2.4.0
   Compiling serde_derive v1.0.228
   Compiling ctor v0.2.9
   Compiling schemars_derive v0.8.22
    Checking napi v2.16.17
   Compiling napi-derive v2.16.13
    Checking serde_yaml v0.9.34+deprecated
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 9.60s
