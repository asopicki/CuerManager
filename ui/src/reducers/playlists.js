import * as constants from '../constants/ActionTypes';

const playlistsReducer = (state= {}, action) => {

	if ( typeof state.playlistsResult === "undefined") {
    		state = Object.assign({}, state, {playlistsResult: [], refresh: true, createForm: {name: ""}});
    }

	switch(action.type) {
		case constants.PLAYLIST_UPDATED: {
			let playlists = state.playlistsResult.filter((element) => {
				return action.payload.result.id !== element.id;
			});

			playlists.push(action.payload.result);

			return Object.assign({}, state, {playlistsResult: playlists, refresh: true});
		}
		case constants.PLAYLIST_CREATE_NAME: {
			return Object.assign({}, state, {createForm: {name: action.payload.name}})
		}
		case constants.PLAYLIST_CREATED: {
			return Object.assign({}, state, {
					playlistsResult: [...state.playlistsResult, action.payload.result],
					createForm: {name: ""}
				}
			);
		}
		case constants.PLAYLIST_RESULT: {
            return Object.assign({}, state, {playlistsResult: action.payload.result, refresh: false})
    	}
		default:
			return state;
	}
}

export default playlistsReducer;