import * as constants from '../constants/ActionTypes';

const cuesheetSearchReducer = (state= {}, action) => {

	if ( !state.cuesheetSearch ) {
		state = Object.assign(state, {cuesheetSearch: { searchResult: []}});
	}

	switch(action.type) {
		case constants.CUESHEET_RESULT: {
			return Object.assign({}, state, {searchResult: action.searchResult})
		}
		default:
			return state;
	}
};

export default cuesheetSearchReducer;