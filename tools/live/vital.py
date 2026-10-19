#!/usr/bin/env python3
"""Vital: the window accent reacts to how hard the machine is working.
Calm = primary, busy = tertiary, hot or starved = error. Drives the Hyprland active border (and a
non-black shadow glow). With --bar it also writes a waybar accent line (needs waybar-side to be on)."""
import glob
import os
import subprocess
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.realpath(__file__)))
import common as c

BAR = "--bar" in sys.argv
BAR_CSS = f"{c.HOME}/.config/ml4w/settings/live-accent.css"
MODE_FILE = f"{c.STATE}/mode"  # written by modal.py; while set, borders belong to Modal
STEPS = 16
INTERVAL = 2.0

base = {"cols": None, "angle": 0, "shadow": None}
cache = {"step": None, "mode_seen": False, "bar_t": 0.0}


def cpu_times():
    with open("/proc/stat") as f:
        v = [int(x) for x in f.readline().split()[1:]]
    return v[3] + v[4], sum(v)  # idle(+iowait), total


TEMP_SOURCES = {"coretemp": None, "k10temp": None, "nvme": None, "amdgpu": None, "xe": "pkg"}  # hwmon name -> required label


def temp_norm():
    """Hottest trusted sensor (CPU, NVMe, GPU package). Board chips like nct6791 report floating inputs, so they are skipped."""
    best = 0.0
    for hw in glob.glob("/sys/class/hwmon/hwmon*"):
        try:
            name = open(f"{hw}/name").read().strip()
        except OSError:
            continue
        if name not in TEMP_SOURCES:
            continue
        for p in glob.glob(f"{hw}/temp*_input"):
            want = TEMP_SOURCES[name]
            try:
                if want and want not in open(p.replace("_input", "_label")).read():
                    continue
                t = int(open(p).read()) / 1000
            except (OSError, ValueError):
                continue
            if 0 < t < 120:
                best = max(best, t)
    return max(0.0, min(1.0, (best - 50) / 40)) if best else 0.0  # 50C calm .. 90C hot


def battery_norm():
    for d in glob.glob("/sys/class/power_supply/BAT*"):
        try:
            cap = int(open(f"{d}/capacity").read())
            st = open(f"{d}/status").read().strip()
        except (OSError, ValueError):
            continue
        if st != "Charging" and cap < 30:
            return 1.0 - cap / 30
    return 0.0


def accent(pal, t):
    if t < 0.5:
        return c.mix(pal["primary"], pal["tertiary"], t / 0.5)
    return c.mix(pal["tertiary"], pal["error"], (t - 0.5) / 0.5)


def write_bar(col):
    css = f"window#waybar {{ box-shadow: inset 0 -2px 0 {c.css_hex(col)}; }}\n#workspaces button.active {{ color: {c.css_hex(col)}; }}\n"
    try:
        if open(BAR_CSS).read() == css:
            return
    except OSError:
        pass
    open(BAR_CSS, "w").write(css)
    if time.time() - cache["bar_t"] > 30:  # waybar flickers on reload: at most twice a minute
        cache["bar_t"] = time.time()
        subprocess.run(["pkill", "-USR2", "-x", "waybar"])


def setup():
    cols, angle = c.parse_gradient(c.option("general:col.active_border"))
    base["cols"], base["angle"] = cols, angle
    sh = c.option("decoration:shadow:color")
    base["shadow"] = c.parse_gradient(sh)[0][:1] or None


def loop(stop):
    idle0, tot0 = cpu_times()
    ema = 0.0
    while not stop["flag"]:
        time.sleep(INTERVAL)
        idle1, tot1 = cpu_times()
        d = max(1, tot1 - tot0)
        cpu = 1.0 - (idle1 - idle0) / d
        idle0, tot0 = idle1, tot1
        ema = ema * 0.7 + cpu * 0.3
        score = max(min(1.0, ema * 1.25), temp_norm(), battery_norm())
        step = round(score * STEPS)

        mode = open(MODE_FILE).read().strip() if os.path.exists(MODE_FILE) else ""
        if mode:
            cache["mode_seen"] = True  # Modal owns the border right now
            continue
        if step == cache["step"] and not cache["mode_seen"]:
            continue
        cache["step"], cache["mode_seen"] = step, False

        pal = c.palette()
        if not pal:
            continue
        col = accent(pal, step / STEPS)
        a0 = (base["cols"][0][3] if base["cols"] else 255) / 255
        c.set_active_border([(*col, round(a0 * 255)), (*c.mix(col, pal["secondary"], 0.4), round(a0 * 255))], base["angle"] or 45)
        sh = base["shadow"]
        if sh and sum(sh[0][:3]) > 90:  # recolor only visible colored shadows (Riso, Cyber), not black ones
            c.set_shadow_color(c.rgba(col, sh[0][3] / 255))
        if BAR:
            write_bar(col)


def cleanup():
    if base["cols"]:
        c.set_active_border(base["cols"], base["angle"])
    if base["shadow"]:
        s = base["shadow"][0]
        c.set_shadow_color(c.rgba(s[:3], s[3] / 255))
    if BAR:
        try:
            open(BAR_CSS, "w").write("")
            subprocess.run(["pkill", "-USR2", "-x", "waybar"])
        except OSError:
            pass


sys.exit(c.run_until_term(setup, loop, cleanup))
