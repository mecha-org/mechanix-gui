class BluetoothListItem {
  String title;
  String? subTitle;  // status - connected/connecting/disconnected/saved..
  String? connectNetworkRoute; 
  BluetoothListItem({required this.title, this.subTitle = '', this.connectNetworkRoute = '', required BluetoothDetailsType bluetoothDetails});
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

