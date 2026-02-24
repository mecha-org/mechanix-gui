import 'package:bluez/bluez.dart';

class BluetoothAdapter {
  String name;
  String? alias;
  bool powered;
  bool discovering;
  bool discoverable;

  BluetoothAdapter(
      {required this.name,
      this.alias,
      required this.powered,
      required this.discovering,
      required this.discoverable});

  BluetoothAdapter copyWith({
    String? name,
    String? alias,
    bool? powered,
    bool? discovering,
    bool? discoverable,
  }) {
    return BluetoothAdapter(
      name: name ?? this.name,
      alias: alias ?? this.alias,
      powered: powered ?? this.powered,
      discovering: discovering ?? this.discovering,
      discoverable: discoverable ?? this.discoverable,
    );
  }
}

class BluetoothDeviceDetails {
  final BlueZDevice device;
  final BluetoothDeviceCategory deviceType;

  BluetoothDeviceDetails({
    required this.device,
    required this.deviceType,
  });

  BluetoothDeviceDetails copyWith({
    final BlueZDevice? device,
    final BluetoothDeviceCategory? deviceType,
  }) {
    return BluetoothDeviceDetails(
      device: device ?? this.device,
      deviceType: deviceType ?? this.deviceType,
    );
  }
}

enum BluetoothStatus {
  connected,
  connecting,
  disconnected,
  disconnecting,
  saved,
  unknown
}

class BluetoothConnection {
  final String address;
  final bool connectionLoading;

  const BluetoothConnection({
    required this.address,
    required this.connectionLoading,
  });
}

enum BluetoothDeviceCategory {
  speaker,
  headphones,
  mobile,
  computer,
  tv,
  car,
  unknown,
}
