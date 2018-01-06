import * as types from '../constants/ActionTypes'

export const playlistSearch = (error=undefined) => ({
	type: types.API,
	payload: {
		success: types.PLAYLIST_RESULT,
		url: '/playlists',
		method: 'GET',
		error: error
	},
	error: error===undefined,
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
    error: error===undefined,
    meta: {
        origin: 'playlists.search'
    }
})

export const removePlaylist = (id, error=undefined) => ({
	type: types.API,
	payload: {
		success: types.PLAYLIST_REMOVED,
		url: '/playlists/' + id,
		method: 'DELETE',
		error: error,
		headers: {
            'Content-Type': 'application/json'
        }
	},
	error: error===undefined,
    meta: {
        origin: 'playlists.search'
    }
})

export const removeCuesheet = (id, cuesheet_id, error=undefined) => ({
	type: types.API,
	payload: {
        success: types.PLAYLIST_UPDATED,
        url: '/playlists/' + id + '/cuesheet/' + cuesheet_id,
        method: 'DELETE',
        error: error,
        headers: {
            'Content-Type': 'application/json'
        }
    },
    error: error===undefined,
    meta: {
        origin: 'playlists.view'
    }
})

export const createPlaylistName = (name) => ({
	type: types.PLAYLIST_CREATE_NAME,
	payload: {
		name: name
	}
})