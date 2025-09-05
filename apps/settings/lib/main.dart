import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/features/about/presentation/about.dart';
import 'package:mechanix_settings/src/features/appearance/presentation/appearance.dart';
import 'package:mechanix_settings/src/features/appearance/presentation/apply_wallpaper.dart';
import 'package:mechanix_settings/src/features/battery/blocs/battery_bloc.dart';
import 'package:mechanix_settings/src/features/battery/blocs/battery_event.dart';
import 'package:mechanix_settings/src/features/battery/data/battery_repository.dart';
import 'package:mechanix_settings/src/features/battery/data/battery_repository_impl.dart';
import 'package:mechanix_settings/src/features/battery/presentation/battery.dart';
import 'package:mechanix_settings/src/features/battery/presentation/battery_performance.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_event.dart';
import 'package:mechanix_settings/src/features/bluetooth/data/bluetooth_repository.dart';
import 'package:mechanix_settings/src/features/bluetooth/data/bluetooth_repository_impl.dart';
import 'package:mechanix_settings/src/features/bluetooth/presentation/adapter_settings.dart';
import 'package:mechanix_settings/src/features/bluetooth/presentation/bluetooth.dart';
import 'package:mechanix_settings/src/features/bluetooth/presentation/bluetooth_device_discoverable.dart';
import 'package:mechanix_settings/src/features/bluetooth/presentation/device_info/bluetooth_device_info.dart';
import 'package:mechanix_settings/src/features/bluetooth/presentation/device_types.dart';
import 'package:mechanix_settings/src/features/bluetooth/presentation/rename_adapter.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_bloc.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_event.dart';
import 'package:mechanix_settings/src/features/date_time/presentation/date_settings.dart';
import 'package:mechanix_settings/src/features/date_time/presentation/date_time.dart';
import 'package:mechanix_settings/src/features/date_time/presentation/time_settings.dart';
import 'package:mechanix_settings/src/features/display/presentation/display.dart';
import 'package:mechanix_settings/src/features/display/presentation/settings.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkBloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_bloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_event.dart';
import 'package:mechanix_settings/src/features/network/data/wifi_repository.dart';
import 'package:mechanix_settings/src/features/network/data/wifi_repository_impl.dart';
import 'package:mechanix_settings/src/features/network/presentation/add_network.dart';
import 'package:mechanix_settings/src/features/network/presentation/configure_dns.dart';
import 'package:mechanix_settings/src/features/network/presentation/configure_proxy.dart';
import 'package:mechanix_settings/src/features/network/presentation/connect_secure_network.dart';
import 'package:mechanix_settings/src/features/network/presentation/dns_settings/dns_settings.dart';
import 'package:mechanix_settings/src/features/network/presentation/ethernet_settings/ethernet_settings.dart';
import 'package:mechanix_settings/src/features/network/presentation/ipsettings/ip_settings.dart';
import 'package:mechanix_settings/src/features/network/presentation/ipv4_address.dart';
import 'package:mechanix_settings/src/features/network/presentation/network_details.dart';
import 'package:mechanix_settings/src/features/network/presentation/network_settings.dart';
import 'package:mechanix_settings/src/features/network/presentation/wifi_security.dart';
import 'package:mechanix_settings/src/features/network/presentation/wireless.dart';
import 'package:mechanix_settings/src/features/settings_menu/presentation/menu.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_bloc.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_event.dart';
import 'package:mechanix_settings/src/features/sound/data/sound_repository.dart';
import 'package:mechanix_settings/src/features/sound/data/sound_repository_impl.dart';
import 'package:mechanix_settings/src/features/sound/presentation/input_devices.dart';
import 'package:mechanix_settings/src/features/sound/presentation/output_devices.dart';
import 'package:mechanix_settings/src/features/sound/presentation/sound.dart';
import 'package:watch_it/watch_it.dart';
import 'package:widgets/mechanix.dart';

void main() async {
  di.registerSingleton(ThemeToggle());
  WidgetsFlutterBinding.ensureInitialized();
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
      child: MechanixSettingsApp(),
    ),
  );
}

class MechanixSettingsApp extends StatelessWidget with WatchItMixin {
  MechanixSettingsApp({super.key});

  @override
  Widget build(BuildContext context) {
    final themeMode = watchPropertyValue((ThemeToggle t) => t.themeMode);
    final mechanixVariant =
        watchPropertyValue((ThemeToggle t) => t.mechanixVariant);

    return MechanixTheme(
      data: MechanixThemeData(
        mechanixVariant: mechanixVariant,
      ),
      builder: (context, mechanix, child) => MainApp(
        darkTheme: mechanix.darkTheme,
        lightTheme: mechanix.lightTheme,
        themeMode: themeMode,
      ),
    );
  }
}

class MainApp extends StatelessWidget {
  const MainApp({
    super.key,
    required this.lightTheme,
    required this.darkTheme,
    required this.themeMode,
  });

  final ThemeData lightTheme;
  final ThemeData darkTheme;
  final ThemeMode themeMode;

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      home: SettingMenu(),
      theme: lightTheme,
      darkTheme: darkTheme,
      themeMode: themeMode,
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
            ),
        AppRoutes.configureDNS: (context) => BlocProvider(
              create: (_) => WirelessSettingsBloc(
                  wifiRepository: context.read<WifiRepository>())
                ..add(LoadSavedNetworks()),
              child: ConfigureDnsWidget(),
            ),
        AppRoutes.configureProxy: (context) => BlocProvider(
              create: (_) => WirelessSettingsBloc(
                  wifiRepository: context.read<WifiRepository>())
                ..add(LoadSavedNetworks()),
              child: ConfigureProxyWidget(),
            ),

        AppRoutes.ipv4Address: (context) => BlocProvider(
              create: (_) => WirelessSettingsBloc(
                  wifiRepository: context.read<WifiRepository>())
                ..add(LoadSavedNetworks()),
              child: Ipv4AddressWidget(),
            ),

        AppRoutes.security: (context) => BlocProvider(
              create: (_) => WirelessSettingsBloc(
                  wifiRepository: context.read<WifiRepository>())
                ..add(LoadSavedNetworks()),
              child: WifiSecurityWidget(),
            ),
        AppRoutes.bluetoothDiscoverable: (context) => BlocProvider(
              create: (_) => BluetoothBloc(
                bluetoothRepository: context.read<BluetoothRepository>(),
              ),
              child: BluetoothDeviceDiscoverable(),
            ),
        AppRoutes.bluetoothDeviceTypes: (context) => DeviceTypes()
      },
    );
  }
}
