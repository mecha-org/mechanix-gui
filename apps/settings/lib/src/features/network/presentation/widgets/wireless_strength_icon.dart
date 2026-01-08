import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:widgets/mechanix.dart';

IconWidget getWirelessStrengthIcon(
    {required int strength, bool isActive = false, required bool isSecure}) {
  if (isSecure) {
    if (strength > 75) {
      return _getIconWidget(
          iconPath: Images.wifiHighLocked, isActive: isActive);
    } else if (strength > 50) {
      return _getIconWidget(
          iconPath: Images.wifiMediumLocked, isActive: isActive);
    } else if (strength > 25) {
      return _getIconWidget(iconPath: Images.wifiLowLocked, isActive: isActive);
    } else {
      return _getIconWidget(iconPath: Images.wifiNone, isActive: isActive);
    }
  } else {
    if (strength > 75) {
      return _getIconWidget(iconPath: Images.wifiHighOpen, isActive: isActive);
    } else if (strength > 50) {
      return _getIconWidget(
          iconPath: Images.wifiMediumOpen, isActive: isActive);
    } else if (strength > 25) {
      return _getIconWidget(iconPath: Images.wifiLowOpen, isActive: isActive);
    } else {
      return _getIconWidget(iconPath: Images.wifiNone, isActive: isActive);
    }
  }
}

IconWidget _getIconWidget({required String iconPath, required bool isActive}) {
  return IconWidget(
    iconPath: iconPath,
    isActive: isActive,
    boxWidth: 24,
    boxHeight: 24,
    iconWidth: 22.5,
    iconHeight: 18,
  );
}
