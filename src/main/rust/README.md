# Generate the file

You can run in debug mode that is still faster than the python version but not as fast
```sh
cargo run -- 1000000000
```

Or you can compile it for release and run it (which is the best case if you want to run it several times)
```sh
target/release/create-measurements 100000000
```

Then check your file
```sh
ls -lrt ../../../data/measurements.txt
```
