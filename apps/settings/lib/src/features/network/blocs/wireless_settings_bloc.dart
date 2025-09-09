// features/network/presentation/bloc/wireless_settings_bloc.dart

import 'dart:async';

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
    on<SelectNetwork>((event, emit) async {
      emit(state.copyWith(selectedAccessPoint: event.selectedAccessPoint));
    });
    on<LoadSavedNetworks>(_onLoadSavedNetworks);
    on<ConnectSavedNetwork>(_connectSavedNetwork);
    on<ForgetNetwork>(onForgetNetwork);
    on<DeleteSavedNetwork>(_deleteSavedNetwork);
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
        // logger.i("Network Property Update: $prop");
        if (prop.contains("WirelessEnabled")) {
          add(InitializeWifi());
        }
        if (prop.contains("State")) {
          var state = await wifiRepository.getWifiState();
          switch (state) {
            case NetworkManagerState.connectedGlobal:
              // add(LoadNetworks());
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
                prop.contains("ActiveAccessPoint"))) {
          // If the stream emits a new access point, reload networks
          add(LoadNetworks());
        }
      });
    } catch (e, stackTrace) {
      logger.e('Error initializing wifi stream $e, $stackTrace');
    }
  }

  Future<void> _initWifiStateAndReasonStream() async {
    try {
      final streamAndDevice = await wifiRepository.getWifiStateAndReason();

      streamAndDevice.device.propertiesChanged.listen((event) {
        if (event.contains("State")) {
          final currentState = streamAndDevice.device.state;
          if (currentState == NetworkManagerDeviceState.activated ||
              currentState == NetworkManagerDeviceState.disconnected) {
            add(LoadNetworks());
          }
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
  Future<void> close() {
    _wifiEventsSubscription?.cancel();
    _accessPointSubscription?.cancel();
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
      emit(state.copyWith(
          networks: res.available,
          loading: false,
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
      add(LoadNetworks());
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
      add(LoadNetworks()); // Reload networks when WiFi is enabled
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
    if (enabled) {
      add(LoadNetworks());
    }
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
    // add(LoadNetworks());
  }
}

// Define the custom event for WiFi status change
class WifiStatusChanged extends WirelessSettingsEvent {
  final List<String> wifiStatus; // Or whatever type your stream emits

  WifiStatusChanged(this.wifiStatus);
}
