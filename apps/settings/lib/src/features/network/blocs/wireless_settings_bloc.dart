// features/network/presentation/bloc/wireless_settings_bloc.dart

import 'dart:async';
import 'dart:convert';

import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/web.dart';
import 'package:mechanix_settings/src/features/network/data/wifi_repository.dart';
import 'package:mechanix_settings/src/features/network/models/access_points.dart';
import 'package:mechanix_settings/src/features/network/models/saved_networks.dart';
import 'package:nm/nm.dart';

import 'wireless_settings_event.dart';
import 'wireless_settings_state.dart';

class WirelessSettingsBloc
    extends Bloc<WirelessSettingsEvent, WirelessSettingsState> {
  final WifiRepository wifiRepository;
  final logger = Logger();
  StreamSubscription? _wifiEventsSubscription; // Make nullable
  StreamSubscription? _accessPointSubscription; // Make nullable
  StreamSubscription? _wifiStateAndReason; // Make nullable
  WirelessSettingsBloc({required this.wifiRepository})
      : super(WirelessSettingsState(
          wifiOn: false,
          networks: [],
          savedNetworks: [],
        )) {
    on<LoadNetworks>(_onLoadNetworks);
    on<ToggleWifi>(_onToggleWifi); //_onToggleWifi
    on<WifiStatusChanged>(_onWifiStatusChanged);
    on<InitializeWifi>(_onInitializeWifi);
    on<SelectNetwork>(_setSelectedNetwork);
    on<SelectNetworkPoint>(_setSelectedNetworkPoint);
    on<LoadSavedNetworks>(_onLoadSavedNetworks);
    on<ConnectSavedNetwork>(_connectSavedNetwork);
    on<ForgetNetwork>(onForgetNetwork);
    on<DeleteSavedNetwork>(_deleteSavedNetwork);
    on<UpdateAvailableNetworksEvent>(_updateAvailableNetworkList);
    on<UpdateConnectedNetworkEvent>(_updateConnectedNetwork);
    on<DeviceConnectionStateEvent>(_updateDeviceConnectionStateUpdate);

    on<Error>(handleError);
    // Handle async stream initialization
    _initializeWifiStream();
    _initializeAccessPointStream();
    _initWifiStateAndReasonStream();
  }

  Future<void> _initializeWifiStream() async {
    try {
      final stream = await wifiRepository.streamWifiEvents();
      _wifiEventsSubscription = stream.listen((prop) async {
        logger.i("Network Property Update: $prop");
        if (prop.contains("WirelessEnabled")) {}
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
      final stream = await wifiRepository.streamAccessPointStream();
      _accessPointSubscription = stream.listen((prop) async {
        logger.i("Access Point Update: $prop");
        if (prop.isNotEmpty &&
            (prop.contains("AccessPoints") ||
                prop.contains("ActiveAccessPoint") ||
                prop.contains("LastScan"))) {
          final savedNetworks =
              await wifiRepository.savedNetworks(state.networks);

          final availAccessPoints =
              await wifiRepository.availableAccessPoints(savedNetworks);

          if (availAccessPoints.available.isNotEmpty) {
            add(UpdateAvailableNetworksEvent(availAccessPoints.available));
          }

          if (availAccessPoints.active != null) {
            add(UpdateConnectedNetworkEvent(availAccessPoints.active!));
          }
          // If the stream emits a new access point, reload networks
        }
      });
    } catch (e, stackTrace) {
      // logger.e('Error initializing wifi stream $e, $stackTrace');
    }
  }

  Future<void> _initWifiStateAndReasonStream() async {
    try {
      final streamAndDevice = await wifiRepository.getWifiStateAndReason();

      _wifiStateAndReason =
          streamAndDevice.device.propertiesChanged.listen((event) {
        if (event.contains("State")) {
          final currentState = streamAndDevice.device.state;
          add(DeviceConnectionStateEvent(streamAndDevice.device.state));
        }
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
    _wifiEventsSubscription?.cancel();
    _accessPointSubscription?.cancel();
    _wifiStateAndReason?.cancel();
    wifiRepository.close();
    return super.close();
  }

  Future<void> _onLoadNetworks(
      LoadNetworks event, Emitter<WirelessSettingsState> emit) async {
    emit(state.copyWith(loading: true));
    try {
      logger.i('Loading networks...');
      final List<SavedNetworks> savedNetworks =
          await wifiRepository.savedNetworks(state.networks);

      var res = await wifiRepository.availableAccessPoints(savedNetworks);

      final deviceState = await wifiRepository.getNetworkState();

      if (deviceState != null) {
        add(DeviceConnectionStateEvent(deviceState));
      }

      emit(state.copyWith(
          networks: res.available,
          loading: false,
          savedNetworks: savedNetworks,
          connectedNetwork: res.active));
    } catch (e) {
      emit(state.copyWith(loading: false, error: e.toString()));
    }
  }

  void _onToggleWifi(
      ToggleWifi event, Emitter<WirelessSettingsState> emit) async {
    logger.i('Toggling WiFi: ${event.enabled}');
    try {
      wifiRepository.setWifiEnabled(event.enabled);

      // Call the repository to set WiFi enabled/disabled
    } catch (e) {
      logger.e('Error toggling WiFi: $e');
      emit(state.copyWith(error: e.toString()));
      return;
    }
    emit(state.copyWith(wifiOn: event.enabled));
    // Optionally reload networks or call a use case here
    if (event.enabled) {
      logger.i("event is wifi enabled");
    } else {
      emit(state.copyWith(networks: [])); // Clear networks when disabled
    }
    logger.i('WiFi toggled successfully: ${event.enabled}');
  }

  void _onWifiStatusChanged(
      WifiStatusChanged event, Emitter<WirelessSettingsState> emit) {
    for (var action in event.wifiStatus) {
      emit(state.copyWith(wifiState: action));
    }
  }

  Future<void> _onInitializeWifi(
      InitializeWifi event, Emitter<WirelessSettingsState> emit) async {
    final enabled = await wifiRepository.isWirelessEnabled();
    emit(state.copyWith(wifiOn: enabled));
    if (enabled) {}
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

  Future<void> _onLoadSavedNetworks(
      LoadSavedNetworks event, Emitter<WirelessSettingsState> emit) async {
    try {
      var res = await wifiRepository.availableAccessPoints(state.savedNetworks);
      List<AccessPoints> availableAccessPoints = [];
      if (res.active != null) {
        availableAccessPoints.add(res.active!);
      }
      if (res.available.isNotEmpty) {
        availableAccessPoints.addAll(res.available);
      }

      final saved = await wifiRepository.savedNetworks(availableAccessPoints);

      emit(state.copyWith(
        savedNetworks: saved,
        connectedNetwork: res.active,
      ));
    } catch (e) {
      // Optionally handle error
      emit(state.copyWith(savedNetworks: []));
    }
  }

  Future<void> _deleteSavedNetwork(
      DeleteSavedNetwork event, Emitter<WirelessSettingsState> emit) async {
    await wifiRepository.deleteSavedNetwork(event.ssid);
  }

  Future<void> _updateAvailableNetworkList(UpdateAvailableNetworksEvent event,
      Emitter<WirelessSettingsState> emit) async {
    try {
      final existingSSIDs =
          state.networks.map((n) => utf8.decode(n.nmAccessPoint.ssid)).toSet();

      final newNetworks = event.accessPoints.where((network) {
        final ssid = utf8.decode(network.nmAccessPoint.ssid);
        return ssid.isNotEmpty && !existingSSIDs.contains(ssid);
      }).toList();

      final newScanSSIDs = event.accessPoints
          .map((network) => utf8.decode(network.nmAccessPoint.ssid))
          .where((ssid) => ssid.isNotEmpty)
          .toSet();

      final unavailableSSIDs =
          existingSSIDs.where((ssid) => !newScanSSIDs.contains(ssid)).toList();

      if (newNetworks.isNotEmpty) {
        emit(state.copyWith(networks: [...state.networks, ...newNetworks]));
      }

      if (unavailableSSIDs.isNotEmpty) {
        final updatedNetworks = List<AccessPoints>.from(state.networks);

        updatedNetworks.removeWhere((network) {
          final networkSsid = utf8.decode(network.nmAccessPoint.ssid);
          return unavailableSSIDs.contains(networkSsid);
        });

        emit(state.copyWith(networks: updatedNetworks));
      }
    } catch (e) {
      // logger.e('error in update available networks $e');
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
