import * as constants from '../constants/ActionTypes';

const playlistsReducer = (state= {}, action) => {

	if ( !state.playlists ) {
    		state = Object.assign(state, {playlistsResult: [], refresh: true});
    }

	switch(action.type) {
		case constants.PLAYLIST_RESULT: {
    			return Object.assign({}, state, {playlistsResult: action.searchResult, refresh: false})
    	}
		default:
			return state;
	}
}

export default playlistsReducer;