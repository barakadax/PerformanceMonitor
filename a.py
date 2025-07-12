import multiprocessing
import threading
import os
import random
import time
from concurrent.futures import ThreadPoolExecutor, wait

def heavy_computation():
    time.sleep(3.0)
    rand_value = random.randint(100_000_000, 999_999_999)
    for _ in range(rand_value):
        _ = 123456789 * 987654321 + 1000000000 - 543210987 / 12345

def run_thread(thread_num: int):
    heavy_computation()
    #print(f"Thread iteration: {thread_num} started with PID: {os.getpid()} & thread id: {threading.get_native_id()}")

def run_process(state: bool):
    #print(f"Process started with PID: {os.getpid()} state: {state}")

    if state:
        p1 = multiprocessing.Process(target=run_process, args=(False,))
        p2 = multiprocessing.Process(target=run_process, args=(False,))
        p1.start()
        p2.start()
        p1.join()
        p2.join()
    else:
        with ThreadPoolExecutor(max_workers=2) as executor:
            futures = [executor.submit(run_thread, i) for i in range(4)]
            wait(futures)

    #print(f"Process {os.getpid()} completed.")

if __name__ == "__main__":
    p1 = multiprocessing.Process(target=run_process, args=(True,))
    p2 = multiprocessing.Process(target=run_process, args=(False,))
    p1.start()
    p2.start()
    p1.join()
    p2.join()

    #print("Done Python code")
    exit(42)

######################################################################
"""
import sys
import os
from time import sleep

print(f"hello from python, PID: {os.getpid()}")

sleep(1)

print("Fake error message", file=sys.stderr)

exit(42)
"""
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
