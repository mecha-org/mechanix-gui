import 'dart:async';

import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/web.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkEvent.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkState.dart';
import 'package:mechanix_settings/src/features/network/data/wifi_repository.dart';
import 'package:nm/nm.dart';

class ConnectNetworkBloc
    extends Bloc<ConnectNetworkEvent, ConnectNetworkState> {
  final logger = Logger();
  final WifiRepository wifiRepository;
  StreamSubscription? _wifiStateAndReason;

  ConnectNetworkBloc({required this.wifiRepository})
      : super(ConnectNetworkState()) {
    on<PasswordChanged>(passwordChanged);
    on<UsernameChanged>(usernameChanged);
    on<TogglePasswordVisibility>(togglePasswordVisibility);
    on<ConnectToNetwork>(connectToNetwork);
    on<ConnectToUnknownNetwork>(connectToUnknownNetwork);
    on<DeviceConnectionStateEvent>(_updateDeviceConnectionStateUpdate);

    _initWifiStateAndReasonStream();
  }

  Future<void> _initWifiStateAndReasonStream() async {
    try {
      final streamAndDevice = await wifiRepository.getWifiStateAndReason();

      _wifiStateAndReason =
          streamAndDevice.device.propertiesChanged.listen((event) {
        if (event.contains('StateReason')) {
          logger.i(
              "ConnectNetworkBloc STATE REASON: ${streamAndDevice.device.stateReason}");

          if (streamAndDevice.device.stateReason.state ==
                  NetworkManagerDeviceState.failed &&
              streamAndDevice.device.stateReason.reason ==
                  NetworkManagerDeviceStateReason.noSecrets) {
            add(Error("Authentication required!"));
          }
        }
      });
    } catch (e, stackTrace) {
      logger.e('Error initializing wifi stream $e, $stackTrace');
    }
  }

  Future<void> handleError(
      Error event, Emitter<ConnectNetworkState> emit) async {
    // logger.e("ERROR MESSAGE IN HANDLE ERROR: ${event.error}");
    emit(state.copyWith(error: event.error));
  }

  Future<void> passwordChanged(
      PasswordChanged event, Emitter<ConnectNetworkState> emit) async {
    emit(state.copyWith(password: event.password));
  }

  Future<void> usernameChanged(
      UsernameChanged event, Emitter<ConnectNetworkState> emit) async {
    emit(state.copyWith(username: event.username));
  }

  Future<void> togglePasswordVisibility(
      TogglePasswordVisibility event, Emitter<ConnectNetworkState> emit) async {
    emit(state.copyWith(obscurePassword: !state.obscurePassword));
  }

  Future<void> connectToNetwork(
      ConnectToNetwork event, Emitter<ConnectNetworkState> emit) async {
    emit(state.copyWith(isConnecting: true));
    // logger.i('Connecting to network with password: ${state.password}');
    try {
      await wifiRepository.connectToNetwork(event.accessPoint, state.password);
      emit(state.copyWith(isConnected: false, isConnecting: true));
    } catch (e) {
      logger.e('Failed to connect to network: $e');
      emit(state.copyWith(
          isConnected: false, isConnecting: false, error: e.toString()));
    }
    emit(state.copyWith(isConnecting: false));
  }

  Future<void> connectToUnknownNetwork(
      ConnectToUnknownNetwork event, Emitter<ConnectNetworkState> emit) async {
    emit(state.copyWith(isConnecting: true));
    // logger.i('Connecting to unknown network: ${event.ssid}');
    try {
      await wifiRepository.connectToUnknownNetwork(event.ssid, state.password);
      emit(state.copyWith(isConnected: true, isConnecting: false));
      // logger.i('Successfully connected to unknown network');
    } catch (e) {
      logger.e('Failed to connect to unknown network: $e');
      emit(state.copyWith(
          isConnected: false, isConnecting: false, error: e.toString()));
    }
    emit(state.copyWith(isConnecting: false));
  }

  Future<void> _updateDeviceConnectionStateUpdate(
      DeviceConnectionStateEvent event,
      Emitter<ConnectNetworkState> emit) async {
    emit(state.copyWith(deviceState: event.deviceSate));
  }

  @override
  Future<void> close() async {
    await _wifiStateAndReason?.cancel();
    _wifiStateAndReason = null;
    return super.close();
  }
}
