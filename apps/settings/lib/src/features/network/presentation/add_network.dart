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
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/text_input/mechanix_text_input_theme.dart';

class AddNetwork extends StatelessWidget {
  const AddNetwork({super.key});

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
          return Scaffold(
            backgroundColor: context.secondary,
            body: SingleChildScrollView(
              physics: const BouncingScrollPhysics(),
              child: ContainerWidget(
                child: Column(
                  spacing: 0,
                  children: [
                    Form(
                      key: formKey,
                      child: Column(
                        children: [
                          CustomTitle(
                            title: "Add Wireless Network",
                            textStyle: TextStyle(
                              color: context.onSurfaceVariant,
                              fontWeight: FontWeight.w600,
                              fontSize: 24,
                            ),
                          ).padOnly(top: 32, bottom: 20),
                          MechanixTextInput.textInput(
                            hintText: 'Name',
                            isFormField: true,
                            theme: MechanixTextInputThemeData(
                              borderRadius: BorderRadius.circular(8),
                              enabledBorderSide: BorderSide(
                                color: context.surfaceContainerLow,
                                style: BorderStyle.solid,
                                width: 1,
                              ),
                              contentPadding: const EdgeInsets.only(
                                  left: 16, top: 16, right: 16, bottom: 16),
                              widgetPadding: EdgeInsets.zero,
                              widgetDecoration: const BoxDecoration(
                                color: Colors.transparent,
                              ),
                            ),
                            onChanged: (value) {
                              context
                                  .read<ConnectNetworkBloc>()
                                  .add(UsernameChanged(value));
                            },
                          ).padBottom(16),
                          MechanixTextInput.password(
                            hintText: 'Enter Password',
                            isFormField: true,
                            prefixIcon: const IconWidget(
                              iconPath: Images.lockIcon,
                              iconWidth: 19,
                              iconHeight: 21,
                              boxWidth: 24,
                              boxHeight: 24,
                            ).padOnly(left: 16, top: 16, right: 8, bottom: 16),
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
                            onChanged: (value) {
                              context
                                  .read<ConnectNetworkBloc>()
                                  .add(PasswordChanged(value));
                            },
                            onFieldSubmitted: (_) {
                              if (state.password.isNotEmpty &&
                                  state.password.length >= 8) {
                                onAddButtonPressed(context, state);
                              }
                            },
                          ).padBottom(32),
                          // // NOTE: Not in use currently
                          // WirelessProtocols()
                        ],
                      ),
                    ),
                    const WirelessProtocols(),
                  ],
                ),
              ).padTop(8),
            ),
            bottomSheet: MechanixBottomBar(
              leadingWidget: [context.backButton],
            ),
          );
        },
      ),
    );
  }
}
