# HyperLogLog implementation

This is a very simple implementation of hyperloglog for educational purposes.
This is not production-ready code.


### Testing out hyperloglog
```
ls target.txt # make sure it exists
python3 hyperloglog.py
```

1 is considered a perfect score.

The results are estimated to be within 0.8 - 1.2, with a 20% error margin


### Running with a different dataset

**NOTE**: this operation might take a while
```
bash time shuffle.sh target.txt
```

### Replace target file with new one
```
mv target.txt _target.txt
mv shuffled_target.txt target.txt
```

### Run hyperloglog again
python3 hyperloglog.py
```