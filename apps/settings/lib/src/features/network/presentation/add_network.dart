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
import 'package:mechanix_settings/src/features/network/presentation/widgets/wireless_protocols.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/text_input/mechanix_text_input_theme.dart';

class AddNetwork extends StatefulWidget {
  const AddNetwork({super.key});

  @override
  State<AddNetwork> createState() => _AddNetworkState();
}

class _AddNetworkState extends State<AddNetwork> {
  @override
  Widget build(BuildContext context) {
    final formKey = GlobalKey<FormState>();
    final wifiRepository = context.read<WifiRepository>();

    void onAddButtonPressed(BuildContext context, ConnectNetworkState state) {
      if (formKey.currentState!.validate()) {
        context.read<ConnectNetworkBloc>().add(
              ConnectToHiddenNetwork(
                state.username,
                state.password,
              ),
            );
      }
    }

    return BlocProvider(
      create: (_) => ConnectNetworkBloc(wifiRepository: wifiRepository),
      child: BlocBuilder<ConnectNetworkBloc, ConnectNetworkState>(
        builder: (context, state) {
          return ContainerWidget(
            child: Column(
              spacing: 0,
              children: [
                Form(
                  key: formKey,
                  child: Column(
                    children: [
                      CustomTitle(
                        title: "Add wireless network",
                        textStyle: TextStyle(
                          color: context.onSurface,
                          fontWeight: FontWeight.w400,
                          fontSize: 20,
                        ),
                      ).padOnly(
                        top: 20,
                        bottom: 20,
                      ),
                      MechanixTextInput.textInput(
                        hintText: 'Name',
                        isFormField: true,
                        theme: MechanixTextInputThemeData(
                          widgetHeight: 58,
                          borderRadius: BorderRadius.circular(8),
                          enabledBorderSide: BorderSide(
                            color: context.outline,
                            style: BorderStyle.solid,
                            width: 1,
                          ),
                          borderSide: BorderSide(
                            color: context.outline,
                            style: BorderStyle.solid,
                            width: 1,
                          ),
                          focusedBorderSide: BorderSide(
                            color: context.outlineVariant,
                            style: BorderStyle.solid,
                            width: 1,
                          ),
                          contentPadding: const EdgeInsets.only(
                              left: 16, top: 16, right: 16, bottom: 16),
                          widgetPadding: EdgeInsets.zero,
                          widgetDecoration: BoxDecoration(
                            color: context.surfaceContainerHigh,
                          ),
                        ),
                        prefixIcon: IconWidget(
                          iconPath: Images.wifi,
                          iconWidth: 19,
                          iconHeight: 21,
                          boxWidth: 24,
                          boxHeight: 24,
                          iconColor: context.outline,
                        ).padOnly(left: 16, top: 16, right: 8, bottom: 16),
                        onChanged: (value) {
                          context
                              .read<ConnectNetworkBloc>()
                              .add(UsernameChanged(value));
                        },
                      ).padBottom(24),
                      BlocBuilder<WirelessSettingsBloc, WirelessSettingsState>(
                        builder: (context, wirelessState) {
                          return MechanixTextInput.password(
                            hintText: 'Enter Password',
                            isFormField: true,
                            prefixIcon: IconWidget(
                              iconPath: Images.lockIcon,
                              iconWidth: 19,
                              iconHeight: 21,
                              boxWidth: 24,
                              boxHeight: 24,
                              iconColor: context.outline,
                            ).padOnly(left: 16, top: 16, right: 8, bottom: 16),
                            theme: MechanixTextInputThemeData(
                              widgetHeight: 58,
                              borderRadius: BorderRadius.circular(8),
                              enabledBorderSide: BorderSide(
                                color: context.outline,
                                style: BorderStyle.solid,
                                width: 1,
                              ),
                              borderSide: BorderSide(
                                color: context.outline,
                                style: BorderStyle.solid,
                                width: 1,
                              ),
                              focusedBorderSide: BorderSide(
                                color: context.outlineVariant,
                                style: BorderStyle.solid,
                                width: 1,
                              ),
                              widgetPadding: EdgeInsets.zero,
                              widgetDecoration: BoxDecoration(
                                color: context.surfaceContainerHigh,
                              ),
                            ),
                            onChanged: (value) {
                              context
                                  .read<ConnectNetworkBloc>()
                                  .add(PasswordChanged(value));
                            },
                            onFieldSubmitted: (_) {
                              if (state.password.isNotEmpty &&
                                  state.password.length >= 8) {
                                onAddButtonPressed(context, state);
                                Navigator.pop(context);
                              }
                            },
                          );
                        },
                      ).padBottom(32),
                      // // NOTE: Not in use currently
                    ],
                  ),
                ),
                const WirelessProtocols(),
              ],
            ),
          ).padTop(8);
        },
      ),
    );
  }
}
