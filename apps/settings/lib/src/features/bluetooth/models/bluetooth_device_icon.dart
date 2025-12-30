import 'package:mechanix_settings/src/commons/constants.dart';

String getBluetoothDeviceIcon(String device) {
  switch (device) {
    case 'audio-headset':
      return Images.audioHeadset;

    default:
      return Images.audioHeadset;
  }
}
