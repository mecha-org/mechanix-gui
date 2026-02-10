// features/network/presentation/bloc/wireless_settings_state.dart

import 'package:equatable/equatable.dart';
import 'package:mechanix_settings/src/features/network/models/access_points.dart';
import 'package:mechanix_settings/src/features/network/models/saved_networks.dart';
import 'package:mechanix_settings/src/features/network/models/types.dart';
import 'package:nm/nm.dart';

class WirelessSettingsState extends Equatable {
  final bool wifiOn;

  final List<AccessPoints> availableOtherNetworks;
  final List<AccessPoints> availableNetworks;
  final List<AccessPoints> availableSavedNetworks;
  final List<SavedWirelessNetwork> allSavedNetworks;

  final bool availableOtherNetworksLoading;
  final bool availableSavedNetworksLoading;

  final AccessPoints? selectedAccessPoint;
  final NetworkManagerAccessPoint? selectedNMAccessPoint;

  final WifiStatus? wifiState;
  final String? error;
  final AccessPoints? connectedNetwork;
  final NetworkManagerDeviceState? deviceState;
  final WirelessProtocol selectedAPWirelessProtocol;
  final ActivatingNetwork? activatingNetwork;
  final ActivatingNetwork? activatedNetwork;
  final ActivatingNetwork? deActivatedNetwork;
  final WiredDevice? wiredDevice;

  const WirelessSettingsState({
    required this.wifiOn,
    required this.availableNetworks,
    required this.availableOtherNetworks,
    required this.availableSavedNetworks,
    required this.allSavedNetworks,
    this.availableOtherNetworksLoading = false,
    this.availableSavedNetworksLoading = false,
    this.error,
    this.selectedAccessPoint,
    this.wifiState,
    this.connectedNetwork,
    this.deviceState,
    this.selectedNMAccessPoint,
    this.wiredDevice,
    this.activatingNetwork,
    this.activatedNetwork,
    this.deActivatedNetwork,
    this.selectedAPWirelessProtocol = WirelessProtocol.none,
  });

  WirelessSettingsState copyWith({
    bool? wifiOn,
    List<AccessPoints>? availableNetworks,
    List<AccessPoints>? availableOtherNetworks,
    List<AccessPoints>? availableSavedNetworks,
    List<SavedWirelessNetwork>? allSavedNetworks,
    bool? availableOtherNetworksLoading,
    bool? availableSavedNetworksLoading,
    AccessPoints? selectedAccessPoint,
    WifiStatus? wifiState,
    String? error,
    AccessPoints? connectedNetwork,
    NetworkManagerDeviceState? deviceState,
    NetworkManagerAccessPoint? selectedNMAccessPoint,
    WiredDevice? wiredDevice,
    ActivatingNetwork? activatingNetwork,
    ActivatingNetwork? activatedNetwork,
    ActivatingNetwork? deActivatedNetwork,
    WirelessProtocol? selectedAPWirelessProtocol,
  }) {
    return WirelessSettingsState(
      wifiOn: wifiOn ?? this.wifiOn,
      availableNetworks: availableNetworks ?? this.availableNetworks,
      availableOtherNetworks:
          availableOtherNetworks ?? this.availableOtherNetworks,
      availableSavedNetworks:
          availableSavedNetworks ?? this.availableSavedNetworks,
      allSavedNetworks: allSavedNetworks ?? this.allSavedNetworks,
      availableOtherNetworksLoading:
          availableOtherNetworksLoading ?? this.availableOtherNetworksLoading,
      availableSavedNetworksLoading:
          availableSavedNetworksLoading ?? this.availableSavedNetworksLoading,
      error: error,
      selectedAccessPoint: selectedAccessPoint ?? this.selectedAccessPoint,
      wifiState: wifiState ?? this.wifiState,
      connectedNetwork: connectedNetwork ?? this.connectedNetwork,
      deviceState: deviceState ?? this.deviceState,
      selectedNMAccessPoint:
          selectedNMAccessPoint ?? this.selectedNMAccessPoint,
      wiredDevice: wiredDevice ?? this.wiredDevice,
      activatingNetwork: activatingNetwork ?? this.activatingNetwork,
      activatedNetwork: activatedNetwork ?? this.activatedNetwork,
      deActivatedNetwork: deActivatedNetwork ?? this.deActivatedNetwork,
      selectedAPWirelessProtocol:
          selectedAPWirelessProtocol ?? this.selectedAPWirelessProtocol,
    );
  }

  @override
  List<Object?> get props => [
        wifiOn,
        availableNetworks,
        availableOtherNetworks,
        availableSavedNetworks,
        allSavedNetworks,
        availableOtherNetworksLoading,
        availableSavedNetworksLoading,
        selectedAccessPoint,
        error,
        wifiState,
        connectedNetwork,
        deviceState,
        selectedNMAccessPoint,
        activatingNetwork,
        activatedNetwork,
        deActivatedNetwork,
        selectedAPWirelessProtocol,
      ];
}

class WiredDevice {
  final int speed;
  final bool enabled;
  WiredDevice({required this.speed, required this.enabled});
}
