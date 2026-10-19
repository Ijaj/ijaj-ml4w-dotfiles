#!/usr/bin/env python3
"""Modal: the window border shows the active Hyprland submap (resize, move, ...), so the mode is
visible at a glance. Each submap name gets a palette slot; leaving the submap restores the border."""
import os
import subprocess
import sys
import threading
import time

sys.path.insert(0, os.path.dirname(os.path.realpath(__file__)))
import common as c

MODE_FILE = f"{c.STATE}/mode"
SLOTS = {"resize": "tertiary", "move": "secondary", "gaming": "error", "passthru": "error"}
FALLBACK = ["tertiary", "secondary", "inverse_primary", "error"]
base = {"cols": None, "angle": 0}


def slot_for(name):
    if name in SLOTS:
        return SLOTS[name]
    return FALLBACK[sum(map(ord, name)) % len(FALLBACK)]


def enter(name):
    pal = c.palette()
    if not pal:
        return
    open(MODE_FILE, "w").write(name)
    col = pal[slot_for(name)]
    c.set_active_border([(*col, 255), (*c.mix(col, pal["on_surface"], 0.25), 255)], 45)
    subprocess.Popen(["notify-send", "-t", "1200", "-h", "string:x-canonical-private-synchronous:ml4w-mode", f"MODE  {name}"],
                     stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)


def leave():
    try:
        open(MODE_FILE, "w").write("")
    except OSError:
        pass
    if base["cols"]:
        c.set_active_border(base["cols"], base["angle"])


def setup():
    cols, angle = c.parse_gradient(c.option("general:col.active_border"))
    base["cols"], base["angle"] = cols, angle
    leave()


def loop(stop):
    def reader():
        for ev, data in c.events():
            if ev == "submap":
                # Vital may have changed the baseline between modes; re-read it right before entering
                if data:
                    if not os.path.exists(MODE_FILE) or not open(MODE_FILE).read().strip():
                        cols, angle = c.parse_gradient(c.option("general:col.active_border"))
                        if cols:
                            base["cols"], base["angle"] = cols, angle
                    enter(data)
                else:
                    leave()

    threading.Thread(target=reader, daemon=True).start()
    while not stop["flag"]:
        time.sleep(0.5)


def cleanup():
    leave()


sys.exit(c.run_until_term(setup, loop, cleanup))
