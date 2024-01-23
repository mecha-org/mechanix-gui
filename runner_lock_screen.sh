#!/bin/bash
WAYLAND_DISPLAY=wayland-1;
cd status_bar && cargo run &
cd .. &
cd lock_screen && cargo run &
cd .. &
wait