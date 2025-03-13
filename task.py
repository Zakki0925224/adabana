import os
import subprocess
import sys

OUTPUT_DIR = "build"
KERNEL_DIR = "kernel"
DUMP_DIR = "dump"
THIRD_PARTY_DIR = "third-party"

KERNEL_FILE = "kernel8.img"
KERNEL_OUT = "target/aarch64-raspi3-kernel/debug/kernel"
DTB_FILE = "bcm2710-rpi-3-b.dtb"
FONT_FILE = "font.psf"
COZETTE_BDF = "cozette.bdf"

ARCH_TOOLCHAIN = "aarch64-linux-gnu-"

QEMU_ARCH = "qemu-system-aarch64"
QEMU_MACHINE_TYPE = "raspi3b"
QEMU_DEVICES = []
QEMU_DRIVES = []
QEMU_ARGS = [
    f"-M {QEMU_MACHINE_TYPE}",
    f"-kernel {OUTPUT_DIR}/{KERNEL_FILE}",
    f"-dtb {THIRD_PARTY_DIR}/{DTB_FILE}",
    "-no-reboot",
    "-no-shutdown",
    "-m 1G",
    # "-display none",
    "-serial null -serial stdio",  # PL011 -> Mini UART
    "-monitor telnet::5678,server,nowait",
    "-gdb tcp::3333",
]


def qemu_cmd() -> str:
    qemu_args = " ".join(QEMU_ARGS)
    qemu_drives = " ".join(QEMU_DRIVES)
    qemu_devices = " ".join(QEMU_DEVICES)

    return f"{QEMU_ARCH} {qemu_args} {qemu_drives} {qemu_devices}"


def git_submodule_update_cmd(path: str) -> str:
    return f"git submodule update --init --recursive {path}"


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
    run_cmd(f"mkdir -p {THIRD_PARTY_DIR}")

    # download DTB file
    dtb_path = f"{THIRD_PARTY_DIR}/{DTB_FILE}"
    if not os.path.exists(dtb_path):
        run_cmd(
            f"wget https://github.com/raspberrypi/firmware/raw/master/boot/{DTB_FILE} -O {dtb_path}"
        )


def build_cozette():
    d = f"./{THIRD_PARTY_DIR}"

    if not os.path.exists(f"{d}/{FONT_FILE}"):
        run_cmd(
            f'wget -qO- https://api.github.com/repos/slavfox/Cozette/releases/latest | grep "{COZETTE_BDF}" | cut -d : -f 2,3 | tr -d \\" | wget -O ./{COZETTE_BDF} -i -',
            dir=d,
            ignore_error=True,
        )
        run_cmd(
            f"bdf2psf --fb ./{COZETTE_BDF} /usr/share/bdf2psf/standard.equivalents /usr/share/bdf2psf/fontsets/Uni2.512 512 ./{FONT_FILE}",
            dir=d,
        )
        run_cmd(f"rm ./{COZETTE_BDF}", dir=d)


def build_kernel():
    d = f"./{KERNEL_DIR}"
    run_cmd("cargo build", d)
    run_cmd(
        f"{ARCH_TOOLCHAIN}objcopy --strip-all -O binary {KERNEL_OUT} {OUTPUT_DIR}/{KERNEL_FILE}"
    )


def build():
    init()
    build_cozette()
    build_kernel()


def run():
    build()
    run_cmd(qemu_cmd())


def run_with_gdb():
    build()
    run_cmd(f"{qemu_cmd()} -S")


def monitor():
    run_cmd("telnet localhost 5678")


def gdb():
    run_cmd(f'gdb-multiarch {KERNEL_OUT} -ex "target remote :3333"')


def dump():
    build()
    run_cmd(f"mkdir -p {DUMP_DIR}")
    run_cmd(f"{ARCH_TOOLCHAIN}objdump -d {KERNEL_OUT} > {DUMP_DIR}/dump_kernel.txt")


def clean():
    run_cmd(f"rm -rf {OUTPUT_DIR}")
    run_cmd(f"rm -rf {DUMP_DIR}")
    run_cmd("cargo clean")


TASKS = [
    init,
    build_kernel,
    build_cozette,
    build,
    run,
    run_with_gdb,
    monitor,
    gdb,
    dump,
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
