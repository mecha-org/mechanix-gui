// features/network/presentation/bloc/wireless_settings_event.dart

import 'package:equatable/equatable.dart';
import 'package:mechanix_settings/src/features/network/models/access_points.dart';
import 'package:mechanix_settings/src/features/network/models/types.dart';
import 'package:nm/nm.dart';

abstract class WirelessSettingsEvent extends Equatable {
  @override
  List<Object?> get props => [];
}

class InitWifi extends WirelessSettingsEvent {}

class InitializeWifi extends WirelessSettingsEvent {}

class LoadNetworks extends WirelessSettingsEvent {}

// class LoadSavedNetworks extends WirelessSettingsEvent {}

class ForgetNetwork extends WirelessSettingsEvent {
  final String ssid;
  ForgetNetwork(this.ssid);
}

class ConnectSavedNetwork extends WirelessSettingsEvent {
  final String? state;
  final NetworkManagerAccessPoint nmAccessPoint;
  ConnectSavedNetwork(this.state, this.nmAccessPoint);
}

class ToggleWifi extends WirelessSettingsEvent {
  final bool enabled;
  ToggleWifi(this.enabled);

  @override
  List<Object?> get props => [enabled];
}

class WifiEnabledChanged extends WirelessSettingsEvent {
  final bool enabled;
  WifiEnabledChanged(this.enabled);

  @override
  List<Object?> get props => [enabled];
}

class Error extends WirelessSettingsEvent {
  final String error;
  Error(this.error);
}

class SelectNetwork extends WirelessSettingsEvent {
  final AccessPoints selectedAccessPoint;
  SelectNetwork(this.selectedAccessPoint);
}

class SelectNetworkPoint extends WirelessSettingsEvent {
  final NetworkManagerAccessPoint selectedAccessPoint;
  SelectNetworkPoint(this.selectedAccessPoint);
}

class ConnectedNetwork extends WirelessSettingsEvent {}

class UpdateAvailableNetworksEvent extends WirelessSettingsEvent {
  final List<AccessPoints> accessPoints;
  UpdateAvailableNetworksEvent(this.accessPoints);

  @override
  List<Object> get props => [accessPoints];
}

class GetSavedNetworksEvent extends WirelessSettingsEvent {}

class UpdateConnectedNetworkEvent extends WirelessSettingsEvent {
  final AccessPoints accessPoint;
  UpdateConnectedNetworkEvent(this.accessPoint);

  @override
  List<Object> get props => [accessPoint];
}

class SelectedWirelessProtocol extends WirelessSettingsEvent {
  final WirelessProtocol protocol;

  SelectedWirelessProtocol(this.protocol);

  @override
  List<Object> get props => [protocol];
}

class RefreshWifiList extends WirelessSettingsEvent {}

class WifiStatusChanged extends WirelessSettingsEvent {
  final WifiStatus wifiStatus;

  WifiStatusChanged(this.wifiStatus);

  @override
  List<Object> get props => [wifiStatus];
}
