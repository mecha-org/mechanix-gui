#!/bin/bash
WAYLAND_DISPLAY=wayland-1;
cd status_bar && cargo build &
cd .. &
cd action_bar && cargo build &
cd .. &
cd app_dock && cargo build &
cd .. &
cd app_drawer && cargo build &
cd .. &
cd app_manager && cargo build &
cd .. &
cd lock_screen && cargo build &
cd .. &
cd settings_drawer && cargo build &
wait