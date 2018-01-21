import * as types from '../constants/ActionTypes'

export const playlistSearch = (error=undefined) => ({
	type: types.API,
	payload: {
		success: types.PLAYLIST_RESULT,
		url: '/v2/playlists',
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
        url: '/v2/playlists',
        method: 'PUT',
        error: error,
        body: JSON.stringify({
            name: name,
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
		url: '/v2/playlists/' + id,
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
        url: '/v2/playlists/' + id + '/cuesheet/' + cuesheet_id,
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