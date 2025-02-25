import os
import subprocess
import sys

OUTPUT_DIR = "build"
KERNEL_DIR = "kernel"

KERNEL_FILE = "kernel.elf"

QEMU_ARCH = "qemu-system-aarch64"
QEMU_MACHINE_TYPE = "raspi3b"
QEMU_DEVICES = []
QEMU_DRIVES = []
QEMU_ARGS = [
    "-no-reboot",
    "-no-shutdown",
    "-m 1G",
    "-serial mon:stdio",
    "-monitor telnet::5678,server,nowait",
]

def qemu_cmd() -> str:
    qemu_args = " ".join(QEMU_ARGS)
    qemu_drives = " ".join(QEMU_DRIVES)
    qemu_devices = " ".join(QEMU_DEVICES)

    return f"{QEMU_ARCH} -M {QEMU_MACHINE_TYPE} {qemu_args} {qemu_drives} {qemu_devices} -kernel {OUTPUT_DIR}/{KERNEL_FILE}"

def run_cmd(
    cmd: str,
    dir: str = "./",
    ignore_error: bool = False,
):
    print(f"\033[32m{cmd}\033[0m")
    cp = subprocess.run(cmd, shell=True, cwd=dir)
    exit_code = cp.returncode

    if exit_code != 0 and not ignore_error:
        exit(exit_code)

# tasks
def init():
    run_cmd(f"mkdir -p {OUTPUT_DIR}")

def build_kernel():
    d = f"./{KERNEL_DIR}"

    init()
    run_cmd("cargo build", d)
    run_cmd(f"cp ./target/aarch64-unknown-none/debug/kernel {OUTPUT_DIR}/{KERNEL_FILE}")

def build():
    build_kernel()

def run():
    run_cmd(qemu_cmd())

def monitor():
    run_cmd("telnet localhost 5678")

def clean():
    run_cmd(f"rm -rf {OUTPUT_DIR}")
    run_cmd("cargo clean")

TASKS = [
    init,
    build_kernel,
    build,
    run,
    monitor,
    clean,
]

if __name__ == "__main__":
    args = sys.argv

    if len(args) >= 2:
        for task in TASKS:
            if task.__name__ == args[1]:
                task()
                exit(0)

        print("Invalid task name.")
    else:
        print(f"Usage: {list(map(lambda x: x.__name__, TASKS))}")
