// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'song_info.dart';

// **************************************************************************
// TypeAdapterGenerator
// **************************************************************************

class SongInfoAdapter extends TypeAdapter<SongInfo> {
  @override
  final int typeId = 0;

  @override
  SongInfo read(BinaryReader reader) {
    final numOfFields = reader.readByte();
    final fields = <int, dynamic>{
      for (int i = 0; i < numOfFields; i++) reader.readByte(): reader.read(),
    };
    return SongInfo(
      id: fields[0] as String,
      index: fields[8] as int,
      path: fields[1] as String,
      title: fields[2] as String,
      artist: fields[3] as String,
      album: fields[4] as String?,
      duration: fields[5] as String?,
      artworkPath: fields[7] as String?,
      isFavourite: fields[6] as bool,
      playlistIds: (fields[9] as List).cast<String>(),
    );
  }

  @override
  void write(BinaryWriter writer, SongInfo obj) {
    writer
      ..writeByte(10)
      ..writeByte(0)
      ..write(obj.id)
      ..writeByte(1)
      ..write(obj.path)
      ..writeByte(2)
      ..write(obj.title)
      ..writeByte(3)
      ..write(obj.artist)
      ..writeByte(4)
      ..write(obj.album)
      ..writeByte(5)
      ..write(obj.duration)
      ..writeByte(6)
      ..write(obj.isFavourite)
      ..writeByte(7)
      ..write(obj.artworkPath)
      ..writeByte(8)
      ..write(obj.index)
      ..writeByte(9)
      ..write(obj.playlistIds);
  }

  @override
  int get hashCode => typeId.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SongInfoAdapter &&
          runtimeType == other.runtimeType &&
          typeId == other.typeId;
}
