#!/usr/bin/env python3
"""Solar: time-of-day color curve. Screen temperature and gamma glide between day and night values
(smooth, not stepped profiles); the phase is also written to the state dir for other tools.
Config (all optional): ~/.config/ml4w/settings/solar.conf with key=value lines:
  sunrise=06:00  sunset=18:00  day_k=6500  night_k=3600  night_gamma=88  fade_minutes=90"""
import math
import os
import subprocess
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.realpath(__file__)))
import common as c

CONF = f"{c.HOME}/.config/ml4w/settings/solar.conf"
PHASE_FILE = f"{c.STATE}/solar-phase"
spawned = {"proc": None}


def conf():
    d = dict(sunrise="06:00", sunset="18:00", day_k="6500", night_k="3600", night_gamma="88", fade_minutes="90")
    try:
        for l in open(CONF):
            if "=" in l and not l.lstrip().startswith("#"):
                k, v = l.split("=", 1)
                d[k.strip()] = v.strip()
    except OSError:
        pass
    return d


def minutes(hhmm):
    h, m = hhmm.split(":")
    return int(h) * 60 + int(m)


def day_factor(now_min, rise, sets, fade):
    """0 = full night, 1 = full day, smoothstep fades centered on sunrise and sunset."""
    def smooth(x):
        x = max(0.0, min(1.0, x))
        return x * x * (3 - 2 * x)
    up = smooth((now_min - (rise - fade / 2)) / fade)
    down = smooth((now_min - (sets - fade / 2)) / fade)
    return max(0.0, up - down)


def sunset_running():
    return subprocess.run(["pgrep", "-x", "hyprsunset"], capture_output=True).returncode == 0


def setup():
    if not sunset_running():
        spawned["proc"] = subprocess.Popen(["hyprsunset", "-t", "6500"], env=c._env(),
                                           stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        time.sleep(1.5)


def loop(stop):
    last = None
    while not stop["flag"]:
        cf = conf()
        t = time.localtime()
        f = day_factor(t.tm_hour * 60 + t.tm_min + t.tm_sec / 60, minutes(cf["sunrise"]), minutes(cf["sunset"]), float(cf["fade_minutes"]))
        kelvin = round(float(cf["night_k"]) + (float(cf["day_k"]) - float(cf["night_k"])) * f, -1)
        gamma = round(float(cf["night_gamma"]) + (100 - float(cf["night_gamma"])) * f)
        if (kelvin, gamma) != last:
            last = (kelvin, gamma)
            c.hctl("hyprsunset", "temperature", str(int(kelvin)))
            c.hctl("hyprsunset", "gamma", str(gamma))
            open(PHASE_FILE, "w").write(f"{f:.3f}\n")
        for _ in range(30):  # re-evaluate every 30 s, but wake fast on SIGTERM
            if stop["flag"]:
                break
            time.sleep(1)


def cleanup():
    c.hctl("hyprsunset", "identity")
    if spawned["proc"]:
        spawned["proc"].terminate()
    try:
        os.remove(PHASE_FILE)
    except OSError:
        pass


sys.exit(c.run_until_term(setup, loop, cleanup))
