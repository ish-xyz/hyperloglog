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
```
python3 hyperloglog.py
```


# Some results


### First dataset
dataset size: 466550
cardinality: 466550
result: 496201.14285714284
error: 0.9402437030144458 (1 is a perfect score)


### Second dataset
dataset size: 466550
cardinality: 466550
result: 426201.14285714284
error: 1.09403 (1 is a perfect score)

