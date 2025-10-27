import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/select/select_type.dart';

class BluetoothAdapter {
  String name;
  String? alias;
  bool powered;
  bool discovering;
  bool discoverable;

  BluetoothAdapter({required this.name, this.alias, required this.powered, required this.discovering, required this.discoverable});

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

class BluetoothListItem {
  String title;
  String? subTitle; // status - connected/connecting/disconnected/saved..
  String? connectNetworkRoute;
  BluetoothListItem(
      {required this.title,
      this.subTitle = '',
      this.connectNetworkRoute = '',
      required BluetoothDetailsType bluetoothDetails});
}

// connected
class BluetoothDetailsType {
  String bluetoothName;
  String status;
  BluetoothDetailsType({required this.bluetoothName, required this.status});
}

enum BluetoothStatus {
  connected,
  connecting,
  disconnected,
  disconnecting,
  saved,
  unknown
}

Map<BluetoothStatus, String> bluetoothStatusToString = {
  BluetoothStatus.unknown: '-',
  BluetoothStatus.connected: 'Connected',
  BluetoothStatus.connecting: 'Connecting',
  BluetoothStatus.disconnected: 'Disconnected',
  BluetoothStatus.disconnecting: 'Disconnecting',
  BluetoothStatus.saved: 'Saved'
};

enum BluetoothDevice {
  speaker,
  headphone,
  mobile,
  tv,
  car,
  other,
}

final List<SelectOption<BluetoothDevice>> bluetoothDeviceOptions = [
  BluetoothDevice.speaker.toSelectOption('Speaker',
      leading:
          IconWidget(iconWidth: 16, iconHeight: 20, iconPath: Images.speaker)),
  BluetoothDevice.headphone.toSelectOption('Headphone',
      leading: IconWidget(
          iconWidth: 20, iconHeight: 18, iconPath: Images.audioHeadset)),
  BluetoothDevice.mobile.toSelectOption('Mobile',
      leading:
          IconWidget(iconWidth: 16, iconHeight: 20, iconPath: Images.mobile)),
  BluetoothDevice.tv.toSelectOption('TV',
      leading: IconWidget(iconWidth: 20, iconHeight: 20, iconPath: Images.tv)),
  BluetoothDevice.car.toSelectOption('Car',
      leading: IconWidget(iconWidth: 24, iconHeight: 20, iconPath: Images.car)),
  BluetoothDevice.other.toSelectOption(
    'Other',
  ),
];
