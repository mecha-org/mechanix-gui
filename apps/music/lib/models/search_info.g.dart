// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'search_info.dart';

// **************************************************************************
// TypeAdapterGenerator
// **************************************************************************

class SearchInfoAdapter extends TypeAdapter<SearchInfo> {
  @override
  final int typeId = 3;

  @override
  SearchInfo read(BinaryReader reader) {
    final numOfFields = reader.readByte();
    final fields = <int, dynamic>{
      for (int i = 0; i < numOfFields; i++) reader.readByte(): reader.read(),
    };
    return SearchInfo(
      id: fields[0] as String,
      isPlaylist: fields[1] as bool,
      songInfo: fields[2] as SongInfo?,
      playlistInfo: fields[3] as PlaylistInfo?,
      createdAt: fields[4] as DateTime,
    );
  }

  @override
  void write(BinaryWriter writer, SearchInfo obj) {
    writer
      ..writeByte(5)
      ..writeByte(0)
      ..write(obj.id)
      ..writeByte(1)
      ..write(obj.isPlaylist)
      ..writeByte(2)
      ..write(obj.songInfo)
      ..writeByte(3)
      ..write(obj.playlistInfo)
      ..writeByte(4)
      ..write(obj.createdAt);
  }

  @override
  int get hashCode => typeId.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SearchInfoAdapter &&
          runtimeType == other.runtimeType &&
          typeId == other.typeId;
}
