import * as types from '../constants/ActionTypes'

export const playlistSearch = (error=undefined) => ({
	type: types.API,
	payload: {
		success: types.PLAYLIST_RESULT,
		url: '/playlists',
		method: 'GET',
		error: error
	},
	error: error=undefined,
	meta: {
		origin: 'playlists.search'
	}
})