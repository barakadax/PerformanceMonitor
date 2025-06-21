import sys
import os
from time import sleep

print(f"hello from python, PID: {os.getpid()}")

sleep(1)

print("Fake error message", file=sys.stderr)

exit(42)

######################################################################
"""
import os

def infinite_running_loop():
    while True:
        _ = 123456789 * 987654321 + 1000000000 - 543210987 / 12345

if __name__ == "__main__":
    print(f"PID: {os.getpid()}")
    infinite_running_loop()

"""
