# Mechanix Services

This document provides an overview of the Mechanix services and how to introspect them using `busctl`.

### Requirements

- Linux system with `busctl` installed.
- D-Bus access (available by default on most Linux distributions).

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

## Dbus Rules

In order to use the services, you need to have the following dbus rules in place: `/etc/dbus-1/system.d/mecha.conf`

```bash
<!DOCTYPE busconfig PUBLIC "-//freedesktop//DTD D-BUS Bus Configuration 1.0//EN"
          "http://www.freedesktop.org/standards/dbus/1.0/busconfig.dtd">
<busconfig>

  <policy user="root">
    <allow own="org.mechanix.services.Display"/>
    <allow own="org.mechanix.services.Wireless"/>
    <allow own="org.mechanix.services.Power"/>
    <allow own="org.mechanix.services.HostMetrics"/>
    <allow own="org.mechanix.services.Bluetooth"/>
  </policy>

  <policy context="default">
    <allow send_destination="org.mechanix.services.Display"/>
    <allow send_destination="org.mechanix.services.Wireless"/>
    <allow send_destination="org.mechanix.services.Power"/>
    <allow send_destination="org.mechanix.services.HostMetrics"/>
    <allow send_destination="org.mechanix.services.Bluetooth"/>
    <allow receive_sender="org.mechanix.services.*"/>

    <deny send_destination="org.mechanix.services.*"
          send_interface="org.mechanix.services.*.Server" send_member="SetHostName"/>
  </policy>

  <policy user="root">
    <allow send_destination="org.mechanix.services.*"/>
    <allow receive_sender="org.mechanix.services.*"/>
  </policy>
  <policy at_console="true">
    <allow send_destination="org.mechanix.services.*"/>
    <allow receive_sender="org.mechanix.services.*"/>
  </policy>

</busconfig>
```

## Introspection

You can use the `busctl` command to introspect these services. Here's how you can do it:

```bash
busctl --system introspect SERVICE OBJECT INTERFACE
```

## example

```bash
busctl --system introspect org.mechanix.services.Wireless /org/mechanix/services/Wireless org.mechanix.services.Wireless Scan
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

#### Bluetooth

```bash
busctl --system call org.mechanix.services.Bluetooth /org/mechanix/services/Bluetooth org.mechanix.services.Bluetooth Scan
```

```bash
busctl --system call org.mechanix.services.Bluetooth /org/mechanix/services/Bluetooth org.mechanix.services.Bluetooth Connect s "address"
```

```bash
busctl --system call org.mechanix.services.Bluetooth /org/mechanix/services/Bluetooth org.mechanix.services.Bluetooth Disconnect s "id"
```

```bash
busctl --system call org.mechanix.services.Bluetooth /org/mechanix/services/Bluetooth org.mechanix.services.Bluetooth Enable
```

```bash
busctl --system call org.mechanix.services.Bluetooth /org/mechanix/services/Bluetooth org.mechanix.services.Bluetooth Disable
```

```bash
busctl --system call org.mechanix.services.Bluetooth /org/mechanix/services/Bluetooth org.mechanix.services.Bluetooth Status
```

```bash
busctl --system call org.mechanix.services.Bluetooth /org/mechanix/services/Bluetooth org.mechanix.services.Bluetooth GetBluetoothProperties
```

### org.mechanix.services.Wireless

- `.Connect`
- `.Disconnect`
- `.Info`
- `.KnownNetworks`
- `.Scan`
- `.Status`

#### Wireless

```bash
busctl --system call org.mechanix.services.Wireless /org/mechanix/services/Wireless org.mechanix.services.Wireless Scan
```

```bash
busctl --system call org.mechanix.services.Wireless /org/mechanix/services/Wireless org.mechanix.services.Wireless Connect ss "ssid" "psk"
```

```bash
busctl --system call org.mechanix.services.Wireless /org/mechanix/services/Wireless org.mechanix.services.Wireless Info
```

```bash
busctl --system call org.mechanix.services.Wireless /org/mechanix/services/Wireless org.mechanix.services.Wireless Status
```

```bash
busctl --system call org.mechanix.services.Wireless /org/mechanix/services/Wireless org.mechanix.services.Wireless Enable
```

```bash
busctl --system call org.mechanix.services.Wireless /org/mechanix/services/Wireless org.mechanix.services.Wireless Disable
```

```bash
busctl --system call org.mechanix.services.Wireless /org/mechanix/services/Wireless org.mechanix.services.Wireless Disconnect s "id"
```

```bash
busctl --system call org.mechanix.services.Wireless /org/mechanix/services/Wireless org.mechanix.services.Wireless KnownNetworks
```

### org.mechanix.services.Power

- `.GetBatteryInfo`
- `.GetBatteryStatus`
- `.GetCpuFrequency`
- `.GetCpuGovernor`
- `.Info`
- `.SetCpuFrequency`
- `.SetCpuGovernor`

#### Power

```bash
busctl --system call org.mechanix.services.Power /org/mechanix/services/Power org.mechanix.services.Power GetBatteryInfo
```

```bash
busctl --system call org.mechanix.services.Power /org/mechanix/services/Power org.mechanix.services.Power GetBatteryStatus
```

```bash
busctl --system call org.mechanix.services.Power /org/mechanix/services/Power org.mechanix.services.Power GetCpuFrequency
```

```bash
busctl --system call org.mechanix.services.Power /org/mechanix/services/Power org.mechanix.services.Power GetCpuGovernor
```

```bash
busctl --system call org.mechanix.services.Power /org/mechanix/services/Power org.mechanix.services.Power Info
```

```bash
busctl --system call org.mechanix.services.Power /org/mechanix/services/Power org.mechanix.services.Power SetCpuFrequency s "frequency"
```

```bash
busctl --system call org.mechanix.services.Power /org/mechanix/services/Power org.mechanix.services.Power SetCpuGovernor s "governor"
```

### org.mechanix.services.Display

- `.GetBrightness`
- `.SetBacklightOff`
- `.SetBacklightOn`
- `.SetBrightness`

#### Display

```bash
busctl --system call org.mechanix.services.Display /org/mechanix/services/Display org.mechanix.services.Display GetBrightness
```

```bash
busctl --system call org.mechanix.services.Display /org/mechanix/services/Display org.mechanix.services.Display SetBacklightOff
```

```bash
busctl --system call org.mechanix.services.Display /org/mechanix/services/Display org.mechanix.services.Display SetBacklightOn
```

```bash
busctl --system call org.mechanix.services.Display /org/mechanix/services/Display org.mechanix.services.Display SetBrightness s "brightness"
```

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

#### HostMetrics

```bash
busctl --system call org.mechanix.services.HostMetrics /org/mechanix/services/HostMetrics org.mechanix.services.HostMetrics GetCpuFreq
```

```bash
busctl --system call org.mechanix.services.HostMetrics /org/mechanix/services/HostMetrics org.mechanix.services.HostMetrics GetCpuUsage
```

```bash
busctl --system call org.mechanix.services.HostMetrics /org/mechanix/services/HostMetrics org.mechanix.services.HostMetrics GetDiskInfo
```

```bash
busctl --system call org.mechanix.services.HostMetrics /org/mechanix/services/HostMetrics org.mechanix.services.HostMetrics GetLoadAverage
```

```bash
busctl --system call org.mechanix.services.HostMetrics /org/mechanix/services/HostMetrics org.mechanix.services.HostMetrics GetMemoryInfo
```

```bash
busctl --system call org.mechanix.services.HostMetrics /org/mechanix/services/HostMetrics org.mechanix.services.HostMetrics GetMemoryUsage
```

```bash
busctl --system call org.mechanix.services.HostMetrics /org/mechanix/services/HostMetrics org.mechanix.services.HostMetrics GetNetworkData
```

```bash
busctl --system call org.mechanix.services.HostMetrics /org/mechanix/services/HostMetrics org.mechanix.services.HostMetrics GetNetworkUsage
```

```bash
busctl --system call org.mechanix.services.HostMetrics /org/mechanix/services/HostMetrics org.mechanix.services.HostMetrics GetUptime
```

## Run Dbus Server

```bash
sudo ./mechanix-zbus-server -c "path/to/services-config.yml"
```