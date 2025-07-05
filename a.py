
import random
import os
import threading
import multiprocessing
import time

def thread_task(thread_num):
    #print(f"  Thread {thread_num} in process PID: {os.getpid()}, Thread ID: {threading.get_ident()}")
    s = 0
    for i in range(10**6):
        s += i * i
    fname = f"thread_{os.getpid()}_{thread_num}.tmp"
    with open(fname, "w") as f:
        for i in range(10000):
            f.write(f"{i}\n")
        f.flush()
        os.fsync(f.fileno())

    try:
        with open('/proc/sys/vm/drop_caches', 'w') as drop:
            drop.write('3\n')
    except Exception as e:
        pass
        #print(f"Warning: Could not drop caches: {e}")

    with open(fname, "r") as f:
        lines = f.readlines()
    os.remove(fname)
    time.sleep(0.2)

def process_task(proc_num):
    #print(f"Process {proc_num} started, PID: {os.getpid()}")
    threads = []
    for t in range(2):
        th = threading.Thread(target=thread_task, args=(t,))
        th.start()
        threads.append(th)
    for th in threads:
        th.join()
    #print(f"Process {proc_num} finished, PID: {os.getpid()}")

if __name__ == "__main__":
    filename = "a.txt"
    num = random.randint(1, 1000000)
    with open(filename, "w") as f:
        f.write(str(num))
        f.flush()
    with open(filename, "r") as f:
        read_num = f.read().strip()
    #print(f"Random number written and read from {filename}: {read_num}")
    os.remove(filename)

    thread_task('000')

    processes = []
    for p in range(2):
        proc = multiprocessing.Process(target=process_task, args=(p,))
        proc.start()
        processes.append(proc)
    for proc in processes:
        proc.join()
    #print("All processes and threads finished.")

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
