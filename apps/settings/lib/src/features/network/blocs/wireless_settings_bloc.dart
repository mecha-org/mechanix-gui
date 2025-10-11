// features/network/presentation/bloc/wireless_settings_bloc.dart

import 'dart:async';
import 'dart:convert';

import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/web.dart';
import 'package:mechanix_settings/src/features/network/data/wifi_repository.dart';
import 'package:mechanix_settings/src/features/network/models/access_points.dart';
import 'package:nm/nm.dart';

import 'wireless_settings_event.dart';
import 'wireless_settings_state.dart';

class WirelessSettingsBloc
    extends Bloc<WirelessSettingsEvent, WirelessSettingsState> {
  final WifiRepository wifiRepository;
  final logger = Logger();
  StreamSubscription? _wifiEventsSubscription;
  StreamSubscription? _accessPointSubscription;
  StreamSubscription? _savedAccessPointSubscription;
  StreamSubscription? _wifiStateAndReason;

  StreamSubscription<bool>? _wirelessSub;

  WirelessSettingsBloc({required this.wifiRepository})
      : super(WirelessSettingsState(
          wifiOn: false,
          availableOtherNetworks: [],
          availableSavedNetworks: [],
          allSavedNetworks: [],
          availableOtherNetworksLoading: false,
          availableSavedNetworksLoading: false,
        )) {
    _wirelessSub = wifiRepository.wirelessEnabledStream.listen((enabled) {
      logger.w('Wireless enabled state changed: $enabled');
      if (enabled != state.wifiOn) {
        add(WifiEnabledChanged(enabled));
      }
    });

    // Event handlers
    on<InitializeWifi>(_onInitializeWifi);
    on<ToggleWifi>(_onToggleWifi);
    on<WifiEnabledChanged>(_onWifiEnabledChanged);
    on<WifiStatusChanged>(_onWifiStatusChanged); // connecting, connected, etc
    on<SelectNetwork>(_setSelectedNetwork);
    on<SelectNetworkPoint>(_setSelectedNetworkPoint);
    on<ConnectSavedNetwork>(_connectSavedNetwork);
    on<ForgetNetwork>(onForgetNetwork);
    on<DeleteSavedNetwork>(_deleteSavedNetwork);

    on<UpdateAvailableNetworksEvent>(_updateAvailableNetworkList);
    on<GetSavedNetworksEvent>(_getSavedNetworkList);

    on<UpdateConnectedNetworkEvent>(_updateConnectedNetwork);
    on<DeviceConnectionStateEvent>(_updateDeviceConnectionStateUpdate);

    on<Error>(handleError);

    // Handle async stream initialization
    _initializeWifiStream(); // for a network connection state
  }

  Future<void> _initializeWifiStream() async {
    try {
      logger.i('Initializing WiFi event stream HERE...');
      final stream = await wifiRepository.streamWifiEvents();
      _wifiEventsSubscription = stream.listen((prop) async {
        if (prop.contains("State")) {
          var state = await wifiRepository.getWifiState();
          switch (state) {
            case NetworkManagerState.connectedGlobal:
              add(WifiStatusChanged(["Connected"]));
              break; // Prevents fallthrough to the default case
            default:
              add(WifiStatusChanged(["Connecting..."]));
              break;
          }
        }
      });
    } catch (e, stackTrace) {
      logger.e('Error initializing wifi stream $e, $stackTrace');
    }
  }

  Future<void> _initializeAccessPointStream() async {
    try {
      if (!state.wifiOn) return;

      final stream = await wifiRepository.streamWirelessDeviceStream();
      _accessPointSubscription = stream.listen((prop) async {
        // logger.i("Access Point Update: $prop");
        if (prop.isNotEmpty &&
            (prop.contains("LastScan") ||
                prop.contains("AccessPoints") ||
                prop.contains("ActiveAccessPoint"))) {
          final savedNetworks =
              await wifiRepository.savedNetworks(state.availableOtherNetworks);

          final availAccessPoints =
              await wifiRepository.availableAccessPoints(savedNetworks);

          if (availAccessPoints.available.isNotEmpty) {
            add(UpdateAvailableNetworksEvent(availAccessPoints.available));
          }

          if (availAccessPoints.active != null) {
            add(UpdateConnectedNetworkEvent(availAccessPoints.active!));
          }
        }
      });
    } catch (e, stackTrace) {
      logger.e('Error initializing wifi stream $e, $stackTrace');
    }
  }

  Future<void> _initWifiStateAndReasonStream() async {
    try {
      if (!state.wifiOn) return;

      final streamAndDevice = await wifiRepository.getWifiStateAndReason();

      _wifiStateAndReason =
          streamAndDevice.device.propertiesChanged.listen((event) {
        if (event.contains('StateReason')) {
          if (streamAndDevice.device.stateReason.state ==
                  NetworkManagerDeviceState.failed &&
              streamAndDevice.device.stateReason.reason ==
                  NetworkManagerDeviceStateReason.noSecrets) {
            logger.w('Authentication required!');
            add(Error("Authentication required!"));
          }
        }
      });
    } catch (e, stackTrace) {
      logger.e('Error initializing wifi stream $e, $stackTrace');
    }
  }

  // Always cancel your subscriptions when Bloc is closed
  @override
  Future<void> close() async {
    await _wifiEventsSubscription?.cancel();
    _wifiEventsSubscription = null;
    await _accessPointSubscription?.cancel();
    _accessPointSubscription = null;
    await _savedAccessPointSubscription?.cancel();
    _savedAccessPointSubscription = null;
    await _wifiStateAndReason?.cancel();
    _wifiStateAndReason = null;

    _wirelessSub?.cancel();
    wifiRepository.close();
    return super.close();
  }

  void _onToggleWifi(
      ToggleWifi event, Emitter<WirelessSettingsState> emit) async {
    logger.i('Toggling WiFi: ${event.enabled}');
    try {
      await wifiRepository.setWifiEnabled(event.enabled);
    } catch (e) {
      logger.e('Error toggling WiFi: $e');
      emit(state.copyWith(error: e.toString()));
      return;
    }
    logger.i('WiFi toggled successfully: ${event.enabled}');
  }

  void _onWifiEnabledChanged(
      WifiEnabledChanged event, Emitter<WirelessSettingsState> emit) {
    emit(state.copyWith(wifiOn: event.enabled));
    _handleWifiEnableChange(event.enabled);
  }

  void _handleWifiEnableChange(bool enabled) {
    if (enabled) {
      _initializeAccessPointStream();
      _initWifiStateAndReasonStream();
    } else {
      // On power off, cancel relevant the streams
      _accessPointSubscription?.cancel();
      _accessPointSubscription = null;

      _wifiStateAndReason?.cancel();
      _wifiStateAndReason = null;

      emit(state.copyWith(
          availableOtherNetworks: [],
          availableSavedNetworks: [],
          allSavedNetworks: [],
          availableOtherNetworksLoading: false,
          availableSavedNetworksLoading: false,
          connectedNetwork: null));
    }
  }

  void _onWifiStatusChanged(
      WifiStatusChanged event, Emitter<WirelessSettingsState> emit) {
    for (var action in event.wifiStatus) {
      emit(state.copyWith(wifiState: action));
    }
  }

  Future<void> _onInitializeWifi(
      InitializeWifi event, Emitter<WirelessSettingsState> emit) async {
    logger.i("Initializing WiFi...");
    final enabled = await wifiRepository.isWirelessEnabled();
    emit(state.copyWith(wifiOn: enabled));
    _handleWifiEnableChange(enabled);
  }

  Future<void> handleError(
      Error event, Emitter<WirelessSettingsState> emit) async {
    emit(state.copyWith(error: event.error));
  }

  Future<void> onForgetNetwork(
      ForgetNetwork event, Emitter<WirelessSettingsState> emit) async {
    await wifiRepository.forgetNetwork(event.ssid);
  }

  Future<void> _connectSavedNetwork(
      ConnectSavedNetwork event, Emitter<WirelessSettingsState> emit) async {
    try {
      emit(state.copyWith(wifiState: event.state));
      await wifiRepository.connectToSavedNetwork(event.nmAccessPoint);
      logger.i('Connected to saved network');
    } catch (e) {
      logger.e('Error connecting to saved network: $e');
      emit(state.copyWith(error: e.toString()));
    }
  }

  Future<void> _deleteSavedNetwork(
      DeleteSavedNetwork event, Emitter<WirelessSettingsState> emit) async {
    await wifiRepository.deleteSavedNetwork(event.ssid);
  }

// OKAY
  Future<void> _updateAvailableNetworkList(UpdateAvailableNetworksEvent event,
      Emitter<WirelessSettingsState> emit) async {
    try {
      emit(state.copyWith(
          availableOtherNetworksLoading: true,
          availableSavedNetworksLoading: true));

      final existingSSIDs = state.availableOtherNetworks
          .map((n) => utf8.decode(n.nmAccessPoint.ssid))
          .toSet();

      final newNetworks = event.accessPoints
          .where((network) {
            final ssid = utf8.decode(network.nmAccessPoint.ssid);
            return ssid.isNotEmpty && !existingSSIDs.contains(ssid);
          })
          .where((network) => !network.isSaved) // Only include unsaved networks
          .toList();

      final newScanSSIDs = event.accessPoints
          .map((network) => utf8.decode(network.nmAccessPoint.ssid))
          .where((ssid) => ssid.isNotEmpty)
          .toSet();

      final unavailableSSIDs =
          existingSSIDs.where((ssid) => !newScanSSIDs.contains(ssid)).toList();

      if (newNetworks.isNotEmpty) {
        emit(state.copyWith(availableOtherNetworks: [
          ...state.availableOtherNetworks,
          ...newNetworks
        ]));
      }

      if (unavailableSSIDs.isNotEmpty) {
        final updatedNetworks =
            List<AccessPoints>.from(state.availableOtherNetworks);

        updatedNetworks.removeWhere((network) {
          final networkSsid = utf8.decode(network.nmAccessPoint.ssid);
          return (unavailableSSIDs.contains(networkSsid));
        });

        var availableSavedNetworks =
            event.accessPoints.where((ap) => (ap.isSaved)).toList();

        emit(state.copyWith(
            availableOtherNetworks: updatedNetworks,
            availableSavedNetworks: availableSavedNetworks,
            availableOtherNetworksLoading: false,
            availableSavedNetworksLoading: false));
      }
    } catch (e) {
      logger.e('error in update available networks $e');
      emit(state.copyWith(
          availableOtherNetworksLoading: false,
          availableSavedNetworksLoading: false));
    }
  }

  Future<void> _getSavedNetworkList(
      GetSavedNetworksEvent event, Emitter<WirelessSettingsState> emit) async {
    try {
      final saved = await wifiRepository.getSavedNetworks();
      emit(state.copyWith(allSavedNetworks: saved));
    } catch (e) {
      logger.e('error in get saved networks $e');
    }
  }

  Future<void> _updateConnectedNetwork(UpdateConnectedNetworkEvent event,
      Emitter<WirelessSettingsState> emit) async {
    try {
      if (state.connectedNetwork == null) {
        emit(state.copyWith(connectedNetwork: event.accessPoint));
        return;
      }

      final newSsid = utf8.decode(event.accessPoint.nmAccessPoint.ssid);
      final currentSsid =
          utf8.decode(state.connectedNetwork!.nmAccessPoint.ssid);

      // Use exact equality comparison, not contains()
      final isSameNetwork = newSsid == currentSsid;

      // Only update if it's actually a different network
      if (!isSameNetwork) {
        emit(state.copyWith(connectedNetwork: event.accessPoint));
      }
    } catch (e) {
      logger.e('error in update connected network $e');
    }
  }

  Future<void> _updateDeviceConnectionStateUpdate(
      DeviceConnectionStateEvent event,
      Emitter<WirelessSettingsState> emit) async {
    emit(state.copyWith(deviceState: event.deviceState));
  }

  void _setSelectedNetwork(
      SelectNetwork event, Emitter<WirelessSettingsState> emit) {
    emit(state.copyWith(
      selectedAccessPoint: event.selectedAccessPoint,
    ));
  }

  void _setSelectedNetworkPoint(
      SelectNetworkPoint event, Emitter<WirelessSettingsState> emit) {
    emit(state.copyWith(
      selectedNMAccessPoint: event.selectedAccessPoint,
    ));
  }
}

// Define the custom event for WiFi status change
class WifiStatusChanged extends WirelessSettingsEvent {
  final List<String> wifiStatus; // Or whatever type your stream emits

  WifiStatusChanged(this.wifiStatus);
}
