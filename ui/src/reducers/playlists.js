const playlistsReducer = (state= {}, action) => {

	if ( !state.playlists ) {
    		state = Object.assign(state, {playlists: { playlistsResult: []}});
    }

	switch(action.type) {
		default:
			return state;
	}
}

export default playlistsReducer;