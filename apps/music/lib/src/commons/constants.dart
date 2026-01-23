class TableName {
  static const String songsInfoTable = "musicSongsTablev4";
  static const String recentlyPlayedTable = "recentlyPlayedTablev2";
  static const String playlistTable = "playlistTablev3";
  static const String searchTable = "searchTablev2";
}

class Constants {
  static const double recentlyPlayedLimit = 9;
  static const double playlistLimit = 10;
  static const double maxSongsPerPlaylist = 30;
  static const double maxSearchItems = 30;
  static const Duration debounceDuration = Duration(milliseconds: 300);
  static const String musicDir = '/home/mecha/Music';
}

const audioExt = ['.mp3', '.wav', '.flac', '.m4a', '.aac', '.ogg', '.opus'];
