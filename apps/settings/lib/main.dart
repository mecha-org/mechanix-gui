import 'dart:io';

import 'package:dbus/dbus.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/features/about/bloc/about_bloc.dart';
import 'package:mechanix_settings/src/features/about/data/about_repository.dart';
import 'package:mechanix_settings/src/features/about/data/about_repository_impl.dart';
import 'package:mechanix_settings/src/features/about/presentation/about.dart';
import 'package:mechanix_settings/src/features/appearance/bloc/appearance_bloc.dart';
import 'package:mechanix_settings/src/features/appearance/data/appearance_repository.dart';
import 'package:mechanix_settings/src/features/appearance/data/appearance_repository_impl.dart';
import 'package:mechanix_settings/src/features/appearance/presentation/appearance.dart';
import 'package:mechanix_settings/src/features/appearance/presentation/set_up_theme.dart';
import 'package:mechanix_settings/src/features/appearance/presentation/set_up_wallpaper.dart';
import 'package:mechanix_settings/src/features/appearance/presentation/wallpaper_preview.dart';
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
import 'package:mechanix_settings/src/features/bluetooth/presentation/manage_device.dart';
import 'package:mechanix_settings/src/features/bluetooth/presentation/rename_adapter.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_bloc.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_event.dart';
import 'package:mechanix_settings/src/features/date_time/presentation/date_settings.dart';
import 'package:mechanix_settings/src/features/date_time/presentation/date_time.dart';
import 'package:mechanix_settings/src/features/date_time/presentation/time_settings.dart';
import 'package:mechanix_settings/src/features/date_time/presentation/time_zone_list.dart';
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
import 'package:mechanix_settings/src/features/system_update/presentation/system_updates.dart';
import 'package:watch_it/watch_it.dart';
import 'package:widgets/mechanix.dart';

import 'load_settings.dart';

void main(List<String> args) async {
  di.registerSingleton(ThemeToggle());
  WidgetsFlutterBinding.ensureInitialized();

  const compileTimeOpenPath =
      String.fromEnvironment('MECHANIX_SETTINGS_OPEN_PATH');

  print("Main compileTimeOpenPath  $compileTimeOpenPath");

  final runtimeOpenPath = Platform.environment['MECHANIX_SETTINGS_OPEN_PATH'];

  print("Main runtimeOpenPath $runtimeOpenPath");

  final openPath =
      compileTimeOpenPath.isNotEmpty ? compileTimeOpenPath : runtimeOpenPath;

  print('Open path: $openPath');

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
        RepositoryProvider<AppearanceRepository>(
          create: (_) => AppearanceRepositoryImpl(),
        ),
      ],
      child: MechanixSettingsApp(openPath: openPath ?? ''),
    ),
  );
}

class MechanixSettingsApp extends StatelessWidget with WatchItMixin {
  MechanixSettingsApp({super.key, required this.openPath});
  final String openPath;

  @override
  Widget build(BuildContext context) {
    final themeMode = watchPropertyValue((ThemeToggle t) => t.themeMode);

    return _MechanixSettingsAppContent(
      themeMode: themeMode,
      openPath: openPath,
    );
  }
}

class _MechanixSettingsAppContent extends StatefulWidget {
  const _MechanixSettingsAppContent(
      {required this.themeMode, required this.openPath});

  final ThemeMode themeMode;
  final String openPath;

  @override
  State<_MechanixSettingsAppContent> createState() =>
      _MechanixSettingsAppContentState();
}

class _MechanixSettingsAppContentState
    extends State<_MechanixSettingsAppContent> {
  late final DBusClient _bus;
  late final ThemeSettingsService _themeService;

  MechanixThemeData _currentThemeData = MechanixThemeData(
    mechanixVariant: MechanixVariant.amber,
  );

  @override
  void initState() {
    super.initState();
    _initializeThemeService();
  }

  void _initializeThemeService() {
    _bus = DBusClient.session();
    _themeService = ThemeSettingsService(_bus);

    _themeService.listenForThemeChanges(_handleThemeChange);
    _fetchInitialTheme();
  }

  Future<void> _fetchInitialTheme() async {
    final colors = await _themeService.fetchCurrentTheme();
    if (colors != null) {
      _handleThemeChange(colors);
    }
  }

  void _handleThemeChange(Map<String, String> colors) {
    setState(() {
      _currentThemeData = _themeService.colorsToThemeData(colors);
    });
  }

  @override
  void dispose() {
    _themeService.dispose();
    _bus.close();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    print("Main openpath ${widget.openPath}");

    return MechanixTheme(
      data: _currentThemeData,
      builder: (context, mechanix, child) => MainApp(
        darkTheme: mechanix.darkTheme,
        lightTheme: mechanix.lightTheme,
        themeMode: widget.themeMode,
        openPath: widget.openPath,
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
    required this.openPath,
  });

  final ThemeData lightTheme;
  final ThemeData darkTheme;
  final ThemeMode themeMode;
  final String openPath;

  @override
  Widget build(BuildContext context) {
    return MultiBlocProvider(
      providers: [
        // Sound Bloc
        BlocProvider(
          create: (context) => SoundBloc(
            soundRepository: context.read<SoundRepository>(),
          )..add(InitializeSound()),
        ),
        // Bluetooth Bloc
        BlocProvider(
          create: (context) => BluetoothBloc(
            bluetoothRepository: context.read<BluetoothRepository>(),
          )..add(InitBluetooth()),
        ),
        // Battery Bloc
        BlocProvider(
          create: (context) => BatteryBloc(
            batteryRepository: context.read<BatteryRepository>(),
          )..add(BatteryInfoRequested()),
        ),
        // Display Bloc
        BlocProvider(
          create: (context) => DisplayBloc(
            displayRepository: context.read<DisplayRepository>(),
          )..add(GetDefaultSettingsEvent()),
        ),
        // DateTime Bloc
        BlocProvider(
          create: (context) => DateTimeBloc()..add(GetDateTimeData()),
        ),
        // ConnectNetwork Bloc (note: this might need special handling)
        BlocProvider(
          create: (context) => ConnectNetworkBloc(
            wifiRepository: context.read<WifiRepository>(),
          ),
        ),
        // About Bloc
        BlocProvider(
          create: (context) => AboutBloc(
            aboutRepository: context.read<AboutRepository>(),
          )..add(InitializeAbout()),
        ),

        // Appearance Bloc
        BlocProvider(
          create: (context) => AppearanceBloc(
            appearanceRepository: context.read<AppearanceRepository>(),
          )..add(AppearanceInit()),
        ),
      ],
      child: MaterialApp(
        debugShowCheckedModeBanner: false,
        home: SettingMenu(openPath: openPath),
        theme: lightTheme,
        darkTheme: _buildDarkTheme(),
        themeMode: themeMode,
        routes: {
          // Sound Routes
          AppRoutes.sound: (context) => BlocProvider(
                create: (context) => SoundBloc(
                  soundRepository: context.read<SoundRepository>(),
                )..add(InitializeSound()),
                child: const Sound(),
              ),
          AppRoutes.vibrationLevel: (context) => const VibrationLevel(),
          AppRoutes.soundOutputDevices: (context) => const OutputDevices(),
          AppRoutes.soundInputDevices: (context) => const InputDevices(),
          AppRoutes.notificationSound: (context) => const NotificationSounds(),

          // Wireless Routes
          AppRoutes.wireless: (context) => BlocProvider(
                create: (context) => WirelessSettingsBloc(
                  wifiRepository: context.read<WifiRepository>(),
                )..add(InitWifi()),
                lazy: false,
                child: const WirelessSettings(),
              ),
          AppRoutes.wirelessNetworkDetails: (context) => BlocProvider.value(
                // make common widget
                value: context.read<WirelessSettingsBloc>(),
                child: const NetworkDetails(),
              ),
          AppRoutes.ipSettings: (context) => const IpSettings(),
          AppRoutes.ethernetDetails: (context) => const EthernetSettings(),
          AppRoutes.dnsDetails: (context) => const DnsSettings(),
          AppRoutes.wirelessNetworkSettings: (context) => BlocProvider.value(
                value: context.read<WirelessSettingsBloc>(),
                child: const NetworkSettings(),
              ),
          AppRoutes.wirelessSavedNetworkDetails: (context) =>
              BlocProvider.value(
                value: context.read<WirelessSettingsBloc>(),
                child: const SavedNetworkDetails(network: null),
              ),
          AppRoutes.wirelessConnectSecureNetwork: (context) =>
              BlocProvider.value(
                value: context.read<WirelessSettingsBloc>(),
                child: const ConnectSecureNetwork(),
              ),
          AppRoutes.wirelessConnectHiddenNetwork: (context) =>
              BlocProvider.value(
                value: context.read<WirelessSettingsBloc>(),
                child: const AddNetwork(),
              ),
          AppRoutes.configureDNS: (context) => const ConfigureDnsWidget(),
          AppRoutes.configureProxy: (context) => const ConfigureProxyWidget(),
          AppRoutes.ipv4Address: (context) => const Ipv4AddressWidget(),
          AppRoutes.security: (context) => const WifiSecurityWidget(),

          // Bluetooth Routes
          AppRoutes.bluetooth: (context) => const Bluetooth(),
          AppRoutes.bluetoothDeviceInfo: (context) =>
              const BluetoothDeviceInfo(),
          AppRoutes.adapterSettings: (context) => const AdapterSettings(),
          AppRoutes.adapterRename: (context) => const RenameAdapter(),
          AppRoutes.bluetoothDiscoverable: (context) =>
              const BluetoothDeviceDiscoverable(),
          AppRoutes.bluetoothDeviceTypes: (context) => const DeviceTypes(),
          AppRoutes.manageDevice: (context) => const ManageDevice(),

          // Battery Routes
          AppRoutes.battery: (context) => BlocProvider(
                create: (context) => BatteryBloc(
                  batteryRepository: context.read<BatteryRepository>(),
                )..add(BatteryInit()),
                child: const Battery(),
              ),
          AppRoutes.batteryPerformance: (context) => const BatteryPerformance(),

          // Appearance Routes
          AppRoutes.appearance: (context) => BlocProvider(
                create: (context) => AppearanceBloc(
                  appearanceRepository: context.read<AppearanceRepository>(),
                )..add(AppearanceInit()),
                child: const Appearance(),
              ),
          AppRoutes.setTheme: (context) => const SetUpTheme(),
          AppRoutes.setWallpaper: (context) => const SetUpWallpaper(),
          AppRoutes.wallpaperPreview: (context) => const WallpaperPreview(),

          // Display Routes
          AppRoutes.display: (context) => BlocProvider(
                create: (context) => DisplayBloc(
                  displayRepository: context.read<DisplayRepository>(),
                )..add(GetDefaultSettingsEvent()),
                child: const DisplayPage(),
              ),
          AppRoutes.displayScreenOffTime: (context) =>
              const ScreenOffTimeSettings(),
          AppRoutes.lockScreenTimeout: (context) => const LockScreenTimeout(),
          AppRoutes.timeZone: (context) => const TimeZoneList(),

          // Other Routes
          AppRoutes.about: (context) => const About(),
          AppRoutes.systemUpdates: (context) => const SystemUpdates(),
          AppRoutes.dateTime: (context) => BlocProvider(
                create: (context) => DateTimeBloc()..add(InitializeDateTime()),
                child: const DateTimeSettings(),
              ),
          AppRoutes.timeSettings: (context) => const TimeSettings(),
          AppRoutes.dateSettings: (context) => const DateSettings(),
        },
      ),
    );
  }

  ThemeData _buildDarkTheme() {
    return darkTheme.copyWith(
      scaffoldBackgroundColor: Colors.black,
      pageTransitionsTheme: const PageTransitionsTheme(
        builders: {
          TargetPlatform.linux: CupertinoPageTransitionsBuilder(),
        },
      ),
    );
  }
}
