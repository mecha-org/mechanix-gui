// Define your enum
import 'package:widgets/widgets/select/select_type.dart';

enum NotificationSound {
  off,
  bloop,
  cosmic,
  space,
  supernova,
  crash,
}

final List<SelectOption<String>> notificationSoundOptions = [
  SelectOption(value: 'bloop', label: 'Bloop'),
  SelectOption(value: 'cosmic', label: 'Cosmic'),
  SelectOption(value: 'space', label: 'Space'),
  SelectOption(value: 'supernova', label: 'Supernova'),
  SelectOption(value: 'crash', label: 'Crash'),
  SelectOption(value: 'off', label: 'Off'),
];

class DBusSoundSettings {
  final bool enableSounds;
  final bool enableVibration;
  final String vibrationLevel;
  final String notificationSound;

  DBusSoundSettings({
    required this.enableSounds,
    required this.enableVibration,
    required this.vibrationLevel,
    required this.notificationSound,
  });
}

enum VibrationLevels {
  low,
  high,
  medium;
}

final List<SelectOption<String>> vibrationLevelOptions = [
  SelectOption(value: 'low', label: 'Low'),
  SelectOption(value: 'high', label: 'High'),
  SelectOption(value: 'medium', label: 'Medium'),
];

String vibrationLabel(value) {
  switch (value) {
    case 'low':
      return 'Low';
    case 'high':
      return 'High';
    default:
      return 'Medium';
  }
}

String notificationSoundLabel(value) {
  switch (value) {
    case 'bloop':
      return 'Bloop';
    case 'cosmic':
      return 'Cosmic';
    case 'space':
      return 'Space';
    case 'supernova':
      return 'Supernova';
    case 'crash':
      return 'Crash';
    default:
      return 'Off';
  }
}
