# Mechanix Services

This document provides an overview of the Mechanix services and how to introspect them using `busctl`.


### Requirements

* Linux system with `busctl` installed.
* D-Bus access (available by default on most Linux distributions).

### Usage

This script relies on the `busctl` command-line utility. You can use `busctl` to directly interact with the services using the introspection information provided by this script.

Here's a general usage example:



## Services

The following services are available:

- `org.mechanix.services.Bluetooth`
- `org.mechanix.services.Wireless`
- `org.mechanix.services.Power`
- `org.mechanix.services.Display`
- `org.mechanix.services.HostMetrics`

## Introspection

You can use the `busctl` command to introspect these services. Here's how you can do it:

```bash
busctl --user introspect SERVICE OBJECT INTERFACE

```

## Use

```bash
busctl --user call org.mechanix.services.Display /org/mechanix/services/Display org.mechanix.services.Display GetBrightness
```

Replace `SERVICE`, `OBJECT`, and `INTERFACE` with the actual service name, object path, and interface name you want to introspect.

## Methods

Here are some of the methods available under each service:

### org.mechanix.services.Bluetooth

- `.Connect`
- `.Disable`
- `.Disconnect`
- `.Enable`
- `.GetBluetoothProperties`
- `.Scan`
- `.Status`

### org.mechanix.services.Wireless

- `.Connect`
- `.Disconnect`
- `.Info`
- `.KnownNetworks`
- `.Scan`
- `.Status`

### org.mechanix.services.Power

- `.GetBatteryInfo`
- `.GetBatteryStatus`
- `.GetCpuFrequency`
- `.GetCpuGovernor`
- `.Info`
- `.SetCpuFrequency`
- `.SetCpuGovernor`

### org.mechanix.services.Display

- `.GetBrightness`
- `.SetBacklightOff`
- `.SetBacklightOn`
- `.SetBrightness`

### org.mechanix.services.HostMetrics

- `.GetCpuFreq`
- `.GetCpuUsage`
- `.GetDiskInfo`
- `.GetLoadAverage`
- `.GetMemoryInfo`
- `.GetMemoryUsage`
- `.GetNetworkData`
- `.GetNetworkUsage`
- `.GetUptime`

