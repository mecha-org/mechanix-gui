import 'dart:async';

import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/web.dart';
import 'package:mechanix_settings/src/features/bluetooth/data/bluetooth_repository.dart';

import 'bluetooth_event.dart';
import 'bluetooth_state.dart';

class BluetoothBloc extends Bloc<BluetoothEvent, BluetoothState> {
  final BluetoothRepository bluetoothRepository;
  final logger = Logger();

  StreamSubscription? changeStream;

  BluetoothBloc({required this.bluetoothRepository})
      : super(BluetoothState(
          isPowered: false,
          loading: false,
          devices: [],
        )) {
    on<InitializeBluetooth>(_onInitializeBluetooth);
    on<ToggleBluetooth>(_onToggleBluetooth);

    on<StartDiscovery>(_onStartDiscovery);
    on<StopDiscovery>(_onStopDiscovery);
    on<RefreshDeviceList>(_onRefresh);
    on<PairDevice>(_onPair);
    on<ConnectDevice>(_onConnect);
    on<DisconnectDevice>(_onDisconnect);
    on<RemoveDevice>(_onRemoveDevice);
    on<SelectDevice>(_setSelectBluetoothDevice);
    on<GetAdapterAlias>(_onGetAdapterAlias);
    on<RenameAdapterEvent>(_onRenameAdapter);
    on<DiscoveryEnabled>(_setDeviceDiscoverable);
    on<CheckDeviceDiscoverable>(_isDeviceDiscoverable);
    _initializeBluetoothStream();
    _deviceAddedStream();
    _deviceRemovedStream();
  }

  Future<void> _initializeBluetoothStream() async {
    try {
      // checks adapter props changes
      final stream = await bluetoothRepository.streamBluetoothEvents();
      stream.listen((prop) async {
        logger.i("Bluetooth Property Update: $prop");

        if (!isClosed) {
          if (prop.contains("Discovering")) {
            add(InitializeBluetooth());
            // add(StartDiscovery());
          } else if (prop.contains("Alias")) {
            logger.w("Adapter alias changed ");
            add(GetAdapterAlias());
          }
        }
      });
    } catch (e, stackTrace) {
      logger.e('Error initializing bluetooth stream $e, $stackTrace ');
    }
  }

  Future<void> _deviceAddedStream() async {
    try {
      final stream = await bluetoothRepository.onDeviceAdded();
      changeStream = stream.listen((device) async {
        if (!isClosed) {
          logger.i("+++++++ Device added +++++++ : $device");
          add(RefreshDeviceList()); // device
        }
      });
    } catch (e, stackTrace) {
      logger.e('Error listening to device added events: $e, $stackTrace');
    }
  }

  Future<void> _deviceRemovedStream() async {
    try {
      final stream = await bluetoothRepository.onDeviceRemoved();
      changeStream = stream.listen((device) async {
        if (!isClosed) {
          logger.i("-------- Device removed -------- : $device");
          add(RefreshDeviceList());
        }
      });
    } catch (e, stackTrace) {
      logger.e('Error listening to device removed events: $e, $stackTrace');
    }
  }

  Future<void> _onInitializeBluetooth(
      InitializeBluetooth event, Emitter<BluetoothState> emit) async {
    final enabled = await bluetoothRepository.isBluetoothEnabled();
    emit(state.copyWith(isPowered: enabled));
    if (enabled) {
      add(RefreshDeviceList());
      add(GetAdapterAlias());
      add(CheckDeviceDiscoverable());
    }
  }

  Future<void> _onGetAdapterAlias(
      GetAdapterAlias event, Emitter<BluetoothState> emit) async {
    try {
      final adapterAlias = await bluetoothRepository.getAdapterAlias();
      emit(state.copyWith(adapterAlias: adapterAlias));
      logger.i('Get Adapter alias: $adapterAlias');
    } catch (e) {
      logger.e('Error getting adapter details: $e');
      emit(state.copyWith(error: e.toString()));
    }
  }

  Future<void> _onRenameAdapter(
      RenameAdapterEvent event, Emitter<BluetoothState> emit) async {
    try {
      logger.i('Renaming Bluetooth adapter to: ${event.newName}');
      await bluetoothRepository.setAdapterAlias(event.newName);
      if (!isClosed) {
        emit(state.copyWith(adapterAlias: event.newName));
        logger.i('Bluetooth adapter renamed successfully');
      }
    } catch (e) {
      logger.e('Error renaming Bluetooth adapter: $e');
      emit(state.copyWith(error: e.toString()));
    }
  }

  Future<void> _onStartDiscovery(
      StartDiscovery event, Emitter<BluetoothState> emit) async {
    try {
      logger.i('Starting Bluetooth discovery');
      await bluetoothRepository.startDiscovery();
      emit(state.copyWith(loading: true));
      add(RefreshDeviceList());
      // Future.delayed(Duration(seconds: 15), () async {
      //   add(StopDiscovery());
      // });
    } catch (e) {
      logger.e('Error starting discovery: $e');
      emit(state.copyWith(error: e.toString(), loading: false));
    }
  }

  Future<void> _onStopDiscovery(
      StopDiscovery event, Emitter<BluetoothState> emit) async {
    try {
      logger.i('Stopping Bluetooth discovery');
      await bluetoothRepository.stopDiscovery();
      emit(state.copyWith(loading: false));
    } catch (e) {
      logger.e('Error stopping discovery: $e');
      emit(state.copyWith(error: e.toString(), loading: false));
    }
  }

  Future<void> _onRefresh(
    RefreshDeviceList event,
    Emitter<BluetoothState> emit,
  ) async {
    logger.i("Loading available Bluetooth devices...");
    // emit(state.copyWith(loading: true));
    try {
      var allDevices = await bluetoothRepository.getDevices();

      var validDevices = allDevices
          .where((d) => d.name.isNotEmpty && d.address.isNotEmpty)
          .toList();

      emit(state.copyWith(
        devices: validDevices,
      ));
    } catch (e, stack) {
      logger.e('Error refreshing device list', error: e, stackTrace: stack);
      emit(state.copyWith(
        error: e.toString(),
      ));
    }
  }

  Future<void> _onPair(PairDevice event, Emitter<BluetoothState> emit) async {
    try {
      logger.i('Pairing device with address: ${event.address}');

      await bluetoothRepository.pair(event.address);
      logger.i('Device paired successfully: ${event.address}');
    } catch (e) {
      logger.e('Error pairing device: $e');
      emit(state.copyWith(error: e.toString()));
    }
  }

  Future<void> _onConnect(
      ConnectDevice event, Emitter<BluetoothState> emit) async {
    try {
      logger.i('Connecting to device with address: ${event.address}');
      await bluetoothRepository.connect(event.address);
      add(RefreshDeviceList());
      logger.i('Device connected successfully: ${event.address}');
    } catch (e) {
      logger.e('Error connecting to device: $e');
      emit(state.copyWith(error: e.toString()));
    }
  }

  Future<void> _onDisconnect(
      DisconnectDevice event, Emitter<BluetoothState> emit) async {
    try {
      logger.i('Disconnecting from device with address: ${event.address}');
      await bluetoothRepository.disconnect(event.address);
      logger.i('Device disconnected successfully: ${event.address}');
    } catch (e) {
      logger.e('Error disconnecting from device: $e');
      emit(state.copyWith(error: e.toString()));
    }
  }

  Future<void> _onRemoveDevice(
      RemoveDevice event, Emitter<BluetoothState> emit) async {
    try {
      logger.i('Removing device with address: ${event.address}');
      await bluetoothRepository.remove(event.address);
      logger.i('Device removed successfully: ${event.address}');
    } catch (e) {
      logger.e('Error removing device: $e');
      emit(state.copyWith(error: e.toString()));
    }
  }

  void _onToggleBluetooth(
      ToggleBluetooth event, Emitter<BluetoothState> emit) async {
    logger.i('Toggling Bluetooth: ${event.enabled}');
    try {
      await bluetoothRepository.setPower(event.enabled);
      if (event.enabled) {
        add(StartDiscovery());
        add(GetAdapterAlias());
        _initializeBluetoothStream();
      }
      emit(state.copyWith(isPowered: event.enabled));
      logger.i('Bluetooth toggled successfully: ${event.enabled}');
    } catch (e) {
      logger.e('Error toggling Bluetooth: $e');
      emit(state.copyWith(error: e.toString()));
      return;
    }
  }

  Future<void> _setSelectBluetoothDevice(
      SelectDevice event, Emitter<BluetoothState> emit) async {
    emit(state.copyWith(selectedDevice: event.selectedDevice));
  }

  void _isDeviceDiscoverable(
      CheckDeviceDiscoverable event, Emitter<BluetoothState> emit) async {
    final enabled = await bluetoothRepository.isDeviceDiscoverable();
    emit(state.copyWith(isDiscoveryEnabled: enabled));
  }

  void _setDeviceDiscoverable(
      DiscoveryEnabled event, Emitter<BluetoothState> emit) async {
    await bluetoothRepository.setDiscoverable(event.isDiscoverable);
    emit(state.copyWith(isDiscoveryEnabled: event.isDiscoverable));
  }

  @override
  Future<void> close() {
    changeStream?.cancel();
    return super.close();
  }
}
