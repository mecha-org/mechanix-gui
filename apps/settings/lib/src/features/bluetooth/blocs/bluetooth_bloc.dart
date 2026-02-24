import 'dart:async';

import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/web.dart';
import 'package:mechanix_settings/src/features/bluetooth/data/bluetooth_repository.dart';
import 'package:mechanix_settings/src/features/bluetooth/models/bluetooth_device_classifier.dart';
import 'package:mechanix_settings/src/features/bluetooth/models/types.dart';

import 'bluetooth_event.dart';
import 'bluetooth_state.dart';

class BluetoothBloc extends Bloc<BluetoothEvent, BluetoothState> {
  final BluetoothRepository bluetoothRepository;
  final logger = Logger();

  StreamSubscription<bool>? _adapterPowerSubscription;
  StreamSubscription<bool>? _adapterDiscoverableSubscription;
  StreamSubscription? _bluetoothDeviceAddedStream;
  StreamSubscription? _bluetoothDeviceRemovedStream;

  BluetoothBloc({required this.bluetoothRepository})
      : super(const BluetoothState(
          isPowered: false,
          loading: false,
          devices: [],
        )) {
    _adapterPowerSubscription =
        bluetoothRepository.bluezAdapterPowerStream.listen((isPowered) {
      if (isPowered != state.isPowered) {
        add(BluetoothPowerChanged(isPowered));
      }
    });

    _adapterDiscoverableSubscription =
        bluetoothRepository.bluezAdapterDiscoverableStream.listen((prop) {
      add(DiscoverableChanged(prop));
    });

    // Event handlers
    on<InitBluetooth>(_onInit);
    on<InitializeBluetooth>(_onInitializeBluetooth);
    on<ToggleBluetooth>(_onToggleBluetooth);
    on<BluetoothPowerChanged>(_onBluetoothPowerChanged);

    on<GetAdapterDetails>(_onGetAdapterDetails);
    // on<StartDiscovery>(_onStartDiscovery);
    // on<StopDiscovery>(_onStopDiscovery);
    on<RefreshDeviceList>(_onGetRefreshDeviceList);

    on<PairDevice>(_onPair);
    on<ConnectDevice>(_onConnect);
    on<DisconnectDevice>(_onDisconnect);
    on<RemoveDevice>(_onRemoveDevice);
    on<SelectDevice>(_setSelectBluetoothDevice);

    on<DiscoveryEnabled>(_setDeviceDiscoverable);
    on<DiscoverableChanged>(_onDiscoverableChanged);
    on<GetDeviceList>(_onGetDeviceList);

    // on<RenameAdapterEvent>(_onRenameAdapter);
    on<BluetoothDevicesAdded>(_addedBluetoothDevices);
    on<BluetoothDevicesRemoved>(_removedBluetoothDevices);
    on<BluetoothConnectingEvent>(_onBluetoothConnectionLoading);
    on<BluetoothDevicesUpdate>(_updateBluetoothDevices);
  }

  Future<void> _onInit(
      InitBluetooth event, Emitter<BluetoothState> emit) async {
    try {
      logger.i('BLOC:: Initializing Bluetooth');
      await bluetoothRepository.init();
      add(InitializeBluetooth());
    } catch (e) {
      logger.e('Error initializing Bluetooth: $e');
      // emit(BluetoothError('Failed to initialize Bluetooth: $e'));
    }
  }

  Future<void> _onInitializeBluetooth(
      InitializeBluetooth event, Emitter<BluetoothState> emit) async {
    logger.i('BLOC:: Initializing Bluetooth - GET & SET POWER');
    final enabled = await bluetoothRepository.isBluetoothEnabled();
    final adapter = await bluetoothRepository.getBluezAdapter();
    emit(state.copyWith(isDiscoveryEnabled: adapter.discoverable));
    if (enabled) {
      add(BluetoothPowerChanged(enabled));
    }
  }

  // Note: when power is on, we need to get adapter details
  Future<void> _onBluetoothPowerChanged(
    BluetoothPowerChanged event,
    Emitter<BluetoothState> emit,
  ) async {
    logger.i('BLOC:: Bluetooth power changed: ${event.enabled}');
    emit(state.copyWith(isPowered: event.enabled));
    if (event.enabled) {
      emit(state.copyWith(loading: true));
      add(GetAdapterDetails());
      add(RefreshDeviceList());
      add(GetDeviceList());

      _deviceAddedStream();
      _deviceRemovedStream();
    } else {
      print("Bluetooth power OFF");
      emit(state.copyWith(devices: [], loading: false));
      _bluetoothDeviceAddedStream?.cancel();
      _bluetoothDeviceRemovedStream?.cancel();
    }
  }

  Future<void> _onGetAdapterDetails(
      GetAdapterDetails event, Emitter<BluetoothState> emit) async {
    try {
      final adapter = await bluetoothRepository.getBluezAdapter();
      final BluetoothAdapter bluetoothAdapter = BluetoothAdapter(
          name: adapter.name,
          alias: adapter.alias,
          powered: adapter.powered,
          discovering: adapter.discovering,
          discoverable: adapter.discoverable);
      emit(state.copyWith(bluetoothAdapter: bluetoothAdapter));
      logger.i('Get Adapter details: $adapter');
    } catch (e) {
      logger.e('Error getting adapter details: $e');
      emit(state.copyWith(error: e.toString()));
    }
  }

  void _onToggleBluetooth(
      ToggleBluetooth event, Emitter<BluetoothState> emit) async {
    logger.i('BLOC:: Toggling Bluetooth: ${event.enabled}');
    try {
      await bluetoothRepository.setPower(event.enabled);
    } catch (e) {
      logger.e('BLOC:: Error toggling Bluetooth: $e');
      // Error toggling Bluetooth: org.bluez.Error.Failed: Failed
      emit(state.copyWith(error: e.toString()));
      return;
    }
  }

  // Remove after testing
  Future<void> _deviceAddedStream() async {
    try {
      print(" Device added stream started :");
      final stream = await bluetoothRepository.onDeviceAdded();
      _bluetoothDeviceAddedStream = stream.listen((device) {
        print("+++++++ Device added +++++++ : ${device.alias}");

        add(BluetoothDevicesAdded(device));
      });
    } catch (e, stackTrace) {
      logger.e('Error listening to device added events: $e, $stackTrace');
    }
  }

  // Remove after testing
  Future<void> _deviceRemovedStream() async {
    try {
      print(" Device remove stream started :");
      final stream = await bluetoothRepository.onDeviceRemoved();
      _bluetoothDeviceRemovedStream = stream.listen((device) {
        if (!isClosed) {
          print("-------- Device removed -------- : ${device.name}");

          add(BluetoothDevicesRemoved(device.address));
        }
      });
    } catch (e, stackTrace) {
      logger.e('Error listening to device removed events: $e, $stackTrace');
    }
  }

  // Future<void> _onRenameAdapter(
  //     RenameAdapterEvent event, Emitter<BluetoothState> emit) async {
  //   try {
  //     logger.i('Renaming Bluetooth adapter to: ${event.newName}');
  //     await bluetoothRepository.setAdapterAlias(event.newName);
  //     if (!isClosed) {
  //       emit(state.copyWith(adapterAlias: event.newName));
  //       logger.i('Bluetooth adapter renamed successfully');
  //     }
  //   } catch (e) {
  //     logger.e('Error renaming Bluetooth adapter: $e');
  //     emit(state.copyWith(error: e.toString()));
  //   }
  // }

  // Future<void> _onStartDiscovery(
  //     StartDiscovery event, Emitter<BluetoothState> emit) async {
  //   try {
  //     logger.i('BLOC:: Starting Bluetooth discovery');
  //     emit(state.copyWith(loading: true));
  //     await bluetoothRepository.startDiscovery();
  //   } catch (e) {
  //     logger.e('BLOC:: Error starting discovery: $e');
  //     emit(state.copyWith(error: e.toString(), loading: false));
  //   }
  // }

  // //  after stopping discovery get devices
  // Future<void> _onStopDiscovery(
  //     StopDiscovery event, Emitter<BluetoothState> emit) async {
  //   try {
  //     logger.i('BLOC:: Stopping Bluetooth discovery');
  //     await bluetoothRepository.stopDiscovery();
  //     add(RefreshDeviceList());
  //     emit(state.copyWith(loading: false));
  //   } catch (e) {
  //     logger.e('BLOC:: Error stopping discovery: $e');
  //     emit(state.copyWith(error: e.toString(), loading: false));
  //   }
  // }

  Future<void> _onGetDeviceList(
    GetDeviceList event,
    Emitter<BluetoothState> emit,
  ) async {
    try {
      final allDevices = await bluetoothRepository.getDevices();

      final validDevices = allDevices
          .where((d) => d.device.name.isNotEmpty && d.device.address.isNotEmpty)
          .toList();

      emit(state.copyWith(
        devices: validDevices,
      ));
    } catch (e, stack) {
      logger.e('error in getting devices list', error: e, stackTrace: stack);
    }
  }

  /// Refresh device list after discovery
  Future<void> _onGetRefreshDeviceList(
    RefreshDeviceList event,
    Emitter<BluetoothState> emit,
  ) async {
    print(
        "BLOC::_onGetRefreshDeviceList:: Loading available Bluetooth devices...");
    emit(state.copyWith(loading: true));
    try {
      await bluetoothRepository.startDiscovery();

      await Future.delayed(const Duration(seconds: 10));

      await bluetoothRepository.stopDiscovery();

      var allDevices = await bluetoothRepository.getDevices();

      var validDevices = allDevices
          .where((d) => d.device.name.isNotEmpty && d.device.address.isNotEmpty)
          .toList();

      emit(state.copyWith(
        devices: validDevices,
      ));

      emit(state.copyWith(loading: false));
    } catch (e, stack) {
      logger.e('BLOC::_onGetRefreshDeviceList:: Error refreshing device list',
          error: e, stackTrace: stack);
      emit(state.copyWith(
        error: e.toString(),
      ));
    }
  }

  Future<void> _onPair(PairDevice event, Emitter<BluetoothState> emit) async {
    try {
      add(BluetoothConnectingEvent(
          address: event.address, connectionLoading: true));

      print('Pairing device with address: ${event.address}');

      await bluetoothRepository.pair(event.address);

      logger.i('Device paired successfully: ${event.address}');
    } catch (e) {
      logger.e('Error pairing device: $e');
      emit(state.copyWith(error: e.toString()));
    } finally {
      add(BluetoothConnectingEvent(
          address: event.address, connectionLoading: false));
    }
  }

  Future<void> _onConnect(
      ConnectDevice event, Emitter<BluetoothState> emit) async {
    try {
      add(BluetoothConnectingEvent(
          address: event.address, connectionLoading: true));
      final result = await bluetoothRepository.connect(event.address);

      if (result) {
        final allDevices = await bluetoothRepository.getDevices();

        final connectedDevice =
            allDevices.firstWhere((d) => d.device.address == event.address);

        final updatedDevices = state.devices.map(
            (d) => d.device.address == event.address ? connectedDevice : d);

        emit(state.copyWith(devices: [...updatedDevices]));
      }
      logger.i('Device connected successfully: ${event.address}');
    } catch (e) {
      logger.e('Error connecting to device: $e');
    } finally {
      add(BluetoothConnectingEvent(
          address: event.address, connectionLoading: false));
    }
  }

  Future<void> _onDisconnect(
      DisconnectDevice event, Emitter<BluetoothState> emit) async {
    try {
      logger.i('Disconnecting from device with address: ${event.address}');

      await bluetoothRepository.disconnect(event.address);

      final allDevices = await bluetoothRepository.getDevices();
      final disconnectedDevice =
          allDevices.firstWhere((d) => d.device.address == event.address);

      final updatedDevices = state.devices.map(
          (d) => d.device.address == event.address ? disconnectedDevice : d);

      emit(state.copyWith(devices: [...updatedDevices]));

      logger.i('Device disconnected successfully: ${event.address}');
    } catch (e) {
      // logger.e('Error disconnecting from device: $e');
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

  Future<void> _setSelectBluetoothDevice(
      SelectDevice event, Emitter<BluetoothState> emit) async {
    emit(state.copyWith(selectedDevice: event.selectedDevice));
  }

  void _setDeviceDiscoverable(
      DiscoveryEnabled event, Emitter<BluetoothState> emit) async {
    emit(state.copyWith(isDiscoveryEnabled: event.isDiscoverable));
    await bluetoothRepository.setDiscoverable(event.isDiscoverable);
    logger.i('BLOC:: SET Device discoverable: ${event.isDiscoverable}');
  }

  Future<void> _onDiscoverableChanged(
      DiscoverableChanged event, Emitter<BluetoothState> emit) async {
    logger.i('BLOC:: Device discoverable changed: ${event.enabled}');

    final adapter = state.bluetoothAdapter;
    if (adapter == null) return;

    emit(state.copyWith(
      bluetoothAdapter: adapter.copyWith(discoverable: event.enabled),
    ));
  }

  void _addedBluetoothDevices(
      BluetoothDevicesAdded event, Emitter<BluetoothState> emit) {
    if (event.device.name != '') {
      final isAlreadyAdded = state.devices
          .any((device) => device.device.address == event.device.address);

      if (!isAlreadyAdded) {
        final deviceType = BluetoothDeviceClassifier.classify(
          deviceClass: event.device.deviceClass,
          uuids: event.device.uuids,
        );

        final BluetoothDeviceDetails deviceDetails = BluetoothDeviceDetails(
            device: event.device, deviceType: deviceType);

        emit(state.copyWith(devices: [...state.devices, deviceDetails]));
      }
    }
  }

  void _updateBluetoothDevices(
    BluetoothDevicesUpdate event,
    Emitter<BluetoothState> emit,
  ) {
    final updatedDevices = state.devices.map((d) {
      if (d.device.address == event.device.device.address) {
        return d.copyWith(deviceType: event.category);
      }
      return d;
    }).toList();

    emit(state.copyWith(
        devices: updatedDevices,
        selectedDevice:
            state.selectedDevice?.copyWith(deviceType: event.category)));
  }

  void _onBluetoothConnectionLoading(
      BluetoothConnectingEvent event, Emitter<BluetoothState> emit) {
    emit(state.copyWith(
        connection: BluetoothConnection(
      address: event.address,
      connectionLoading: event.connectionLoading,
    )));
  }

  void _removedBluetoothDevices(
      BluetoothDevicesRemoved event, Emitter<BluetoothState> emit) {
    if (event.address != '') {
      final devices = state.devices
          .where((device) => device.device.address != event.address)
          .toList();
      emit(state.copyWith(
        devices: devices,
      ));
    }
  }

  @override
  Future<void> close() async {
    print("bluetooth bloc closing...!");
    await _bluetoothDeviceAddedStream?.cancel();
    _bluetoothDeviceAddedStream = null;
    await _bluetoothDeviceRemovedStream?.cancel();
    _bluetoothDeviceRemovedStream = null;

    _adapterPowerSubscription?.cancel();
    _adapterDiscoverableSubscription?.cancel();

    bluetoothRepository.close();
    return super.close();
  }
}
