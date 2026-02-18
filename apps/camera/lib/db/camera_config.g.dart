// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'camera_config.dart';

// **************************************************************************
// TypeAdapterGenerator
// **************************************************************************

class CameraConfigAdapter extends TypeAdapter<CameraConfig> {
  @override
  final int typeId = 0;

  @override
  CameraConfig read(BinaryReader reader) {
    final numOfFields = reader.readByte();
    final fields = <int, dynamic>{
      for (int i = 0; i < numOfFields; i++) reader.readByte(): reader.read(),
    };
    return CameraConfig(
      captureMode: fields[0] as CaptureMode,
      aspectRatio: fields[1] as CameraAspectRatio,
      resolution: fields[2] as CameraResolution,
      grid: fields[3] as CameraGrid,
      timer: fields[4] as CameraTimer,
      soundMode: fields[5] as SoundMode,
    );
  }

  @override
  void write(BinaryWriter writer, CameraConfig obj) {
    writer
      ..writeByte(6)
      ..writeByte(0)
      ..write(obj.captureMode)
      ..writeByte(1)
      ..write(obj.aspectRatio)
      ..writeByte(2)
      ..write(obj.resolution)
      ..writeByte(3)
      ..write(obj.grid)
      ..writeByte(4)
      ..write(obj.timer)
      ..writeByte(5)
      ..write(obj.soundMode);
  }

  @override
  int get hashCode => typeId.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CameraConfigAdapter &&
          runtimeType == other.runtimeType &&
          typeId == other.typeId;
}

class CaptureModeAdapter extends TypeAdapter<CaptureMode> {
  @override
  final int typeId = 1;

  @override
  CaptureMode read(BinaryReader reader) {
    switch (reader.readByte()) {
      case 0:
        return CaptureMode.photo;
      case 1:
        return CaptureMode.video;
      default:
        return CaptureMode.photo;
    }
  }

  @override
  void write(BinaryWriter writer, CaptureMode obj) {
    switch (obj) {
      case CaptureMode.photo:
        writer.writeByte(0);
        break;
      case CaptureMode.video:
        writer.writeByte(1);
        break;
    }
  }

  @override
  int get hashCode => typeId.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CaptureModeAdapter &&
          runtimeType == other.runtimeType &&
          typeId == other.typeId;
}

class SoundModeAdapter extends TypeAdapter<SoundMode> {
  @override
  final int typeId = 2;

  @override
  SoundMode read(BinaryReader reader) {
    switch (reader.readByte()) {
      case 0:
        return SoundMode.on;
      case 1:
        return SoundMode.off;
      default:
        return SoundMode.on;
    }
  }

  @override
  void write(BinaryWriter writer, SoundMode obj) {
    switch (obj) {
      case SoundMode.on:
        writer.writeByte(0);
        break;
      case SoundMode.off:
        writer.writeByte(1);
        break;
    }
  }

  @override
  int get hashCode => typeId.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SoundModeAdapter &&
          runtimeType == other.runtimeType &&
          typeId == other.typeId;
}

class CameraTimerAdapter extends TypeAdapter<CameraTimer> {
  @override
  final int typeId = 3;

  @override
  CameraTimer read(BinaryReader reader) {
    switch (reader.readByte()) {
      case 0:
        return CameraTimer.ten;
      case 1:
        return CameraTimer.five;
      case 2:
        return CameraTimer.three;
      case 3:
        return CameraTimer.off;
      default:
        return CameraTimer.ten;
    }
  }

  @override
  void write(BinaryWriter writer, CameraTimer obj) {
    switch (obj) {
      case CameraTimer.ten:
        writer.writeByte(0);
        break;
      case CameraTimer.five:
        writer.writeByte(1);
        break;
      case CameraTimer.three:
        writer.writeByte(2);
        break;
      case CameraTimer.off:
        writer.writeByte(3);
        break;
    }
  }

  @override
  int get hashCode => typeId.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CameraTimerAdapter &&
          runtimeType == other.runtimeType &&
          typeId == other.typeId;
}

class CameraGridAdapter extends TypeAdapter<CameraGrid> {
  @override
  final int typeId = 4;

  @override
  CameraGrid read(BinaryReader reader) {
    switch (reader.readByte()) {
      case 0:
        return CameraGrid.threeByThree;
      case 1:
        return CameraGrid.fourByFour;
      case 2:
        return CameraGrid.off;
      default:
        return CameraGrid.threeByThree;
    }
  }

  @override
  void write(BinaryWriter writer, CameraGrid obj) {
    switch (obj) {
      case CameraGrid.threeByThree:
        writer.writeByte(0);
        break;
      case CameraGrid.fourByFour:
        writer.writeByte(1);
        break;
      case CameraGrid.off:
        writer.writeByte(2);
        break;
    }
  }

  @override
  int get hashCode => typeId.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CameraGridAdapter &&
          runtimeType == other.runtimeType &&
          typeId == other.typeId;
}

class CameraResolutionAdapter extends TypeAdapter<CameraResolution> {
  @override
  final int typeId = 5;

  @override
  CameraResolution read(BinaryReader reader) {
    switch (reader.readByte()) {
      case 0:
        return CameraResolution.eight;
      case 1:
        return CameraResolution.four;
      case 2:
        return CameraResolution.two;
      default:
        return CameraResolution.eight;
    }
  }

  @override
  void write(BinaryWriter writer, CameraResolution obj) {
    switch (obj) {
      case CameraResolution.eight:
        writer.writeByte(0);
        break;
      case CameraResolution.four:
        writer.writeByte(1);
        break;
      case CameraResolution.two:
        writer.writeByte(2);
        break;
    }
  }

  @override
  int get hashCode => typeId.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CameraResolutionAdapter &&
          runtimeType == other.runtimeType &&
          typeId == other.typeId;
}

class CameraAspectRatioAdapter extends TypeAdapter<CameraAspectRatio> {
  @override
  final int typeId = 6;

  @override
  CameraAspectRatio read(BinaryReader reader) {
    switch (reader.readByte()) {
      case 0:
        return CameraAspectRatio.oneByOne;
      case 1:
        return CameraAspectRatio.fourByThree;
      case 2:
        return CameraAspectRatio.sixteenByNine;
      case 3:
        return CameraAspectRatio.off;
      default:
        return CameraAspectRatio.oneByOne;
    }
  }

  @override
  void write(BinaryWriter writer, CameraAspectRatio obj) {
    switch (obj) {
      case CameraAspectRatio.oneByOne:
        writer.writeByte(0);
        break;
      case CameraAspectRatio.fourByThree:
        writer.writeByte(1);
        break;
      case CameraAspectRatio.sixteenByNine:
        writer.writeByte(2);
        break;
      case CameraAspectRatio.off:
        writer.writeByte(3);
        break;
    }
  }

  @override
  int get hashCode => typeId.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CameraAspectRatioAdapter &&
          runtimeType == other.runtimeType &&
          typeId == other.typeId;
}
