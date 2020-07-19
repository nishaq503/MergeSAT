from pysrc.instance import Instance

if __name__ == '__main__':
    _filename = '../test.cnf'
    with open(_filename, 'r') as _fp:
        instance: Instance = Instance.read(_fp)
    print(instance)

    _out_file = '../out.cnf'
    with open(_out_file, 'w') as _fp:
        instance.write(_fp)
