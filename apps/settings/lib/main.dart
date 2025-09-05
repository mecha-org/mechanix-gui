import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/features/appearance/presentation/appearance.dart';
import 'package:mechanix_settings/src/features/appearance/presentation/apply_wallpaper.dart';
import 'package:mechanix_settings/src/features/battery/blocs/battery_bloc.dart';
import 'package:mechanix_settings/src/features/battery/data/battery_repository.dart';
import 'package:mechanix_settings/src/features/battery/data/battery_repository_impl.dart';
import 'package:mechanix_settings/src/features/battery/blocs/battery_event.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_event.dart';
import 'package:mechanix_settings/src/features/bluetooth/data/bluetooth_repository.dart';
import 'package:mechanix_settings/src/features/bluetooth/data/bluetooth_repository_impl.dart';
import 'package:mechanix_settings/src/features/bluetooth/presentation/bluetooth.dart';
import 'package:mechanix_settings/src/features/bluetooth/presentation/device_info/bluetooth_device_info.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_bloc.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_event.dart';
import 'package:mechanix_settings/src/features/date_time/presentation/date_settings.dart';
import 'package:mechanix_settings/src/features/date_time/presentation/time_settings.dart';
import 'package:mechanix_settings/src/features/network/data/wifi_repository.dart';
import 'package:mechanix_settings/src/features/network/data/wifi_repository_impl.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkBloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_bloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_event.dart';
import 'package:mechanix_settings/src/features/battery/presentation/battery.dart';
import 'package:mechanix_settings/src/features/battery/presentation/battery_performance.dart';
import 'package:mechanix_settings/src/features/network/presentation/add_network.dart';
import 'package:mechanix_settings/src/features/network/presentation/connect_secure_network.dart';
import 'package:mechanix_settings/src/features/network/presentation/dns_settings/dns_settings.dart';
import 'package:mechanix_settings/src/features/network/presentation/ethernet_settings/ethernet_settings.dart';
import 'package:mechanix_settings/src/features/network/presentation/ipsettings/ip_settings.dart';
import 'package:mechanix_settings/src/features/network/presentation/network_details.dart';
import 'package:mechanix_settings/src/features/network/presentation/network_settings.dart';
import 'package:mechanix_settings/src/features/network/presentation/wireless.dart';
import 'package:mechanix_settings/src/features/settings_menu/presentation/menu.dart';
import 'package:mechanix_settings/src/features/bluetooth/presentation/adapter_settings.dart';
import 'package:mechanix_settings/src/features/bluetooth/presentation/rename_adapter.dart';
import 'package:mechanix_settings/src/features/display/presentation/display.dart';
import 'package:mechanix_settings/src/features/display/presentation/settings.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_bloc.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_event.dart';
import 'package:mechanix_settings/src/features/sound/data/sound_repository.dart';
import 'package:mechanix_settings/src/features/sound/data/sound_repository_impl.dart';
import 'package:mechanix_settings/src/features/sound/presentation/input_devices.dart';
import 'package:mechanix_settings/src/features/sound/presentation/output_devices.dart';
import 'package:mechanix_settings/src/features/sound/presentation/sound.dart';
import 'package:mechanix_settings/src/features/about/presentation/about.dart';
import 'package:mechanix_settings/src/features/date_time/presentation/date_time.dart';

void main() {
  runApp(
    MultiRepositoryProvider(
      providers: [
        RepositoryProvider<WifiRepository>(
          create: (_) => WifiRepositoryImpl(),
        ),
        RepositoryProvider<BatteryRepository>(
          create: (_) => BatteryRepositoryImpl(),
        ),
        RepositoryProvider<BluetoothRepository>(
          create: (_) => BluetoothRepositoryImpl(),
        ),
        RepositoryProvider<SoundRepository>(
          create: (_) => SoundRepositoryImpl(),
        ),
      ],
      child: MainApp(),
    ),
  );
}

class MainApp extends StatelessWidget {
  const MainApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      home: SettingMenu(),
      theme: ThemeData.dark(),
      routes: {
        AppRoutes.wireless: (context) => BlocProvider(
              create: (_) => WirelessSettingsBloc(
                  wifiRepository: context.read<WifiRepository>())
                ..add(InitializeWifi()),
              child: WirelessSettings(),
            ),
        AppRoutes.wirelessNetworkDetails: (context) => BlocProvider(
              create: (_) => WirelessSettingsBloc(
                wifiRepository: context.read<WifiRepository>(),
              ),
              child: NetworkDetails(),
            ),
        AppRoutes.ipSettings: (context) => IpSettings(),
        //  BlocProvider(
        //       create: (_) => WirelessSettingsBloc(
        //         wifiRepository: context.read<WifiRepository>(),
        //       ),
        //       child: IpSettings(),
        //     ),
        AppRoutes.ethernetDetails: (context) => EthernetSettings(),
        //  BlocProvider(
        //       create: (_) => WirelessSettingsBloc(
        //         wifiRepository: context.read<WifiRepository>(),
        //       ),
        //       child: EthernetSettings(),
        //     ),
        AppRoutes.dnsDetails: (context) => DnsSettings(),
        //  BlocProvider(
        //       create: (_) => WirelessSettingsBloc(
        //         wifiRepository: context.read<WifiRepository>(),
        //       ),
        //       child: DnsSettings(),
        //     ),
        AppRoutes.wirelessNetworkSettings: (context) => BlocProvider(
              create: (_) => WirelessSettingsBloc(
                  wifiRepository: context.read<WifiRepository>())
                ..add(LoadSavedNetworks()),
              child: NetworkSettings(),
            ),
        AppRoutes.wirelessConnectSecureNetwork: (context) => BlocProvider(
              create: (_) => ConnectNetworkBloc(
                wifiRepository: context.read<WifiRepository>(),
              ),
              child: ConnectSecureNetwork(),
            ),
        AppRoutes.wirelessConnectUnknownNetwork: (context) => BlocProvider(
              create: (_) => ConnectNetworkBloc(
                wifiRepository: context.read<WifiRepository>(),
              ),
              child: AddNetwork(),
            ),
        AppRoutes.bluetooth: (context) => BlocProvider(
              create: (_) => BluetoothBloc(
                bluetoothRepository: context.read<BluetoothRepository>(),
              )..add(InitializeBluetooth()),
              child: Bluetooth(),
            ),
        AppRoutes.bluetoothDeviceInfo: (context) => BlocProvider(
              create: (_) => BluetoothBloc(
                bluetoothRepository: context.read<BluetoothRepository>(),
              ),
              child: BluetoothDeviceInfo(),
            ),
        AppRoutes.adapterSettings: (context) => BlocProvider(
              create: (_) => BluetoothBloc(
                bluetoothRepository: context.read<BluetoothRepository>(),
              )..add(GetAdapterAlias()),
              child: AdapterSettings(),
            ),
        AppRoutes.adapterRename: (context) => BlocProvider(
              create: (_) => BluetoothBloc(
                bluetoothRepository: context.read<BluetoothRepository>(),
              )..add(GetAdapterAlias()),
              child: RenameAdapter(),
            ),
        AppRoutes.battery: (context) => BlocProvider(
              create: (_) => BatteryBloc(
                  batteryRepository: context.read<BatteryRepository>())
                ..add(BatteryInfoRequested()),
              child: Battery(),
            ),
        AppRoutes.batteryPerformance: (context) => BlocProvider(
              create: (_) => BatteryBloc(
                  batteryRepository: context.read<BatteryRepository>())
                ..add(BatteryInfoRequested()),
              child: BatteryPerformance(),
            ),
        AppRoutes.display: (context) => Display(),
        AppRoutes.appearance: (context) => Appearance(),
        AppRoutes.applyWallpaper: (context) => ApplyWallpaper(),
        AppRoutes.displayScreenOffTime: (context) => ScreenOffTimeSettings(),
        AppRoutes.sound: (context) => BlocProvider(
              create: (_) =>
                  SoundBloc(soundRepository: context.read<SoundRepository>())
                    ..add(InitializeSound()),
              child: Sound(),
            ),
        AppRoutes.soundOutputDevices: (context) => BlocProvider(
              create: (_) =>
                  SoundBloc(soundRepository: context.read<SoundRepository>())
                    ..add(
                        InitializeSound()) // TODO: InitializeSound should be called only once, not on every route change
                    ..add(GetOutputDeviceList()),
              child: OutputDevices(),
            ),
        AppRoutes.soundInputDevices: (context) => BlocProvider(
              create: (_) =>
                  SoundBloc(soundRepository: context.read<SoundRepository>())
                    ..add(
                        InitializeSound()) // TODO: InitializeSound should be called only once, not on every route change
                    ..add(GetInputDeviceList()),
              child: InputDevices(),
            ),
        AppRoutes.about: (context) => About(),
        AppRoutes.dateTime: (context) => BlocProvider(
              create: (_) => DateTimeBloc()..add(GetDateTimeData()),
              child: DateTimeSettings(),
            ),
        AppRoutes.timeSettings: (context) => BlocProvider(
              create: (_) => DateTimeBloc()..add(GetDateTimeData()),
              child: TimeSettings(),
            ),
        AppRoutes.dateSettings: (context) => BlocProvider(
              create: (_) => DateTimeBloc()..add(GetDateTimeData()),
              child: DateSettings(),
            )
      },
    );
  }
}
