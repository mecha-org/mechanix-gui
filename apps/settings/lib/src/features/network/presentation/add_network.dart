import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/styles/text.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkBloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkEvent.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkState.dart';
import 'package:mechanix_settings/src/features/network/data/wifi_repository.dart';

class AddNetwork extends StatelessWidget {
  const AddNetwork({super.key});

  @override
  Widget build(BuildContext context) {
    final formKey = GlobalKey<FormState>();
    final nameController = TextEditingController();
    final passwordController = TextEditingController();

    // Get WifiRepository from ancestor provider
    final wifiRepository = context.read<WifiRepository>();

    void backNavigation(BuildContext context) {
      Navigator.pop(context);
    }

    void onAddButtonPressed(BuildContext context, ConnectNetworkState state) {
      if (formKey.currentState!.validate()) {
        context.read<ConnectNetworkBloc>().add(
              PasswordChanged(passwordController.text),
            );
        context.read<ConnectNetworkBloc>().add(
              ConnectToUnknownNetwork(
                nameController.text,
                passwordController.text,
              ),
            );

        if (state.error != '') {
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(
                content: Text(
              'Authentication failed!',
              style: TextStyle(color: Colors.red),
            )),
          );
        } else {
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(content: Text('connecting to network...')),
          );
          backNavigation(context);
        }
      }
    }

    return BlocProvider(
      create: (_) => ConnectNetworkBloc(wifiRepository: wifiRepository),
      child: BlocBuilder<ConnectNetworkBloc, ConnectNetworkState>(
        builder: (context, state) {
          return Scaffold(
            appBar: CustomAppBar(
              title: "Add Network",
              leftIcon: Image.asset(Images.back),
              leftIconOnTap: () => backNavigation(context),
              rightIcon2: const Icon(Icons.check),
              rightIcon2OnTap: () => onAddButtonPressed(context, state),
            ),
            body: ContainerWidget(
              child: Form(
                key: formKey,
                child: Column(
                  children: [
                    const SizedBox(height: 10),
                    // Name Field
                    Row(
                      crossAxisAlignment: CrossAxisAlignment.center,
                      children: [
                        const SizedBox(
                          width: 120,
                          child: Text("Name", style: labelTextStyle),
                        ),
                        Expanded(
                          child: TextFormField(
                            controller: nameController,
                            style: inputFieldTextStyle,
                            decoration: const InputDecoration(
                              hintText: "Enter network name",
                              border: OutlineInputBorder(),
                              focusedBorder: OutlineInputBorder(
                                borderSide: BorderSide(color: Colors.blue),
                              ),
                              isDense: true,
                              contentPadding: EdgeInsets.symmetric(
                                  horizontal: 12, vertical: 10),
                            ),
                            validator: (value) {
                              if (value == null || value.isEmpty) {
                                return 'Please enter a network name';
                              }
                              return null;
                            },
                          ),
                        ),
                      ],
                    ),
                    const SizedBox(height: 16),
                    // Password Field
                    Row(
                      crossAxisAlignment: CrossAxisAlignment.center,
                      children: [
                        const SizedBox(
                          width: 120,
                          child: Text("Password", style: labelTextStyle),
                        ),
                        Expanded(
                          child: TextFormField(
                            controller: passwordController,
                            obscureText: state.obscurePassword,
                            style: inputFieldTextStyle,
                            decoration: InputDecoration(
                              hintText: "Enter password",
                              border: const OutlineInputBorder(),
                              focusedBorder: const OutlineInputBorder(
                                borderSide: BorderSide(color: Colors.blue),
                              ),
                              isDense: true,
                              contentPadding: const EdgeInsets.symmetric(
                                  horizontal: 12, vertical: 10),
                              suffixIcon: IconButton(
                                icon: Icon(
                                  state.obscurePassword
                                      ? Icons.visibility_off
                                      : Icons.visibility,
                                ),
                                onPressed: () {
                                  context
                                      .read<ConnectNetworkBloc>()
                                      .add(TogglePasswordVisibility());
                                },
                              ),
                            ),
                            validator: (value) {
                              if (value == null || value.isEmpty) {
                                return 'Please enter a password';
                              }
                              return null;
                            },
                          ),
                        ),
                      ],
                    ),
                  ],
                ),
              ),
            ),
          );
        },
      ),
    );
  }
}
