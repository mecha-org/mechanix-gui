import 'dart:async';
import 'dart:convert';

import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/web.dart';
import 'package:mechanix_settings/src/features/network/data/wifi_repository.dart';
import 'package:mechanix_settings/src/features/network/models/access_points.dart';
import 'package:mechanix_settings/src/features/network/models/types.dart';
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
      : super(const WirelessSettingsState(
          wifiOn: false,
          availableNetworks: [],
          availableOtherNetworks: [],
          availableSavedNetworks: [],
          allSavedNetworks: [],
          availableOtherNetworksLoading: false,
          availableSavedNetworksLoading: false,
        )) {
    _wirelessSub = wifiRepository.wirelessEnabledStream.listen((enabled) {
      logger.i('BLOC::Wireless enabled state changed: $enabled');
      if (enabled != state.wifiOn) {
        add(WifiEnabledChanged(enabled));
      }
    });

    // Event handlers
    on<InitWifi>(_onInit);
    on<InitializeWifi>(_onInitializeWifi);
    on<ToggleWifi>(_onToggleWifi);
    on<WifiEnabledChanged>(_onWifiEnabledChanged);
    on<WifiStatusChanged>(_onWifiStatusChanged); // connecting, connected, etc
    on<SelectNetwork>(_setSelectedNetwork);
    on<SelectNetworkPoint>(_setSelectedNetworkPoint);
    on<LoadNetworks>(_loadNetworks);

    on<ConnectSavedNetwork>(_connectSavedNetwork);
    on<ForgetNetwork>(onForgetNetwork);

    on<UpdateAvailableNetworksEvent>(_updateAvailableNetworkList);
    on<GetSavedNetworksEvent>(_getSavedNetworkList);

    on<UpdateConnectedNetworkEvent>(_updateConnectedNetwork);
    on<Error>(handleError);
    on<SelectedWirelessProtocol>(_selectWirelessProtocol);
    on<RefreshWifiList>(_onRefreshWifiList);
    on<UpdateNMDeviceState>(_onNetworkManagerDeviceStatusChanged);
  }

  Future<void> _onInit(
      InitWifi event, Emitter<WirelessSettingsState> emit) async {
    try {
      logger.i('BLOC:: Init WiFi Repo');
      await wifiRepository.init();
      add(InitializeWifi());
    } catch (e) {
      logger.e('Error  Init WiFi Repo: $e');
      // emit(WiFiError('Failed to initialize WiFi Client: $e'));
    }
  }

  Future<void> _onInitializeWifi(
      InitializeWifi event, Emitter<WirelessSettingsState> emit) async {
    logger.i("BLOC:: Initializing WiFi...");
    // Get wifi device
    final enabled = await wifiRepository.isWirelessEnabled();
    if (enabled) {
      add(WifiEnabledChanged(enabled));
      add(GetSavedNetworksEvent());
      add(LoadNetworks());
    }

    /// Get wired device
    final ethernetDevice = await wifiRepository.getWiredDevice();
    var wiredDevice = ethernetDevice.wired;

    var ethernetEnabled =
        ethernetDevice.state == NetworkManagerDeviceState.activated
            ? true
            : false;
    var info = WiredDevice(speed: wiredDevice!.speed, enabled: ethernetEnabled);

    emit(state.copyWith(wiredDevice: info));
  }

  Future<void> _loadNetworks(
      LoadNetworks event, Emitter<WirelessSettingsState> emit) async {
    try {
      final savedNetworks = await wifiRepository.getSavedNetworks();

      final result = await wifiRepository.availableAccessPoints(savedNetworks);

      emit(state.copyWith(
        availableOtherNetworks: result.available,
        connectedNetwork: result.active,
        wifiState:
            result.active == null ? WifiStatus.unknown : WifiStatus.connected,
      ));
    } catch (e) {
      logger.e('Error initializing wifi load networks $e, ');
    }
  }

  void _onWifiEnabledChanged(
      WifiEnabledChanged event, Emitter<WirelessSettingsState> emit) {
    emit(state.copyWith(wifiOn: event.enabled));

    if (event.enabled) {
      print("_onWifiEnabledChanged IF :: ${event.enabled}");
      _initializeWifiStream(); // for a network connection state
      _initializeAccessPointStream();
    } else {
      // On power off, cancel relevant the streams
      print("_onWifiEnabledChanged ELSE :: ${event.enabled}");

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

  Future<void> _initializeWifiStream() async {
    try {
      logger.i('Initializing WiFi event stream HERE...');
      final stream = await wifiRepository.streamWifiEvents();
      _wifiEventsSubscription = stream.listen((prop) async {
        if (prop.contains("State")) {
          final wifiState = await wifiRepository.getWifiState();
          final wifiDevice = await wifiRepository.getWifiDevice();

          if (wifiDevice.state != state.deviceState) {
            add(UpdateNMDeviceState(wifiDevice.state));
          }

          switch (wifiState) {
            case NetworkManagerState.connecting:
              add(WifiStatusChanged(WifiStatus.connecting));
              break;
            case NetworkManagerState.connectedGlobal:
              add(WifiStatusChanged(WifiStatus.connected));
              break; // Prevents fallthrough to the default case
            case NetworkManagerState.disconnecting:
              add(WifiStatusChanged(WifiStatus.disconnecting));
              break;
            case NetworkManagerState.disconnected:
              add(WifiStatusChanged(WifiStatus.disconnected));
              break;
            default:
              add(WifiStatusChanged(WifiStatus.unknown));
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
        if (prop.isNotEmpty &&
            (prop.contains("LastScan") ||
                prop.contains("AccessPoints") ||
                prop.contains("ActiveAccessPoint"))) {
          final savedNetworks = await wifiRepository.getSavedNetworks();

          final availAccessPoints =
              await wifiRepository.availableAccessPoints(savedNetworks);

          if (availAccessPoints.available.isNotEmpty) {
            add(UpdateAvailableNetworksEvent(availAccessPoints.available));
          }

          if (availAccessPoints.active != null) {
            add(UpdateConnectedNetworkEvent(availAccessPoints.active!));
          }
          // final savedNetworks =
          //     await wifiRepository.savedNetworks(state.availableOtherNetworks);

          // final availAccessPoints =
          //     await wifiRepository.availableAccessPoints(savedNetworks);

          // if (availAccessPoints.available.isNotEmpty) {
          //   add(UpdateAvailableNetworksEvent(availAccessPoints.available));
          // }

          // if (availAccessPoints.active != null) {
          //   add(UpdateConnectedNetworkEvent(availAccessPoints.active!));
          //   add(WifiStatusChanged(WifiStatus.connected));
          // }
        }
      });
    } catch (e, stackTrace) {
      logger.e('Error initializing wifi stream $e, $stackTrace');
    }
  }

  void _onNetworkManagerDeviceStatusChanged(
      UpdateNMDeviceState event, Emitter<WirelessSettingsState> emit) {
    emit(state.copyWith(deviceState: event.deviceState));
  }

  // Always cancel your subscriptions when Bloc is closed
  @override
  Future<void> close() async {
    logger.i('Closing WirelessSettingsBloc and cancelling subscriptions...');
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
    print('Toggling WiFi: ${event.enabled}');
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

  void _onWifiStatusChanged(
      WifiStatusChanged event, Emitter<WirelessSettingsState> emit) {
    emit(state.copyWith(wifiState: event.wifiStatus));
  }

  Future<void> handleError(
      Error event, Emitter<WirelessSettingsState> emit) async {
    emit(state.copyWith(error: event.error));
  }

  // Refresh saved networks list
  // after forgetting a network, get updated saved networks
  Future<void> onForgetNetwork(
      ForgetNetwork event, Emitter<WirelessSettingsState> emit) async {
    try {
      await wifiRepository.forgetNetwork(event.ssid);
      add(GetSavedNetworksEvent());
    } catch (e) {
      logger.e('Error in forgetting network: $e');
      emit(state.copyWith(error: e.toString()));
    }
  }

  Future<void> _connectSavedNetwork(
      ConnectSavedNetwork event, Emitter<WirelessSettingsState> emit) async {
    try {
      // TODO: handle active connection
      await wifiRepository.connectToSavedNetwork(event.nmAccessPoint);
      logger.i('Connected to saved network');
    } catch (e) {
      logger.e('Error connecting to saved network: $e');
      emit(state.copyWith(error: e.toString()));
    }
  }

  Future<void> _updateAvailableNetworkList(UpdateAvailableNetworksEvent event,
      Emitter<WirelessSettingsState> emit) async {
    try {
      emit(state.copyWith(
          availableOtherNetworksLoading: true,
          availableSavedNetworksLoading: true));

      final existingSSIDs = state.availableOtherNetworks
          .map((n) => utf8.decode(n.nmAccessPoint.ssid))
          .toSet();

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

      var availableOtherNetworks = [
        ...state.availableOtherNetworks,
        ...newNetworks
      ]
          // .where((network) => !network.isSaved) // Only include unsaved networks // For Showing saved and available network
          .toList();

      if (newNetworks.isNotEmpty) {
        emit(state.copyWith(availableOtherNetworks: availableOtherNetworks));
      }

      if (unavailableSSIDs.isNotEmpty) {
        final updatedNetworks = List<AccessPoints>.from(availableOtherNetworks);

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
          availableSavedNetworksLoading: false,
        ));
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

  Future<void> _onRefreshWifiList(
      RefreshWifiList event, Emitter<WirelessSettingsState> emit) async {
    try {
      if (!state.wifiOn) {
        emit(state.copyWith(
            error: "WiFi is turned off. Please enable WiFi first."));
        return;
      }

      logger.i('BLOC:: Refreshing WiFi list...');

      // Show loading state
      emit(state.copyWith(
        availableOtherNetworksLoading: true,
        availableSavedNetworksLoading: true,
        error: null,
      ));

      // Get updated access points
      final savedNetworks = await wifiRepository.getSavedNetworks();
      final availAccessPoints =
          await wifiRepository.availableAccessPoints(savedNetworks);

      if (availAccessPoints.available.isNotEmpty) {
        add(UpdateAvailableNetworksEvent(availAccessPoints.available));
      }

      if (availAccessPoints.active != null) {
        add(UpdateConnectedNetworkEvent(availAccessPoints.active!));
      }

      // Also update saved networks
      add(GetSavedNetworksEvent());

      logger.i('BLOC:: WiFi list refreshed successfully');

      emit(state.copyWith(
        availableOtherNetworksLoading: false,
        availableSavedNetworksLoading: false,
      ));
    } catch (e) {
      logger.e('Error refreshing WiFi list: $e');
      emit(state.copyWith(
        error: 'Failed to refresh WiFi list: ${e.toString()}',
        availableOtherNetworksLoading: false,
        availableSavedNetworksLoading: false,
      ));
    }
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

  void _selectWirelessProtocol(
      SelectedWirelessProtocol event, Emitter<WirelessSettingsState> emit) {
    emit(state.copyWith(selectedAPWirelessProtocol: event.protocol));
  }
}
