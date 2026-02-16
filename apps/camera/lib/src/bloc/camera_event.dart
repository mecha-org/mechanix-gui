import 'package:equatable/equatable.dart';

abstract class CameraEvent extends Equatable {
  const CameraEvent();

  @override
  List<Object?> get props => [];
}

class ToggleCaptureMode extends CameraEvent {}

class ToggleSettingsMode extends CameraEvent {}
