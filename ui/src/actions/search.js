import * as types from '../constants/ActionTypes'

export const cuesheetSearch = (query, error=undefined) => ({
	type: types.API,
	payload: {
		success: types.CUESHEET_RESULT,
		url: '/v2/search/' + query,
		method: 'GET',
		error: error
	},
	error: error===undefined,
	meta: {
		origin: 'cuesheet.search'
	}
})

export const addToListDialog = (uuid, title) => ({
	type: types.CUESHEET_ADD_TO_LIST_DIALOG,
	payload: {
		uuid: uuid,
		title: title
	}
})

export const addToList = (uuid, titleuuid, error=undefined) => ({
	type: types.API,
	payload: {
		success: types.PLAYLIST_UPDATED,
		url: '/v2//playlists/' + uuid + '/cuesheet/' + titleuuid,
		uuid: uuid,
		titleuuid: titleuuid,
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