import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_icon.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_row_item.dart';

import 'package:mechanix_settings/src/commons/styles/custom_styles.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_bloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_event.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_state.dart';
import 'package:mechanix_settings/src/features/network/models/saved_networks.dart';
import 'package:mechanix_settings/src/features/network/presentation/saved_network_details.dart';
import 'package:mechanix_settings/src/features/network/presentation/wireless.dart';
import 'package:nm/nm.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/listItems/simple_list_items_type.dart';

class NetworkSettings extends StatefulWidget {
  const NetworkSettings({super.key});

  @override
  State<NetworkSettings> createState() => _NetworkSettingsState();
}

class _NetworkSettingsState extends State<NetworkSettings> {
  @override
  void initState() {
    super.initState();
    context.read<WirelessSettingsBloc>().add(GetSavedNetworksEvent());
  }

  void onItemTap(SavedWirelessNetwork network) {
    final bloc = context.read<WirelessSettingsBloc>();

    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => BlocProvider.value(
          value: bloc, 
          child: SavedNetworkDetails(network: network),
        ),
      ),
    );
  }

  List<SimpleListItems> getWireless(
      BuildContext context, List<SavedWirelessNetwork> savedNetworks) {
    final list = savedNetworks
        .map((d) => SimpleListItems(
            title: d.ssid ?? '',
            trailing: IconButton(
              onPressed: () => onItemTap(d),
              icon: const SizedBox(
                height: 24,
                width: 24,
                child: IconWidget(iconPath: Images.settings),
              ),
            )))
        .toList();

    return list;
  }

  @override
  Widget build(BuildContext outerContext) {
    // Rename to outerContext

    return BlocSelector<WirelessSettingsBloc, WirelessSettingsState,
        List<SavedWirelessNetwork>>(
      selector: (state) => state.allSavedNetworks,
      builder: (context, state) {
        return Scaffold(
          appBar: PreferredSize(
              preferredSize: const Size.fromHeight(52),
              child: const MechanixNavigationBar(
                title: "Network Settings",
              ).padHorizontal(12)),
          body: SingleChildScrollView(
            padding: const EdgeInsets.all(16.0),
            physics: const BouncingScrollPhysics(),
            child: ContainerWidget(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  MechanixSimpleList(
                    physics: const BouncingScrollPhysics(),
                    listItems: getWireless(context, state),
                  )
                ],
              ).padTop(8),
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
