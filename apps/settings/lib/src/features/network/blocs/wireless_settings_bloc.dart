import 'dart:async';
import 'dart:convert';

import 'package:collection/collection.dart';
import 'package:flutter/foundation.dart';
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

    on<GetSavedNetworksEvent>(_getSavedNetworkList);

    on<UpdateConnectedNetworkEvent>(_updateConnectedNetwork);
    on<Error>(handleError);
    on<SelectedWirelessProtocol>(_selectWirelessProtocol);
    on<RefreshWifiList>(_onRefreshWifiList);
    on<UpdateNMDeviceState>(_onNetworkManagerDeviceStatusChanged);
    on<ActivatingNetworkEvent>(_onActivatingNetwork);
    on<ActivatedNetworkEvent>(_onActivatedNetwork);
    on<UpdateSavedNetworkList>(_updateSavedNetworkList);
    on<UpdateAvailableNetworkList>(_updateAvailableNetworkList);
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
      Future.delayed(const Duration(milliseconds: 500), () {
        add(GetSavedNetworksEvent());
        add(LoadNetworks());
      });
      final wifiDevice = await wifiRepository.getWifiDevice();
      if (wifiDevice.state != state.deviceState) {
        add(UpdateNMDeviceState(wifiDevice.state));
      }
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
      emit(state.copyWith(
        availableOtherNetworksLoading: true,
        availableSavedNetworksLoading: true,
      ));
      final savedNetworks = await wifiRepository.getSavedNetworks();

      final result = await wifiRepository.availableAccessPoints(savedNetworks);

      final List<AccessPoints> availableNotSavedNetworks = [];
      final List<AccessPoints> availableSavedNetworks = [];

      if (result.available.isNotEmpty) {
        for (var network in result.available) {
          if (!network.isActive) {
            if (network.isSaved) {
              availableSavedNetworks.add(network);
            } else if (!network.isSaved) {
              availableNotSavedNetworks.add(network);
            }
          }
        }
      }

      emit(state.copyWith(
        connectedNetwork: result.active,
        availableSavedNetworks: availableSavedNetworks,
        availableOtherNetworks: availableNotSavedNetworks,
        availableOtherNetworksLoading: false,
        availableSavedNetworksLoading: false,
        wifiState:
            result.active == null ? WifiStatus.unknown : WifiStatus.connected,
      ));
    } catch (e) {
      print(" load networks error $e");
      logger.e('Error initializing wifi load networks $e, ');
    }
  }

  Future<void> _onActivatingNetwork(
      ActivatingNetworkEvent event, Emitter<WirelessSettingsState> emit) async {
    emit(state.copyWith(activatingNetwork: event.activatingNetwork));
  }

  Future<void> _onActivatedNetwork(
      ActivatedNetworkEvent event, Emitter<WirelessSettingsState> emit) async {
    final isSameNetwork =
        listEquals(state.activatingNetwork?.ssid, event.activatingNetwork.ssid);

    // remove activating network when network is activated
    if (isSameNetwork) {
      add(ActivatingNetworkEvent(const ActivatingNetwork(
        ssid: [],
        isActivate: false,
        accessPoint: null,
        deviceState: NetworkManagerActiveConnectionState.unknown,
      )));
    }

    emit(state.copyWith(activatedNetwork: event.activatingNetwork));
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

          if (wifiDevice.state == NetworkManagerDeviceState.failed) {
            add(ActivatingNetworkEvent(const ActivatingNetwork(
              ssid: [],
              isActivate: false,
              accessPoint: null,
              deviceState: NetworkManagerActiveConnectionState.unknown,
            )));
          }

          final connections = await wifiRepository.activatingConnection();
          final savedNetworks = await wifiRepository.getSavedNetworks();

          final availAccessPoints =
              await wifiRepository.availableAccessPoints(savedNetworks);
          if (connections.isNotEmpty) {
            for (var connection in connections) {
              List<AccessPoints> networks = [...availAccessPoints.available];

              if (availAccessPoints.active != null) {
                networks.add(availAccessPoints.active!);
              }

              final conn = networks.firstWhereOrNull(
                  (sn) => utf8.decode(sn.nmAccessPoint.ssid) == connection.id);

              if (conn != null &&
                  (connection.state ==
                          NetworkManagerActiveConnectionState.activating ||
                      connection.state ==
                          NetworkManagerActiveConnectionState.activated)) {
                if (connection.state ==
                    NetworkManagerActiveConnectionState.activating) {
                  add(
                    ActivatingNetworkEvent(
                      ActivatingNetwork(
                        deviceState:
                            NetworkManagerActiveConnectionState.activating,
                        isActivate: false,
                        accessPoint: conn,
                        ssid: conn.nmAccessPoint.ssid,
                      ),
                    ),
                  );

                  final savedAccessPoints = availAccessPoints.available
                      .where((ap) => ap.isSaved && !ap.isActive)
                      .toList();

                  add(UpdateSavedNetworkList(savedAccessPoints));

                  final availableAccessPoints = availAccessPoints.available
                      .where((ap) => !ap.isSaved && !ap.isActive)
                      .toList();

                  add(UpdateAvailableNetworkList(availableAccessPoints));
                }

                if (connection.state ==
                    NetworkManagerActiveConnectionState.activated) {
                  add(UpdateConnectedNetworkEvent(availAccessPoints.active!));

                  if (availAccessPoints.available.isNotEmpty) {
                    final savedAccessPoints = <AccessPoints>[];
                    final availableAccessPoints = <AccessPoints>[];

                    for (final ap in availAccessPoints.available) {
                      if (ap.isActive) continue;

                      if (ap.isSaved) {
                        savedAccessPoints.add(ap);
                      } else {
                        availableAccessPoints.add(ap);
                      }
                    }

                    add(UpdateSavedNetworkList(savedAccessPoints));

                    add(UpdateAvailableNetworkList(availableAccessPoints));
                  }
                  add(
                    ActivatedNetworkEvent(
                      ActivatingNetwork(
                        deviceState:
                            NetworkManagerActiveConnectionState.activated,
                        isActivate: false,
                        ssid: conn.nmAccessPoint.ssid,
                      ),
                    ),
                  );
                }
              }
            }
          }
          switch (wifiState) {
            case NetworkManagerState.connecting:
              add(WifiStatusChanged(WifiStatus.connecting));
              break;
            case NetworkManagerState.connectedGlobal:
              add(WifiStatusChanged(WifiStatus.connected));
              break;
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
        if (prop.isNotEmpty && (prop.contains("AccessPoints"))) {
          final savedNetworks = await wifiRepository.getSavedNetworks();

          final availAccessPoints =
              await wifiRepository.availableAccessPoints(savedNetworks);

          if (availAccessPoints.available.isNotEmpty) {
            final savedAccessPoints = <AccessPoints>[];
            final availableAccessPoints = <AccessPoints>[];

            for (final ap in availAccessPoints.available) {
              if (ap.isActive) continue;

              if (ap.isSaved) {
                savedAccessPoints.add(ap);
              } else {
                availableAccessPoints.add(ap);
              }
            }

            add(UpdateSavedNetworkList(savedAccessPoints));

            add(UpdateAvailableNetworkList(availableAccessPoints));
          }
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
      emit(state.copyWith(wifiOn: event.enabled));
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
      final updatedSavedNetworks = state.availableSavedNetworks
          .where((sn) => utf8.decode(sn.nmAccessPoint.ssid) != event.ssid)
          .toList();

      final updatedAllNetworks =
          state.allSavedNetworks.where((sn) => sn.ssid != event.ssid).toList();

      if (updatedSavedNetworks.length != state.availableSavedNetworks.length) {
        emit(state.copyWith(
          availableSavedNetworks: updatedSavedNetworks,
          allSavedNetworks: updatedAllNetworks,
        ));
      }
      await wifiRepository.forgetNetwork(event.ssid);

      add(GetSavedNetworksEvent());
      add(LoadNetworks());
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

  List<AccessPoints> _mergeAccessPoints({
    required List<AccessPoints> oldNetworks,
    required List<AccessPoints> newNetworks,
    required List<int>? activatingSsid,
  }) {
    try {
      final Map<String, AccessPoints> oldBySsid = {
        for (final ap in oldNetworks) utf8.decode(ap.nmAccessPoint.ssid): ap,
      };

      final List<AccessPoints> mergedNetworkList = [];

      for (final ap in newNetworks) {
        final ssidBytes = ap.nmAccessPoint.ssid;
        final ssid = utf8.decode(ssidBytes);

        final old = oldBySsid[ssid];

        if (activatingSsid != null && listEquals(ssidBytes, activatingSsid)) {
          continue;
        }

        if (old != null &&
            listEquals(old.nmAccessPoint.ssid, ap.nmAccessPoint.ssid)) {
          mergedNetworkList.add(old);
        } else {
          mergedNetworkList.add(ap);
        }
      }

      return mergedNetworkList;
    } catch (e) {
      print("Error in merge access points $e");
      return [];
    }
  }

  Future<void> _updateSavedNetworkList(
    UpdateSavedNetworkList event,
    Emitter<WirelessSettingsState> emit,
  ) async {
    try {
      final updatedNetworks = _mergeAccessPoints(
        oldNetworks: state.availableSavedNetworks,
        newNetworks: event.accessPoint,
        activatingSsid: state.activatingNetwork?.ssid,
      );

      if (listEquals(updatedNetworks, state.availableSavedNetworks)) {
        return;
      }

      emit(state.copyWith(
        availableSavedNetworks: updatedNetworks,
      ));
    } catch (e) {
      print("Error in update saved network $e");
    }
  }

  Future<void> _updateAvailableNetworkList(
    UpdateAvailableNetworkList event,
    Emitter<WirelessSettingsState> emit,
  ) async {
    try {
      final updatedNetworks = _mergeAccessPoints(
        oldNetworks: state.availableOtherNetworks,
        newNetworks: event.accessPoint,
        activatingSsid: state.activatingNetwork?.ssid,
      );

      if (listEquals(updatedNetworks, state.availableOtherNetworks)) {
        return;
      }

      emit(state.copyWith(
        availableOtherNetworks: updatedNetworks,
      ));
    } catch (e) {
      print("Error in update saved network $e");
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

      if (!listEquals(event.accessPoint.nmAccessPoint.ssid,
          state.connectedNetwork!.nmAccessPoint.ssid)) {
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

      print('BLOC:: Refreshing WiFi list...');

      emit(state.copyWith(
        availableOtherNetworksLoading: true,
        availableSavedNetworksLoading: true,
        error: null,
      ));

      await Future.delayed(const Duration(milliseconds: 500));

      final savedNetworks = await wifiRepository.getSavedNetworks();

      final availAccessPoints =
          await wifiRepository.availableAccessPoints(savedNetworks);

      if (availAccessPoints.available.isNotEmpty) {
        final savedAccessPoints = availAccessPoints.available
            .where((ap) => ap.isSaved && !ap.isActive)
            .toList();

        add(UpdateSavedNetworkList(savedAccessPoints));

        final availableAccessPoints = availAccessPoints.available
            .where((ap) => !ap.isSaved && !ap.isActive)
            .toList();

        add(UpdateAvailableNetworkList(availableAccessPoints));
      }

      if (availAccessPoints.active != null) {
        add(UpdateConnectedNetworkEvent(availAccessPoints.active!));
      }

      add(GetSavedNetworksEvent());

      print('BLOC:: WiFi list refreshed successfully');

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
