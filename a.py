import sys
from time import sleep

print("hello from python")

sleep(1)

print("Fake error message", file=sys.stderr)
