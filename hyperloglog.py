#!/bin/python3
"""
A simple implementation of hyperloglog
"""
import hashlib

def md5(data: str) -> str:
    """
    A simple/stupid hashing function
    """
    strdata = data
    if not isinstance(data, str):
        strdata = str(data)

    res = hashlib.md5(strdata.encode())

    # convert hash to binary string
    repr = ""
    for c in res.hexdigest():
        repr += str(ord(c))

    return u'{0:0128b}'.format(int(repr))


def update_registry(max_zero: int, prefix: str, value: str) -> list:

    strval = f"{prefix}-{value}"
    hash_val = md5(strval)
    num_of_zeros = 0

    # count number of contigous zeros
    for c in range(0, len(hash_val)):
        index = len(hash_val) - 1 - c
        if hash_val[index] == "0":
            num_of_zeros += 1
        else:
            break

    # update number of contigous zeros if necessary
    if max_zero < num_of_zeros:
        return num_of_zeros

    return max_zero        


if __name__ == "__main__":

    registries = [0,0,0,0,0,0,0]
    actual = 0
    with open("target.txt", "r") as stream:
        lines = stream.readlines()
        actual = len(lines)
        for line in lines:
            for index in range(0, len(registries)):
                registries[index] = update_registry(registries[index], f"reg{index}", line)

    results = []
    for zeros in registries:
        results.append(1/pow(0.5, zeros))

    
    approx = sum(results)/len(results)
    err = actual / approx

    print(f"approximation: {approx}")
    print(f"actual: {actual}")
    print(f"error: {err}")