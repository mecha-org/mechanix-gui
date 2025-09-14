part of 'display_bloc.dart';

class DisplayState extends Equatable {
  const DisplayState({
    this.brightness = 0.4,
    this.isAutoBrightness = true,
    this.screenTimeout = 30,
    this.lockScreenTimeout = 30,
  });

  final double brightness;
  final bool isAutoBrightness;
  final double screenTimeout;
  final double lockScreenTimeout;

  DisplayState copyWith({
    double? brightness,
    bool? isAutoBrightness,
    double? screenTimeout,
    double? lockScreenTimeout,
  }) {
    return DisplayState(
      brightness: brightness ?? this.brightness,
      isAutoBrightness: isAutoBrightness ?? this.isAutoBrightness,
      screenTimeout: screenTimeout ?? this.screenTimeout,
      lockScreenTimeout: lockScreenTimeout ?? this.lockScreenTimeout,
    );
  }

  @override
  List<Object?> get props => [
        brightness,
        isAutoBrightness,
        screenTimeout,
        lockScreenTimeout,
      ];
}
