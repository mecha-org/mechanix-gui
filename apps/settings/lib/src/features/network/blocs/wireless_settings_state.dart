// features/network/presentation/bloc/wireless_settings_state.dart

import 'package:equatable/equatable.dart';
import 'package:mechanix_settings/src/features/network/models/access_points.dart';
import 'package:mechanix_settings/src/features/network/models/saved_networks.dart';

class WirelessSettingsState extends Equatable {
  final bool wifiOn;
  final List<AccessPoints> networks;
  final List<SavedNetworks> savedNetworks;
  final AccessPoints? selectedAccessPoint; // Add this
  final bool loading;
  final String? wifiState;
  final String? error;

  final AccessPoints? connectedNetwork;

  const WirelessSettingsState(
      {required this.wifiOn,
      required this.networks,
      required this.savedNetworks,
      this.loading = false,
      this.error,
      this.selectedAccessPoint,
      this.wifiState,
      this.connectedNetwork});

  WirelessSettingsState copyWith({
    bool? wifiOn,
    List<AccessPoints>? networks,
    List<SavedNetworks>? savedNetworks,
    bool? loading,
    AccessPoints? selectedAccessPoint,
    String? error,
    String? wifiState,
    AccessPoints? connectedNetwork,
  }) {
    return WirelessSettingsState(
        wifiOn: wifiOn ?? this.wifiOn,
        networks: networks ?? this.networks,
        savedNetworks: savedNetworks ?? this.savedNetworks,
        loading: loading ?? this.loading,
        error: error,
        selectedAccessPoint: selectedAccessPoint ?? this.selectedAccessPoint,
        wifiState: wifiState ?? this.wifiState,
        connectedNetwork: connectedNetwork ?? this.connectedNetwork);
  }

  @override
  List<Object?> get props => [
        wifiOn,
        networks,
        savedNetworks,
        loading,
        selectedAccessPoint,
        error,
        wifiState,
        connectedNetwork
      ];
}
