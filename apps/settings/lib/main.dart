import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/features/about/bloc/about_bloc.dart';
import 'package:mechanix_settings/src/features/about/data/about_repository.dart';
import 'package:mechanix_settings/src/features/about/data/about_repository_impl.dart';
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
import 'package:mechanix_settings/src/features/display/bloc/display_bloc.dart';
import 'package:mechanix_settings/src/features/display/data/display_repository.dart';
import 'package:mechanix_settings/src/features/display/data/display_repository_impl.dart';
import 'package:mechanix_settings/src/features/display/presentation/display.dart';
import 'package:mechanix_settings/src/features/display/presentation/lock_screen_timeout.dart';
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
import 'package:mechanix_settings/src/features/network/presentation/saved_network_details.dart';
import 'package:mechanix_settings/src/features/network/presentation/wifi_security.dart';
import 'package:mechanix_settings/src/features/network/presentation/wireless.dart';
import 'package:mechanix_settings/src/features/settings_menu/presentation/menu.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_bloc.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_event.dart';
import 'package:mechanix_settings/src/features/sound/data/sound_repository.dart';
import 'package:mechanix_settings/src/features/sound/data/sound_repository_impl.dart';
import 'package:mechanix_settings/src/features/sound/presentation/input_devices.dart';
import 'package:mechanix_settings/src/features/sound/presentation/notification_sound.dart';
import 'package:mechanix_settings/src/features/sound/presentation/output_devices.dart';
import 'package:mechanix_settings/src/features/sound/presentation/sound.dart';
import 'package:mechanix_settings/src/features/sound/presentation/vibration_level.dart';
import 'package:watch_it/watch_it.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/navigation_bar/mechanix_navigation_bar_theme.dart';
import 'package:widgets/widgets/switch/mechanix_switch_theme.dart';

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
        RepositoryProvider<DisplayRepository>(
          create: (_) => DisplayRepositoryImpl(),
        ),
        RepositoryProvider<AboutRepository>(
          create: (_) => AboutRepositoryImpl(),
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
      data: MechanixThemeData(mechanixVariant: mechanixVariant, extensions: const [
        MechanixNavigationBarThemeData(
          scrolledUnderElevation: 0,
          titleStyle: TextStyle(
            fontSize: 24,
          ),
          titleSpacing: 0,
          backgroundColor: Colors.transparent,
          elevation: 0,
        ),
        MechanixSwitchThemeData(
          style: MechanixSwitchStyle(
            inactiveThumbColor: Color(0xFF989898),
            inactiveTrackColor: Color(0xFF252525),
          ),
        ),
      ]),
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
    return MultiBlocProvider(
      providers: [
        // Sound Bloc
        BlocProvider(
          create: (context) =>
              SoundBloc(soundRepository: context.read<SoundRepository>())
                ..add(InitializeSound())
                ..add(GetOutputDeviceList())
                ..add(GetInputDeviceList()),
        ),

        // Wireless Bloc
        BlocProvider(
          create: (context) => WirelessSettingsBloc(
              wifiRepository: context.read<WifiRepository>())
            ..add(InitWifi()),
        ),

        // Bluetooth Bloc
        BlocProvider(
          create: (context) => BluetoothBloc(
              bluetoothRepository: context.read<BluetoothRepository>())
            ..add(InitBluetooth()),
        ),

        // Battery Bloc
        BlocProvider(
          create: (context) =>
              BatteryBloc(batteryRepository: context.read<BatteryRepository>())
                ..add(BatteryInfoRequested()),
        ),

        // Display Bloc
        BlocProvider(
          create: (context) =>
              DisplayBloc(displayRepository: context.read<DisplayRepository>())
                ..add(GetDefaultSettingsEvent()),
        ),

        // DateTime Bloc
        BlocProvider(
          create: (context) => DateTimeBloc()..add(GetDateTimeData()),
        ),

        // ConnectNetwork Bloc (note: this might need special handling)
        BlocProvider(
          create: (context) => ConnectNetworkBloc(
              wifiRepository: context.read<WifiRepository>()),
        ),

        // About Bloc
        BlocProvider(
          create: (context) =>
              AboutBloc(aboutRepository: context.read<AboutRepository>())
                ..add(InitializeAbout()),
        ),
      ],
      child: MaterialApp(
        debugShowCheckedModeBanner: false,
        home: SettingMenu(),
        theme: lightTheme,
        darkTheme: darkTheme.copyWith(
          scaffoldBackgroundColor: Colors.black,
          pageTransitionsTheme: const PageTransitionsTheme(
            builders: {TargetPlatform.linux: CupertinoPageTransitionsBuilder()},
          ),
        ),
        themeMode: themeMode,
        routes: {
          // Sound Routes
          AppRoutes.sound: (context) => const Sound(),
          AppRoutes.vibrationLevel: (context) => const VibrationLevel(),
          AppRoutes.soundOutputDevices: (context) => const OutputDevices(),
          AppRoutes.soundInputDevices: (context) => const InputDevices(),
          AppRoutes.notificationSound: (context) => const NotificationSound(),

          // Wireless Routes
          AppRoutes.wireless: (context) => const WirelessSettings(),
          AppRoutes.wirelessNetworkDetails: (context) => const NetworkDetails(),
          // TODO: for route level bloc 
          // AppRoutes.wireless: (context) => BlocProvider(
          //       create: (context) => WirelessSettingsBloc(
          //         wifiRepository: context.read<WifiRepository>(),
          //       )..add(InitWifi()),
          //       child: const WirelessSettings(),
          //     ),
          // AppRoutes.wirelessNetworkDetails: (context) => BlocProvider.value(  // make common widget 
          //       value: context.read<WirelessSettingsBloc>(),
          //       child: const NetworkDetails(),
          //     ),
          AppRoutes.ipSettings: (context) => const IpSettings(),
          AppRoutes.ethernetDetails: (context) => const EthernetSettings(),
          AppRoutes.dnsDetails: (context) => const DnsSettings(),
          AppRoutes.wirelessNetworkSettings: (context) => const NetworkSettings(),
          AppRoutes.wirelessSavedNetworkDetails: (context) =>
              const SavedNetworkDetails(),
          AppRoutes.wirelessConnectSecureNetwork: (context) =>
              const ConnectSecureNetwork(),
          AppRoutes.wirelessConnectHiddenNetwork: (context) => const AddNetwork(),
          AppRoutes.configureDNS: (context) => const ConfigureDnsWidget(),
          AppRoutes.configureProxy: (context) => const ConfigureProxyWidget(),
          AppRoutes.ipv4Address: (context) => const Ipv4AddressWidget(),
          AppRoutes.security: (context) => const WifiSecurityWidget(),

          // Bluetooth Routes
          AppRoutes.bluetooth: (context) => const Bluetooth(),
          AppRoutes.bluetoothDeviceInfo: (context) => const BluetoothDeviceInfo(),
          AppRoutes.adapterSettings: (context) => const AdapterSettings(),
          AppRoutes.adapterRename: (context) => const RenameAdapter(),
          AppRoutes.bluetoothDiscoverable: (context) =>
              const BluetoothDeviceDiscoverable(),
          AppRoutes.bluetoothDeviceTypes: (context) => const DeviceTypes(),

          // Battery Routes
          AppRoutes.battery: (context) => const Battery(),
          AppRoutes.batteryPerformance: (context) => const BatteryPerformance(),

          // Display Routes
          AppRoutes.display: (context) => const DisplayPage(),
          AppRoutes.appearance: (context) => const Appearance(),
          AppRoutes.applyWallpaper: (context) => const ApplyWallpaper(),
          AppRoutes.displayScreenOffTime: (context) => const ScreenOffTimeSettings(),
          AppRoutes.lockScreenTimeout: (context) => const LockScreenTimeout(),

          // Other Routes
          AppRoutes.about: (context) => const About(),
          AppRoutes.dateTime: (context) => const DateTimeSettings(),
          AppRoutes.timeSettings: (context) => const TimeSettings(),
          AppRoutes.dateSettings: (context) => const DateSettings(),
        },
      ),
    );
  }
}
