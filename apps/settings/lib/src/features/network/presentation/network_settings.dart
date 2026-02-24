import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkBloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_bloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_event.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_state.dart';
import 'package:mechanix_settings/src/features/network/models/saved_networks.dart';
import 'package:mechanix_settings/src/features/network/presentation/saved_network_details.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/list_items/simple_list_items_type.dart';
import 'package:widgets/widgets/menu/constants/menu_positions.dart';
import 'package:widgets/widgets/menu/models/mechanix_menu_item.dart';

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
        .map((item) => SimpleListItems(
            leading: const IconWidget(
              iconPath: Images.wifi,
              boxWidth: 36,
              boxHeight: 36,
              iconWidth: 24,
              iconHeight: 24,
            ),
            title: item.ssid ?? '',
            trailing: MechanixMenu(
              dropdownSize: const Size(135, 128),
              wingSize: 50,
              animationDuration: const Duration(milliseconds: 400),
              dropdownPosition: MenuDropdownPosition.leftStart,
              items: [
                MechanixMenuItemsType(
                  title: "About",
                  leading: const IconWidget(
                    iconPath: Images.settings,
                    boxWidth: 20,
                    boxHeight: 20,
                    iconWidth: 20,
                    iconHeight: 20,
                  ),
                  onTap: () {
                    final connectNetworkBloc =
                        context.read<ConnectNetworkBloc>();
                    final wirelessSettingsBloc =
                        context.read<WirelessSettingsBloc>();

                    Navigator.push(
                      context,
                      MaterialPageRoute(
                        builder: (context) => MultiBlocProvider(
                          providers: [
                            BlocProvider.value(value: connectNetworkBloc),
                            BlocProvider.value(value: wirelessSettingsBloc),
                          ],
                          child: SavedNetworkDetails(
                            network: item,
                          ),
                        ),
                      ),
                    );
                  },
                ),
                MechanixMenuItemsType(
                  title: "Forget",
                  leading: const IconWidget(
                    iconPath: Images.blockIcon,
                    boxWidth: 20,
                    boxHeight: 20,
                    iconWidth: 16,
                    iconHeight: 16,
                  ),
                  onTap: () {
                    context
                        .read<WirelessSettingsBloc>()
                        .add(ForgetNetwork(item.ssid ?? ''));
                  },
                ),
              ],
            )))
        .toList();

    return list;
  }

  @override
  Widget build(BuildContext outerContext) {
    return BlocSelector<WirelessSettingsBloc, WirelessSettingsState,
        List<SavedWirelessNetwork>>(
      selector: (state) => state.allSavedNetworks,
      builder: (context, state) {
        return Scaffold(
          body: SingleChildScrollView(
            physics: const BouncingScrollPhysics(),
            child: ContainerWidget(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const CustomTitle(title: "Manage wireless"),
                  MechanixSimpleList(
                    physics: const BouncingScrollPhysics(),
                    listItems: getWireless(context, state),
                  ).padTop(12)
                ],
              ).padTop(8),
            ),
          ),
          bottomNavigationBar: MechanixBottomBar(
            leadingWidget: [context.backButton],
          ),
        );
      },
    );
  }
}
