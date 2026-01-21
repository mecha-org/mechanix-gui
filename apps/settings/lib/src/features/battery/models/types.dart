import 'package:flutter/material.dart';
import 'package:widgets/widgets/select/select_type.dart';

class ModeOption {
  final String mode;
  final String? content;
  final List<Color> colors;

  const ModeOption({
    required this.mode,
    this.content,
    required this.colors,
  });
}

// power-saver, balanced, performance,
final List<SelectOption<String>> performanceOptions = [
  const SelectOption(value: 'power-saver', label: 'Power-save'),
  const SelectOption(value: 'balanced', label: 'Balanced'),
  const SelectOption(value: 'performance', label: 'Max Performance'),
];

ModeOption getModeDetails(String value) {
  switch (value) {
    case 'performance':
      return const ModeOption(
          mode: "Max Performance",
          content:
              "High performance mode dissipates battery quicker to enhance computations in Comet",
          colors: [
            Color.fromRGBO(155, 0, 246, 1),
            Color.fromRGBO(100, 0, 161, 0.5)
          ]);

    case 'power-saver':
      return const ModeOption(mode: "Power-save", colors: [
        Color.fromRGBO(170, 100, 0, 1),
        Color.fromRGBO(95, 54, 0, 0.5)
      ]);

    default:
      return const ModeOption(mode: "Balanced", colors: [
        Color.fromRGBO(0, 137, 0, 1),
        Color.fromRGBO(0, 76, 0, 0.5)
      ]);
  }
}
