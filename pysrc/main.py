from time import time

from pysrc.instance import Instance
from random import seed


def gen_random(filename: str, k: int, m: int, n: int) -> Instance:
    instance: Instance = Instance.generate_random(k, m, n)
    with open(filename, 'w') as fp:
        instance.write(fp)
    return instance


def read(filename: str) -> Instance:
    with open(filename, 'r') as fp:
        return Instance.read(fp)


def test(instance: Instance, filename: str) -> None:
    start = time()
    instance.solve()
    end = time()
    print(f'size: {instance.size}, time: {end - start:.2f} seconds.')
    with open(filename, 'w') as fp:
        instance.write_solutions(fp)
    return


if __name__ == '__main__':
    seed(42)
    _infile = '../sdiv15prop.cnf'
    _outfile = '../sdiv15prop_out.cnf'
    # gen_random(_infile, 3, 8, 64)
    test(read(_infile), _outfile)
