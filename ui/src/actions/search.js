import * as types from '../constants/ActionTypes'

export const cuesheetSearch = (query, error=undefined) => ({
	type: types.API,
	payload: {
		success: types.CUESHEET_RESULT,
		url: '/search/' + query,
		method: 'GET',
		error: error
	},
	error: error===undefined,
	meta: {
		origin: 'cuesheet.search'
	}
})

export const addToListDialog = (id, title) => ({
	type: types.CUESHEET_ADD_TO_LIST_DIALOG,
	payload: {
		id: id,
		title: title
	}
})

export const addToList = (id, titleid, error=undefined) => ({
	type: types.API,
	payload: {
		success: types.PLAYLIST_UPDATED,
		url: '/playlists/' + id + '/cuesheet/' + titleid,
		id: id,
		titleid: titleid,
		method: 'PUT',
		error: error
	},
	error: error === undefined,
	meta: {
		origin: 'cuesheet.search'
	}
})

export const closeDialog = () => ({
	type: types.CUESHEET_CLOSE_DIALOG,
})