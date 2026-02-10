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
    on<DeActivatedNetworkEvent>(_onDeActivatedNetwork);
    on<UpdateSavedNetworkList>(_updateSavedNetworkList);
    on<UpdateUnknownNetworkList>(_updateUnknownNetworkList);
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

  Future<void> _onDeActivatedNetwork(DeActivatedNetworkEvent event,
      Emitter<WirelessSettingsState> emit) async {
    final isSameNetwork =
        listEquals(state.activatingNetwork?.ssid, event.activatingNetwork.ssid);

    // remove activating network when network is de-activated
    if (isSameNetwork) {
      add(ActivatingNetworkEvent(const ActivatingNetwork(
        ssid: [],
        isActivate: false,
        accessPoint: null,
        deviceState: NetworkManagerActiveConnectionState.unknown,
      )));
    }

    emit(state.copyWith(deActivatedNetwork: event.activatingNetwork));
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

          // when network connection failed
          if (wifiDevice.state == NetworkManagerDeviceState.failed) {
            add(
              ActivatingNetworkEvent(
                const ActivatingNetwork(
                  deviceState: NetworkManagerActiveConnectionState.deactivated,
                  isActivate: false,
                  accessPoint: null,
                  ssid: [],
                ),
              ),
            );

            final failedNetwork = [
              ...state.availableOtherNetworks,
              ...state.availableOtherNetworks
            ].firstWhereOrNull((sn) =>
                utf8.decode(sn.nmAccessPoint.ssid) ==
                wifiDevice.activeConnection?.id);

            // when network failed remove from saved network
            if (failedNetwork != null) {
              await wifiRepository.deleteSavedNetwork(
                failedNetwork.nmAccessPoint,
              );
            }
            final savedNetworks = await wifiRepository.getSavedNetworks();

            final availAccessPoints =
                await wifiRepository.availableAccessPoints(savedNetworks);

            final savedAccessPoints = availAccessPoints.available
                .where((ap) => ap.isSaved && !ap.isActive)
                .toList();

            add(UpdateSavedNetworkList(savedAccessPoints));
          }

          // get currently activate connection list
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

              if (conn != null) {
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

                  final unSavedAccessPoints = availAccessPoints.available
                      .where((ap) => !ap.isSaved && !ap.isActive)
                      .toList();

                  add(UpdateUnknownNetworkList(unSavedAccessPoints));
                }

                if (connection.state ==
                    NetworkManagerActiveConnectionState.activated) {
                  if (availAccessPoints.active != null) {
                    add(UpdateConnectedNetworkEvent(availAccessPoints.active!));

                    final isActivateFromUnsavedList =
                        state.availableOtherNetworks.any((sn) =>
                            utf8.decode(sn.nmAccessPoint.ssid) ==
                            connection.id);

                    if (isActivateFromUnsavedList) {
                      final unSavedAccessPoints = availAccessPoints.available
                          .where((ap) => !ap.isSaved && !ap.isActive)
                          .toList();

                      add(UpdateUnknownNetworkList(unSavedAccessPoints));
                    } else {
                      final savedAccessPoints = availAccessPoints.available
                          .where((ap) => ap.isSaved && !ap.isActive)
                          .toList();

                      add(UpdateSavedNetworkList(savedAccessPoints));
                    }
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

                if (connection.state ==
                    NetworkManagerActiveConnectionState.deactivated) {
                  add(
                    DeActivatedNetworkEvent(
                      ActivatingNetwork(
                        deviceState:
                            NetworkManagerActiveConnectionState.deactivated,
                        isActivate: false,
                        ssid: conn.nmAccessPoint.ssid,
                      ),
                    ),
                  );
                }
              }
            }
          }

          if (wifiDevice.state != state.deviceState) {
            add(UpdateNMDeviceState(wifiDevice.state));
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
        // print("----- _initializeAccessPointStream ----- $prop");
        if (prop.isNotEmpty && (prop.contains("AccessPoints"))) {
          final savedNetworks = await wifiRepository.getSavedNetworks();

          final availAccessPoints =
              await wifiRepository.availableAccessPoints(savedNetworks);

          final List<AccessPoints> allAccessPoints = [
            ...availAccessPoints.available,
          ];

          if (availAccessPoints.active != null) {
            allAccessPoints.add(availAccessPoints.active!);
          }

          List<AccessPoints> updatedAccessPoints = allAccessPoints;

          if (updatedAccessPoints.isNotEmpty) {
            final savedAccessPoints = updatedAccessPoints
                .where((ap) => ap.isSaved && !ap.isActive)
                .toList();

            add(UpdateSavedNetworkList(savedAccessPoints));

            final unKnownAccessPoints = updatedAccessPoints
                .where((ap) => !ap.isSaved && !ap.isActive)
                .toList();

            add(UpdateUnknownNetworkList(unKnownAccessPoints));
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
      await wifiRepository.forgetNetwork(event.ssid);

      final updatedNetworks = state.availableSavedNetworks
          .where((sn) => utf8.decode(sn.nmAccessPoint.ssid) != event.ssid)
          .toList();

      if (updatedNetworks.length != state.availableSavedNetworks.length) {
        emit(state.copyWith(availableSavedNetworks: updatedNetworks));
      }

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

  Future<void> _updateSavedNetworkList(
      UpdateSavedNetworkList event, Emitter<WirelessSettingsState> emit) async {
    emit(state.copyWith(availableSavedNetworksLoading: true));
    final Set<String> oldSavedAvailableSSids = state.availableSavedNetworks
        .map((ap) => utf8.decode(ap.nmAccessPoint.ssid))
        .toSet();

    final Set<String> newSavedAvailableSSids = event.accessPoint
        .map((ap) => utf8.decode(ap.nmAccessPoint.ssid))
        .toSet();

    final Set<String> existedNetworksSSids = oldSavedAvailableSSids
        .where((ssid) => newSavedAvailableSSids.contains(ssid))
        .toSet();

    final Set<String> newAvailableSSids = newSavedAvailableSSids
        .where((ssid) => !oldSavedAvailableSSids.contains(ssid))
        .toSet();

    final existedAccessPoints = event.accessPoint
        .where((ap) =>
            existedNetworksSSids.contains(utf8.decode(ap.nmAccessPoint.ssid)))
        .toList();

    List<AccessPoints> updatedAccessPoints = [];

    if (newAvailableSSids.isNotEmpty) {
      final newUnknownAccessPoints = event.accessPoint
          .where((ap) =>
              newAvailableSSids.contains(utf8.decode(ap.nmAccessPoint.ssid)))
          .toList();

      updatedAccessPoints = [...existedAccessPoints, ...newUnknownAccessPoints];
    } else {
      updatedAccessPoints = existedAccessPoints;
    }

    final isConnectingNetworkInSavesNetwork = updatedAccessPoints.any(
        (n) => listEquals(n.nmAccessPoint.ssid, state.activatingNetwork?.ssid));

    if (isConnectingNetworkInSavesNetwork) {
      updatedAccessPoints = updatedAccessPoints
          .where((ap) =>
              !listEquals(ap.nmAccessPoint.ssid, state.activatingNetwork?.ssid))
          .toList();
    }

    emit(state.copyWith(
      availableSavedNetworks: updatedAccessPoints,
      availableSavedNetworksLoading: false,
    ));
  }

  Future<void> _updateUnknownNetworkList(UpdateUnknownNetworkList event,
      Emitter<WirelessSettingsState> emit) async {
    emit(state.copyWith(availableOtherNetworksLoading: true));

    final Set<String> oldUnknownAvailableSSids = state.availableOtherNetworks
        .map((ap) => utf8.decode(ap.nmAccessPoint.ssid))
        .toSet();

    final Set<String> newUnknownAvailableSSids = event.accessPoint
        .map((ap) => utf8.decode(ap.nmAccessPoint.ssid))
        .toSet();

    final Set<String> existedNetworksSSids = oldUnknownAvailableSSids
        .where((ssid) => newUnknownAvailableSSids.contains(ssid))
        .toSet();

    final Set<String> newAvailableSSids = newUnknownAvailableSSids
        .where((ssid) => !oldUnknownAvailableSSids.contains(ssid))
        .toSet();

    final existedAccessPoints = event.accessPoint
        .where((ap) =>
            existedNetworksSSids.contains(utf8.decode(ap.nmAccessPoint.ssid)))
        .toList();

    List<AccessPoints> updatedAccessPoints = [];
    if (newAvailableSSids.isNotEmpty) {
      final newUnknownAccessPoints = event.accessPoint
          .where((ap) =>
              newAvailableSSids.contains(utf8.decode(ap.nmAccessPoint.ssid)))
          .toList();

      updatedAccessPoints = [...existedAccessPoints, ...newUnknownAccessPoints];
    } else {
      updatedAccessPoints = existedAccessPoints;
    }

    final isConnectingNetworkInSavesNetwork = updatedAccessPoints.any(
        (n) => listEquals(n.nmAccessPoint.ssid, state.activatingNetwork?.ssid));

    if (isConnectingNetworkInSavesNetwork) {
      updatedAccessPoints = updatedAccessPoints
          .where((ap) =>
              !listEquals(ap.nmAccessPoint.ssid, state.activatingNetwork?.ssid))
          .toList();
    }

    emit(state.copyWith(
      availableOtherNetworks: updatedAccessPoints,
      availableOtherNetworksLoading: false,
    ));
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

      // update saved network and unsaved network list when network is updated
      final isSavedNetworkIsActive =
          state.availableSavedNetworks.any((network) {
        return utf8.decode(network.nmAccessPoint.ssid) ==
            utf8.decode(event.accessPoint.nmAccessPoint.ssid);
      });

      if (isSavedNetworkIsActive) {
        final updatedSavedNetworks = state.availableSavedNetworks
            .where((network) =>
                utf8.decode(network.nmAccessPoint.ssid) !=
                utf8.decode(event.accessPoint.nmAccessPoint.ssid))
            .toList();
        emit(state.copyWith(availableSavedNetworks: updatedSavedNetworks));
      } else {
        final updatedOtherNetworks = state.availableOtherNetworks
            .where((network) =>
                utf8.decode(network.nmAccessPoint.ssid) !=
                utf8.decode(event.accessPoint.nmAccessPoint.ssid))
            .toList();
        emit(state.copyWith(availableOtherNetworks: updatedOtherNetworks));
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

      print('BLOC:: Refreshing WiFi list...');

      emit(state.copyWith(
        availableOtherNetworksLoading: true,
        availableSavedNetworksLoading: true,
        error: null,
      ));

      final savedNetworks = await wifiRepository.getSavedNetworks();

      final availAccessPoints =
          await wifiRepository.availableAccessPoints(savedNetworks);

      if (availAccessPoints.available.isNotEmpty) {
        final savedAccessPoints = availAccessPoints.available
            .where((ap) => ap.isSaved && !ap.isActive)
            .toList();

        add(UpdateSavedNetworkList(savedAccessPoints));

        final unKnownAccessPoints = availAccessPoints.available
            .where((ap) => !ap.isSaved && !ap.isActive)
            .toList();

        add(UpdateUnknownNetworkList(unKnownAccessPoints));
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
