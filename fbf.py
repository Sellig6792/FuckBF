import os
import sys
import argparse

from interpreter import Interpreter


def get_usage() -> str:
    return "fbf (path) [-h] [-c COMPILE] [-x EXECUTE]"


def get_args() -> argparse.Namespace:
    if not sys.argv[1].startswith('-'):
        path = sys.argv[1]
        del sys.argv[1]
    else:
        path = None

    parser = argparse.ArgumentParser(prog="FuckBrainFuck", description="The improved version of BrainFuck",
                                     usage=get_usage())

    parser.add_argument("-c", "--compile", help="Compile the file")
    parser.add_argument("-x", "--execute", help="Execute a string")

    _args = parser.parse_args()
    setattr(_args, 'path', path)
    return _args


if __name__ == '__main__':
    args = get_args()

    if args.path:
        with open(args.path, mode='r+') as file:
            code = ''.join(file.readlines())
            Interpreter(code)()
    if args.execute:
        Interpreter(args.execute)()
