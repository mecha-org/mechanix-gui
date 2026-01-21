import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/select/select_type.dart';

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

class BluetoothConnection {
  final String address;
  final bool connectionLoading;

  const BluetoothConnection({
    required this.address,
    required this.connectionLoading,
  });
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

List<SelectOption<String>> bluetoothDeviceOptions(String value) => [
      SelectOption(
          value: 'speaker',
          label: "Speaker",
          leading: IconWidget(
              isActive: 'Speaker' == value,
              iconWidth: 16,
              iconHeight: 20,
              iconPath: Images.speaker)),
      SelectOption(
          value: 'audio-headphone',
          label: "Headphone",
          leading: IconWidget(
              isActive: 'audio-headphone' == value,
              iconWidth: 16,
              iconHeight: 20,
              iconPath: Images.audioHeadset)),
      SelectOption(
          value: 'phone',
          label: "Mobile",
          leading: IconWidget(
              isActive: 'phone' == value,
              iconWidth: 16,
              iconHeight: 20,
              iconPath: Images.mobile)),
      SelectOption(
          value: 'computer',
          label: "TV",
          leading: IconWidget(
              isActive: 'computer' == value,
              iconWidth: 16,
              iconHeight: 20,
              iconPath: Images.tv)),
      SelectOption(
          value: 'multimedia-player',
          label: "Car",
          leading: IconWidget(
              isActive: 'multimedia-player' == value,
              iconWidth: 16,
              iconHeight: 20,
              iconPath: Images.car)),
      SelectOption(
          value: 'other',
          label: "Other",
          leading: IconWidget(
              isActive: 'other' == value,
              iconWidth: 16,
              iconHeight: 20,
              iconPath: Images.audioHeadset)),
    ];
