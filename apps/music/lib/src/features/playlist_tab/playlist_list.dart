import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/models.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/features/home/widgets/title_widget.dart';
import 'package:mechanix_music/src/features/playlist_tab/add_playlist_bar.dart';
import 'package:mechanix_music/src/features/playlist_tab/empty_playlist_view.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_card.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_tile.dart';
import 'package:tuple/tuple.dart';

class PlaylistList extends StatefulWidget {
  final ScrollController scrollController;

  const PlaylistList({super.key, required this.scrollController});

  @override
  State<PlaylistList> createState() => _PlaylistListState();
}

class _PlaylistListState extends State<PlaylistList> {
  static const _newPlaylistId = '__new_playlist__';

  String _playlistName = 'New Playlist';
  String _renamePlaylistId = '';
  BottomBarView? _lastBottomBarView;

  bool get _isRenaming => _renamePlaylistId.isNotEmpty;

  void _openBottomSheet() {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      backgroundColor: Colors.transparent,
      builder:
          (_) => Container(
            decoration: BoxDecoration(
              color: Theme.of(context).scaffoldBackgroundColor,
              borderRadius: const BorderRadius.vertical(
                top: Radius.circular(16),
              ),
            ),
            child: AddPlaylistBar(
              key: const ValueKey('add_playlist'),
              initialValue: _playlistName,
              playlistId: _isRenaming ? _renamePlaylistId : null,
              onChanged: (value) => setState(() => _playlistName = value),
            ),
          ),
    ).whenComplete(() {
      if (!mounted) return;
      context.read<SongsBloc>().add(BottomBarToggle(BottomBarView.normal));
      setState(() {
        _playlistName = 'New Playlist';
        _renamePlaylistId = '';
      });
    });
  }

  List<PlaylistInfo> _buildDisplayPlaylists(
    List<PlaylistInfo> playlists,
    BottomBarView bottomBarView,
  ) {
    if (_isRenaming) {
      return playlists
          .map(
            (p) =>
                p.id == _renamePlaylistId
                    ? p.copyWith(
                      name: _playlistName,
                      coverImagePath: p.coverImagePath,
                    )
                    : p,
          )
          .toList();
    }

    if (bottomBarView == BottomBarView.add) {
      return [
        PlaylistInfo(
          id: _newPlaylistId,
          name: _playlistName,
          isShuffle: false,
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
          songIds: const [],
          coverImagePath: null,
        ),
        ...playlists,
      ];
    }

    return playlists;
  }

  void _onRename(PlaylistInfo playlist) {
    setState(() {
      _playlistName = playlist.name;
      _renamePlaylistId = playlist.id;
    });
    context.read<SongsBloc>().add(BottomBarToggle(BottomBarView.add));
  }

  void _onPlaylistTap(String playlistId) {
    context.read<SongsBloc>().add(SelectedPlaylist(playlistId));
  }

  @override
  Widget build(BuildContext context) {
    return BlocSelector<
      SongsBloc,
      SongsState,
      Tuple4<BottomBarView, PlaylistViewEnum, List<PlaylistInfo>, String?>
    >(
      selector:
          (state) => Tuple4(
            state.bottomBarView,
            state.playlistView,
            state.playlists,
            state.currentPlaylist.playlistId,
          ),
      builder: (context, data) {
        final bottomBarView = data.item1;
        final playlistView = data.item2;
        final playlists = data.item3;
        final currentPlaylistId = data.item4;

        if (bottomBarView == BottomBarView.add &&
            _lastBottomBarView != BottomBarView.add) {
          WidgetsBinding.instance.addPostFrameCallback(
            (_) => _openBottomSheet(),
          );
        }
        _lastBottomBarView = bottomBarView;

        final displayPlaylists = _buildDisplayPlaylists(
          playlists,
          bottomBarView,
        );

        return Scrollbar(
          controller: widget.scrollController,
          child: ScrollConfiguration(
            behavior: const ScrollBehavior().copyWith(
              overscroll: false,
              scrollbars: false,
              dragDevices: {PointerDeviceKind.touch, PointerDeviceKind.mouse},
            ),
            child: CustomScrollView(
              controller: widget.scrollController,
              physics: const BouncingScrollPhysics(),
              slivers: [
                const SliverToBoxAdapter(
                  child: Padding(
                    padding: EdgeInsets.only(top: 12, left: 16, right: 16),
                    child: TitleWidget(title: 'Playlists'),
                  ),
                ),
                if (displayPlaylists.isEmpty &&
                    bottomBarView != BottomBarView.add)
                  EmptyPlaylistView(playlistView: playlistView)
                else
                  playlistView == PlaylistViewEnum.grid
                      ? _GridView(
                        playlists: displayPlaylists,
                        currentPlaylistId: currentPlaylistId,
                        renamePlaylistId: _renamePlaylistId,
                        onRename: _onRename,
                        onTap: _onPlaylistTap,
                      )
                      : _ListView(
                        playlists: displayPlaylists,
                        currentPlaylistId: currentPlaylistId,
                        renamePlaylistId: _renamePlaylistId,
                        onRename: _onRename,
                        onTap: _onPlaylistTap,
                      ),

                const SliverToBoxAdapter(child: SizedBox(height: 60)),
              ],
            ),
          ),
        );
      },
    );
  }
}

class _GridView extends StatelessWidget {
  final List<PlaylistInfo> playlists;
  final String? currentPlaylistId;
  final String renamePlaylistId;
  final void Function(PlaylistInfo playlist) onRename;
  final void Function(String playlistId) onTap;

  const _GridView({
    required this.playlists,
    required this.currentPlaylistId,
    required this.renamePlaylistId,
    required this.onRename,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return SliverPadding(
      padding: const EdgeInsets.all(16),
      sliver: SliverGrid(
        gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
          crossAxisCount: 3,
          mainAxisExtent: 164,
          crossAxisSpacing: 8,
          mainAxisSpacing: 8,
        ),
        delegate: SliverChildBuilderDelegate(childCount: playlists.length, (
          context,
          index,
        ) {
          final playlist = playlists[index];
          final isNew = playlist.id == _PlaylistListState._newPlaylistId;
          final isRenaming = playlist.id == renamePlaylistId;
          final isActive = playlist.id == currentPlaylistId;

          return AnimatedContainer(
            duration: const Duration(milliseconds: 200),
            decoration:
                isRenaming
                    ? BoxDecoration(
                      borderRadius: BorderRadius.circular(8),
                      border: Border.all(
                        color: Theme.of(context).primaryColor,
                        width: 2,
                      ),
                    )
                    : null,
            child: PlaylistCard(
              playlistInfo: playlist,
              isActive: isActive,
              onRenameClick: (_) => onRename(playlist),
              onPlaylistTap:
                  (isNew || isRenaming) ? () {} : () => onTap(playlist.id),
            ),
          );
        }),
      ),
    );
  }
}

class _ListView extends StatelessWidget {
  final List<PlaylistInfo> playlists;
  final String? currentPlaylistId;
  final String renamePlaylistId;
  final void Function(PlaylistInfo playlist) onRename;
  final void Function(String playlistId) onTap;

  const _ListView({
    required this.playlists,
    required this.currentPlaylistId,
    required this.renamePlaylistId,
    required this.onRename,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return SliverPrototypeExtentList(
      prototypeItem: const SizedBox(height: 79),
      delegate: SliverChildBuilderDelegate(childCount: playlists.length, (
        context,
        index,
      ) {
        final playlist = playlists[index];
        final isNew = playlist.id == _PlaylistListState._newPlaylistId;
        final isRenaming = playlist.id == renamePlaylistId;
        final isActive = playlist.id == currentPlaylistId;

        return AnimatedContainer(
          duration: const Duration(milliseconds: 200),
          decoration:
              isRenaming
                  ? BoxDecoration(
                    color: Theme.of(
                      context,
                    ).primaryColor.withValues(alpha: 0.1),
                    border: Border(
                      left: BorderSide(
                        color: Theme.of(context).primaryColor,
                        width: 4,
                      ),
                    ),
                  )
                  : null,
          child: PlaylistTile(
            playlistInfo: playlist,
            isActive: isActive,
            isDeletePlaylist: true,
            isSelected: isNew || isRenaming,
            onRenameClick: (_) => onRename(playlist),
            onTap: (isNew || isRenaming) ? null : () => onTap(playlist.id),
          ),
        );
      }),
    );
  }
}
