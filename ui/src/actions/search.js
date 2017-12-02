import * as types from '../constants/ActionTypes'

export const cuesheetSearch = (query, error=undefined) => ({
	type: types.API,
	payload: {
		success: types.CUESHEET_RESULT,
		url: '/search/' + query,
		method: 'GET',
		error: error
	},
	error: error=undefined,
	meta: {
		origin: 'cuesheet.search'
	}
})
