import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkBloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkEvent.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkState.dart';
import 'package:mechanix_settings/src/features/network/data/wifi_repository.dart';
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
      child: BlocListener<ConnectNetworkBloc, ConnectNetworkState>(
        listenWhen: (previous, current) {
          return previous.deviceState != current.deviceState ||
              previous.error != current.error;
        },
        listener: (context, state) {
          ScaffoldMessenger.of(context).clearSnackBars();

          final bool hasError =
              state.deviceState == NetworkManagerDeviceState.needAuth ||
                  (state.error != null && state.error!.isNotEmpty);

          final bool isAuthenticating =
              state.deviceState == NetworkManagerDeviceState.ipCheck ||
                  state.deviceState == NetworkManagerDeviceState.config;

          if (hasError) {
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(
                content: Text(
                  state.error ?? "Connection failed",
                  style: const TextStyle(color: Colors.red),
                ),
                duration: const Duration(seconds: 2),
                backgroundColor: Colors.grey[800],
              ),
            );
            return;
          } else if (isAuthenticating) {
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(
                content: const Text(
                  "Authenticating...",
                  style: TextStyle(color: Colors.white),
                ),
                duration: const Duration(seconds: 2),
                backgroundColor: Colors.grey[800],
              ),
            );
          } else {
            // // TODO: check connected network is same as active connection added,
            // // then navigate to previous screen
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(
                content: const Text(
                  "Connected",
                  style: TextStyle(color: Colors.white),
                ),
                duration: const Duration(seconds: 2),
                backgroundColor: Colors.grey[800],
              ),
            );
            // backNavigation(context);
          }
        },
        child: BlocBuilder<ConnectNetworkBloc, ConnectNetworkState>(
          builder: (context, state) {
            return Scaffold(
              backgroundColor: context.secondary,
              body: ContainerWidget(
                child: Form(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Row(
                        children: [
                          CustomTitle(
                            title: "Join",
                            textStyle: TextStyle(
                              color: context.onSurfaceVariant,
                              fontWeight: FontWeight.w600,
                              fontSize: 24,
                            ),
                          ),
                          CustomTitle(
                            title: "'${utf8.decode(accessPoint!.ssid)}'",
                            textStyle: TextStyle(
                              color: context.onSurface,
                              fontWeight: FontWeight.w600,
                              fontSize: 24,
                            ),
                          ),
                        ],
                      ),
                      Row(
                        crossAxisAlignment: CrossAxisAlignment.center,
                        children: [
                          Expanded(
                            child: MechanixTextInput.password(
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
                                enabledBorderSide: BorderSide(
                                  color: context.surfaceContainerLow,
                                  style: BorderStyle.solid,
                                  width: 1,
                                ),
                                widgetPadding: EdgeInsets.zero,
                                widgetDecoration: const BoxDecoration(
                                  color: Colors.transparent,
                                ),
                              ),
                            ).padBottom(24),
                          ),
                        ],
                      ),

                      // // NOTE: Not in use currently
                      // WirelessProtocols()
                      WirelessProtocols(accessPoint: accessPoint),
                    ],
                  ).padTop(8),
                ),
              ),
              bottomSheet: MechanixBottomBar(
                leadingWidget: [context.backButton],
              ),
            );
          },
        ),
      ),
    );
  }
}
