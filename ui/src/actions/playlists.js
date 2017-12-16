import * as types from '../constants/ActionTypes'

export const playlistSearch = (error=undefined) => ({
	type: types.API,
	payload: {
		success: types.PLAYLIST_RESULT,
		url: '/playlists',
		method: 'GET',
		error: error
	},
	error: error,
	meta: {
		origin: 'playlists.search'
	}
})

export const createPlaylist = (name, error=undefined) => ({
	type: types.API,
    payload: {
        success: types.PLAYLIST_CREATED,
        url: '/playlists',
        method: 'PUT',
        error: error,
        body: JSON.stringify({
            id: "",
            name: name,
            cuesheets: []
        }),
        headers: {
            'Content-Type': 'application/json'
        }
    },
    error: error,
    meta: {
        origin: 'playlists.search'
    }
})

export const createPlaylistName = (name) => ({
	type: types.PLAYLIST_CREATE_NAME,
	payload: {
		name: name
	}
})