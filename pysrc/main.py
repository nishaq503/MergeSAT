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


def test(instance: Instance) -> None:
    instance.solve()
    print(f'found {instance.num_solutions()} solutions, {len(instance.certificates)} certificates')
    # for certificate in instance.certificates:
    #     print(str(certificate))
    return


if __name__ == '__main__':
    seed(42)
    _filename = '../random.cnf'
    # gen_random(_filename, 5, 20, 50)
    test(read(_filename))
