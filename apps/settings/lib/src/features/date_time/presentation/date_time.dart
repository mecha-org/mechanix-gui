import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:intl/intl.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/styles/color.dart';
import 'package:mechanix_settings/src/commons/customWidgets/switch_row.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_bloc.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_event.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_state.dart';

class DateTimeSettings extends StatelessWidget {
  const DateTimeSettings({super.key});

  void backNavigation(BuildContext context) {
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<DateTimeBloc, DateTimeState>(
      builder: (context, state) {
        final dateString =
            DateFormat('yyyy-MM-dd').format(state.dateTime ?? DateTime.now());
        final timeString =
            DateFormat('hh:mm a').format(state.dateTime ?? DateTime.now());

        return Scaffold(
          appBar: CustomAppBar(
            title: "Date & Time",
            leftIcon: Image.asset(Images.back),
            leftIconOnTap: () => backNavigation(context),
          ),
          body: SingleChildScrollView(
            child: ContainerWidget(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  CustomSwitchListTile(
                    title: "Automatic Date & Time",
                    subtitle: "Requires internet access",
                    value: state.autoDateTime,
                    onChanged: (val) {
                      context.read<DateTimeBloc>().add(ToggleAutoDateTime(val));
                    },
                  ),
                  const Divider(),
                  FixedHeightRow(
                    showTopBorder: false,
                    showBottomBorder: false,
                    child: ListTile(
                      title: Text(
                        "Set time",
                        style: TextStyle(
                          color:
                              state.autoDateTime ? Colors.grey : Colors.white,
                          fontSize: 24,
                        ),
                      ),
                      trailing: Text(
                        timeString,
                        style: TextStyle(
                          color: state.autoDateTime ? Colors.grey : selectColor,
                          fontSize: 20,
                        ),
                      ),
                      onTap: state.autoDateTime
                          ? null
                          : () {
                              Navigator.pushNamed(
                                  context, AppRoutes.timeSettings);
                            },
                    ),
                  ),
                  FixedHeightRow(
                    showTopBorder: false,
                    child: ListTile(
                      title: Text(
                        "Set date",
                        style: TextStyle(
                          color:
                              state.autoDateTime ? Colors.grey : Colors.white,
                          fontSize: 24,
                        ),
                      ),
                      trailing: Text(
                        dateString,
                        style: TextStyle(
                          color: state.autoDateTime ? Colors.grey : selectColor,
                          fontSize: 20,
                        ),
                      ),
                      onTap: state.autoDateTime
                          ? null
                          : () {
                              Navigator.pushNamed(
                                  context, AppRoutes.dateSettings);
                            },
                    ),
                  ),
                  const SizedBox(height: 40),
                  CustomSwitchListTile(
                      title: "Automatic Time Zone",
                      subtitle:
                          "Requires location services enabled and internet access",
                      value: state.autoTimeZone,
                      onChanged: (val) => {
                            // // TODO: handle with service, discuss with team
                            context
                                .read<DateTimeBloc>()
                                .add(ToggleAutoTimeZone(val))
                          }),
                  const Divider(),
                  FixedHeightRow(
                    showTopBorder: false,
                    child: ListTile(
                      title: Text(
                        "Time Zone",
                        style: TextStyle(
                          color:
                              state.autoTimeZone ? Colors.grey : Colors.white,
                          fontSize: 24,
                        ),
                      ),
                      trailing: Theme(
                        data: Theme.of(context).copyWith(
                          canvasColor: Colors.grey[900],
                          highlightColor: Colors.transparent,
                          hoverColor: Colors.transparent,
                        ),
                        child: state.loading
                            ? Visibility(
                                visible: state.loading,
                                child: Container(
                                  margin: EdgeInsets.only(right: 10),
                                  height: 20,
                                  width: 20,
                                  child: CircularProgressIndicator(
                                    value: null,
                                    strokeWidth: 2,
                                    backgroundColor: Colors.blue,
                                  ),
                                ),
                              )
                            : DropdownMenu<String>(
                                initialSelection: state.currentTimeZone,
                                enabled: !state.autoTimeZone,
                                enableSearch: true,
                                requestFocusOnTap: true,
                                menuHeight: 150,
                                width: 250,
                                textStyle: TextStyle(
                                  color: state.autoTimeZone
                                      ? Colors.grey
                                      : selectColor,
                                  fontSize: 18,
                                ),
                                inputDecorationTheme: InputDecorationTheme(
                                  border: OutlineInputBorder(
                                    borderRadius: BorderRadius.circular(8),
                                  ),
                                  isDense: true,
                                  contentPadding: const EdgeInsets.symmetric(
                                      horizontal: 12, vertical: 10),
                                ),
                                onSelected: (String? newValue) {
                                  if (newValue != null) {
                                    context
                                        .read<DateTimeBloc>()
                                        .add(SetTimeZone(newValue));
                                  }
                                },
                                dropdownMenuEntries:
                                    (state.listTimezones ?? []).map((zone) {
                                  return DropdownMenuEntry<String>(
                                    value: zone,
                                    label: zone,
                                  );
                                }).toList(),
                              ),
                      ),
                    ),
                  ),
                ],
              ),
            ),
          ),
        );
      },
    );
  }
}
