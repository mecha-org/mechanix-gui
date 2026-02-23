import 'package:bluez/bluez.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/features/bluetooth/models/types.dart';

class BluetoothDeviceClassifier {
  static String _shortUuid(String uuid) =>
      uuid.length >= 8 ? uuid.substring(4, 8).toLowerCase() : '';

  static bool _hasUuid(List<String> uuids, String shortCode) =>
      uuids.any((u) => _shortUuid(u) == shortCode.toLowerCase());

  static int _majorDeviceClass(int cod) => (cod >> 8) & 0x1F;
  static int _minorDeviceClass(int cod) => (cod >> 2) & 0x3F;

  static BluetoothDeviceCategory classify({
    required int deviceClass,
    required List<BlueZUUID> uuids,
  }) {
    final uuidStrings = uuids.map((u) => u.toString()).toList();

    final codResult = _classifyByCoD(deviceClass);
    final uuidResult = _classifyByUuids(uuidStrings);

    return _resolve(codResult, uuidResult);
  }

  static BluetoothDeviceCategory _classifyByCoD(int cod) {
    final major = _majorDeviceClass(cod);
    final minor = _minorDeviceClass(cod);

    switch (major) {
      case 0x01:
        return BluetoothDeviceCategory.computer;
      case 0x02:
        return BluetoothDeviceCategory.mobile;
      case 0x04:
        switch (minor) {
          case 0x01:
            return BluetoothDeviceCategory.headphones;
          case 0x02:
            return BluetoothDeviceCategory.headphones;
          case 0x05:
            return BluetoothDeviceCategory.speaker;
          case 0x06:
            return BluetoothDeviceCategory.headphones;
          case 0x08:
            return BluetoothDeviceCategory.car;
          case 0x09:
            return BluetoothDeviceCategory.tv;
          case 0x0A:
            return BluetoothDeviceCategory.speaker;
          default:
            return BluetoothDeviceCategory.unknown;
        }
      default:
        return BluetoothDeviceCategory.unknown;
    }
  }

  static BluetoothDeviceCategory _classifyByUuids(List<String> uuids) {
    final hasAudioSink = _hasUuid(uuids, '110b');
    final hasAudioSource = _hasUuid(uuids, '110a');
    final hasHandsFree = _hasUuid(uuids, '111e');
    final hasHandsFreeAG = _hasUuid(uuids, '111f');
    final hasHeadset = _hasUuid(uuids, '1108');
    final hasHeadsetAG = _hasUuid(uuids, '1112');
    final hasAVRCPTarget = _hasUuid(uuids, '110f');

    if (hasAudioSource && (hasHeadsetAG || hasHandsFreeAG) && !hasAudioSink) {
      return BluetoothDeviceCategory.mobile;
    }

    if (hasAudioSink && (hasHandsFree || hasHeadset)) {
      return BluetoothDeviceCategory.headphones;
    }

    if (hasAudioSink && hasAVRCPTarget && !hasHandsFree && !hasHeadset) {
      return BluetoothDeviceCategory.speaker;
    }

    if (hasAudioSink && !hasHandsFree && !hasHeadset) {
      return BluetoothDeviceCategory.speaker;
    }

    return BluetoothDeviceCategory.unknown;
  }

  static BluetoothDeviceCategory _resolve(
    BluetoothDeviceCategory cod,
    BluetoothDeviceCategory uuid,
  ) {
    if (cod == uuid) return cod;

    if (cod == BluetoothDeviceCategory.unknown) return uuid;
    if (uuid == BluetoothDeviceCategory.unknown) return cod;

    const preferredCod = {
      BluetoothDeviceCategory.mobile,
      BluetoothDeviceCategory.computer,
      BluetoothDeviceCategory.car,
      BluetoothDeviceCategory.tv,
    };

    if (preferredCod.contains(cod)) return cod;

    return uuid;
  }

  static String categoryLabel(category) {
    switch (category) {
      case BluetoothDeviceCategory.speaker:
        return 'Speaker';
      case BluetoothDeviceCategory.headphones:
        return 'Headphones';
      case BluetoothDeviceCategory.mobile:
        return 'Phone';
      case BluetoothDeviceCategory.computer:
        return 'Computer';
      case BluetoothDeviceCategory.tv:
        return 'TV';
      case BluetoothDeviceCategory.car:
        return 'Car';
      default:
        return 'Other';
    }
  }

  static String deviceIcon(category) {
    switch (category) {
      case BluetoothDeviceCategory.speaker:
        return Images.speaker;
      case BluetoothDeviceCategory.headphones:
        return Images.audioHeadset;
      case BluetoothDeviceCategory.mobile:
        return Images.mobile;
      case BluetoothDeviceCategory.computer:
        return Images.laptop;
      case BluetoothDeviceCategory.tv:
        return Images.tv;
      case BluetoothDeviceCategory.car:
        return Images.car;
      default:
        return Images.computer;
    }
  }
}
