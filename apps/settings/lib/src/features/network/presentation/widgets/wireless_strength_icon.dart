import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/features/network/models/types.dart';
import 'package:widgets/mechanix.dart';

IconWidget getWirelessStrengthIcon({
  required int strength,
  bool isActive = false,
  required bool isSecure,
}) {
  final SignalLevel level = _getSignalLevel(strength);

  return _getIconWidget(
    iconPath: _getIconPath(level: level, isSecure: isSecure),
    isActive: isActive,
  );
}

SignalLevel _getSignalLevel(int strength) {
  return switch (strength) {
    > 75 => SignalLevel.high,
    > 50 => SignalLevel.medium,
    > 25 => SignalLevel.low,
    _ => SignalLevel.none,
  };
}

String _getIconPath({
  required SignalLevel level,
  required bool isSecure,
}) {
  const secureIcons = {
    SignalLevel.high: Images.wifiHighLocked,
    SignalLevel.medium: Images.wifiMediumLocked,
    SignalLevel.low: Images.wifiLowLocked,
    SignalLevel.none: Images.wifiNone,
  };

  const openIcons = {
    SignalLevel.high: Images.wifiHighOpen,
    SignalLevel.medium: Images.wifiMediumOpen,
    SignalLevel.low: Images.wifiLowOpen,
    SignalLevel.none: Images.wifiNone,
  };

  return isSecure ? secureIcons[level]! : openIcons[level]!;
}

IconWidget _getIconWidget({required String iconPath, required bool isActive}) {
  return IconWidget(
    iconPath: iconPath,
    isActive: isActive,
    boxWidth: 28,
    boxHeight: 28,
    iconWidth: 24,
    iconHeight: 24,
  );
}
