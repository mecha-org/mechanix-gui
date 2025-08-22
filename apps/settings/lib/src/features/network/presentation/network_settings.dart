import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_icon.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_row_item.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_text_button.dart';
import 'package:mechanix_settings/src/commons/styles/color.dart';
import 'package:mechanix_settings/src/commons/styles/custom_styles.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_bloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_state.dart';
import 'package:nm/nm.dart';
import 'package:mechanix_settings/src/features/network/presentation/wireless.dart';

class NetworkSettings extends StatelessWidget {
  const NetworkSettings({super.key});

  void _backNavigation(BuildContext context) {
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext outerContext) {
    // Rename to outerContext
    return BlocBuilder<WirelessSettingsBloc, WirelessSettingsState>(
      builder: (context, state) {
        void showDeleteDialog(String networkName) {
          showDialog(
            context: context,
            builder: (dialogContext) {
              // This context does NOT have BlocProvider!
              return AlertDialog(
                backgroundColor: const Color.fromARGB(255, 54, 54, 54),
                title: const Text('Delete saved network'),
                content: Column(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Text('Network name: $networkName',
                        style: const TextStyle(fontSize: 18)),
                    Text('Security : None',
                        style: const TextStyle(fontSize: 18)),
                  ],
                ),
                actions: [
                  CustomTextButton(
                    label: 'Cancel',
                    onPressed: () => Navigator.pop(dialogContext),
                  ),
                  CustomTextButton(
                    label: 'Delete',
                    onPressed: () {
                      // Navigator.pop(dialogContext); // pop dialog first
                      // outerContext
                      //     .read<WirelessSettingsBloc>()
                      //     .add(DeleteSavedNetwork(networkName));
                      // Navigator.pop(context);
                    },
                    textColor: dangerColor,
                  ),
                ],
              );
            },
          );
        }

        return Scaffold(
          appBar: CustomAppBar(
            title: "Network Settings",
            leftIcon: Image.asset(Images.back),
            leftIconOnTap: () => _backNavigation(outerContext),
          ),
          body: ContainerWidget(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text("Manage Networks", style: baseHeaderStyle),
                Expanded(
                    child: ListView.builder(
                  itemCount: state.savedNetworks.length,
                  itemBuilder: (context, index) {
                    final item = state.savedNetworks[index];
                    return SavedNetworkRow(
                      name: item.ssid,
                      onTap: () => showDeleteDialog(item.ssid),
                      accessPoint: item.accessPoint,
                    );
                  },
                ))
              ],
            ),
          ),
        );
      },
    );
  }
}

class SavedNetworkRow extends StatelessWidget {
  final String name;
  final VoidCallback? onTap;
  final NetworkManagerAccessPoint? accessPoint;

  const SavedNetworkRow(
      {super.key, required this.name, this.onTap, this.accessPoint});

  @override
  Widget build(BuildContext context) {
    final isAccessPointAvailable = accessPoint != null;
    // Determine the icon path based on the access point's security and signal strength
    final iconPath = isAccessPointAvailable
        ? getNetworkIcon(
            accessPoint!.rsnFlags.isNotEmpty ? "WPA2" : "Open",
            accessPoint!.strength,
          )
        : Images.securedWirelessDisabled;

    return CustomRowItem(
      onTap: isAccessPointAvailable ? onTap : null,
      title: name,
      titleStyle:
          isAccessPointAvailable ? baseHeaderStyle : secondaryHeaderStyle,
      child: Row(
        spacing: 0,
        mainAxisSize: MainAxisSize.min,
        children: [
          IconButton(
              onPressed: null, icon: CustomIcon(icon: Image.asset(iconPath))),
          IconButton(
              onPressed: onTap,
              icon: CustomIcon(
                  icon: Image.asset(isAccessPointAvailable
                      ? Images.settings
                      : Images.settingsDisabledIcon))),
        ],
      ),
    );
  }
}
