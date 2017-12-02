import { combineReducers } from 'redux'

import cuesheetSearchReducer from './search'
import playlistsReducer from './playlists'

const rootReducer = combineReducers({
	cuesheetSearch: cuesheetSearchReducer,
	playlists: playlistsReducer
})

export default rootReducer