import * as constants from '../constants/ActionTypes';
import _ from 'lodash'

const cuesheetSearchReducer = (state= {}, action) => {

	if ( !state.cuesheetSearch ) {
		state = Object.assign(state, {cuesheetSearch: { searchResult: []}});
	}

	switch(action.type) {
		case constants.CUESHEET_RESULT: {
			return Object.assign({}, state, {searchResult: _.orderBy(action.searchResult, ['score', 'phase', 'title'], ['desc', 'asc', 'asc'])})
		}
		default:
			return state;
	}
};

export default cuesheetSearchReducer;