// features/network/presentation/bloc/wireless_settings_state.dart

import 'package:equatable/equatable.dart';
import 'package:mechanix_settings/src/features/network/models/access_points.dart';
import 'package:mechanix_settings/src/features/network/models/saved_networks.dart';
import 'package:nm/nm.dart';

class WirelessSettingsState extends Equatable {
  final bool wifiOn;
  
  final List<AccessPoints> availableOtherNetworks;
  final List<AccessPoints> availableSavedNetworks;
  final List<SavedNetworks> allSavedNetworks;

  final bool availableOtherNetworksLoading;
  final bool availableSavedNetworksLoading;

  final AccessPoints? selectedAccessPoint;    // connected network ? 
  final NetworkManagerAccessPoint? selectedNMAccessPoint;

  final String? wifiState;
  final String? error;
  final AccessPoints? connectedNetwork;     // connected network ? 
  final NetworkManagerDeviceState? deviceState;

  const WirelessSettingsState({
    required this.wifiOn,
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
  });

  WirelessSettingsState copyWith(
      {bool? wifiOn,
      List<AccessPoints>? availableOtherNetworks,
      List<AccessPoints>? availableSavedNetworks,
      List<SavedNetworks>? allSavedNetworks,
      bool? availableOtherNetworksLoading,
      bool? availableSavedNetworksLoading,

      AccessPoints? selectedAccessPoint,
      String? wifiState,
      String? error,
      AccessPoints? connectedNetwork,
      NetworkManagerDeviceState? deviceState,
      NetworkManagerAccessPoint? selectedNMAccessPoint}) {
    return WirelessSettingsState(
      wifiOn: wifiOn ?? this.wifiOn,
      availableOtherNetworks: availableOtherNetworks ?? this.availableOtherNetworks,
      availableSavedNetworks: availableSavedNetworks ?? this.availableSavedNetworks,
      allSavedNetworks: allSavedNetworks ?? this.allSavedNetworks,
      availableOtherNetworksLoading: availableOtherNetworksLoading ?? this.availableOtherNetworksLoading,
      availableSavedNetworksLoading: availableSavedNetworksLoading ?? this.availableSavedNetworksLoading,
      error: error,
      selectedAccessPoint: selectedAccessPoint ?? this.selectedAccessPoint,
      wifiState: wifiState ?? this.wifiState,
      connectedNetwork: connectedNetwork ?? this.connectedNetwork,
      deviceState: deviceState ?? this.deviceState,
      selectedNMAccessPoint:
          selectedNMAccessPoint ?? this.selectedNMAccessPoint,
    );
  }

  @override
  List<Object?> get props => [
        wifiOn,
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
      ];
}
