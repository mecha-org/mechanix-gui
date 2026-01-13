import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkBloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkEvent.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkState.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_bloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_state.dart';
import 'package:mechanix_settings/src/features/network/data/wifi_repository.dart';
import 'package:mechanix_settings/src/features/network/models/types.dart';
import 'package:mechanix_settings/src/features/network/presentation/widgets/wireless_protocols.dart';
import 'package:nm/nm.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/text_input/mechanix_text_input_theme.dart';

class ConnectSecureNetwork extends StatelessWidget {
  const ConnectSecureNetwork({this.accessPoint, super.key});

  final NetworkManagerAccessPoint? accessPoint;

  @override
  Widget build(BuildContext context) {
    final wifiRepository = context.read<WifiRepository>();

    // void backNavigation(BuildContext context) {
    //   Navigator.pop(context);
    // }

    return BlocProvider(
      create: (_) => ConnectNetworkBloc(wifiRepository: wifiRepository),
      child: BlocBuilder<ConnectNetworkBloc, ConnectNetworkState>(
        builder: (context, state) {
          return ContainerWidget(
            child: Form(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      CustomTitle(
                        title: "Join",
                        textStyle: TextStyle(
                          color: context.onInverseSurface,
                          fontWeight: FontWeight.w400,
                          fontSize: 20,
                        ),
                      ),
                      CustomTitle(
                        title: "'${utf8.decode(accessPoint!.ssid)}'",
                        textStyle: TextStyle(
                          color: context.onSurface,
                          fontWeight: FontWeight.w600,
                          fontSize: 20,
                        ),
                      ),
                    ],
                  ).padOnly(top: 20),
                  Row(
                    crossAxisAlignment: CrossAxisAlignment.center,
                    children: [
                      Expanded(
                        child: BlocSelector<WirelessSettingsBloc,
                            WirelessSettingsState, WifiStatus?>(
                          selector: (state) => state.wifiState,
                          builder: (context, wifiState) {
                            return MechanixTextInput.password(
                              isFormField: true,
                              prefixIcon: const IconWidget(
                                iconPath: Images.lockIcon,
                                iconWidth: 19,
                                iconHeight: 21,
                                boxWidth: 24,
                                boxHeight: 24,
                              ).padOnly(
                                  left: 16, top: 16, right: 8, bottom: 16),
                              hintText: 'Enter Password',
                              onChanged: (value) {
                                context
                                    .read<ConnectNetworkBloc>()
                                    .add(PasswordChanged(value));
                              },
                              onFieldSubmitted: (_) {
                                if (state.password.isNotEmpty &&
                                    state.password.length >= 8) {
                                  context
                                      .read<ConnectNetworkBloc>()
                                      .add(ConnectToNetwork(accessPoint!));

                                  Navigator.pop(context);
                                }
                              },
                              validator: (value) {
                                if (value == null || value.isEmpty) {
                                  return 'Please enter a password';
                                }
                                return null;
                              },
                              theme: MechanixTextInputThemeData(
                                borderRadius: BorderRadius.circular(8),
                                borderSide: BorderSide(
                                  color: context.outlineVariant,
                                  style: BorderStyle.solid,
                                  width: 1,
                                ),
                                enabledBorderSide: BorderSide(
                                  color: context.outlineVariant,
                                  style: BorderStyle.solid,
                                  width: 1,
                                ),
                                focusedBorderSide: BorderSide(
                                  color: context.outlineVariant,
                                  style: BorderStyle.solid,
                                  width: 1,
                                ),
                                widgetPadding: EdgeInsets.zero,
                                widgetDecoration: const BoxDecoration(
                                  color: Colors.transparent,
                                ),
                              ),
                            );
                          },
                        ).padBottom(24),
                      ),
                    ],
                  ),

                  // // NOTE: Not in use currently
                  WirelessProtocols(accessPoint: accessPoint),
                ],
              ).padTop(8),
            ),
          );
        },
      ),
    );
  }
}
