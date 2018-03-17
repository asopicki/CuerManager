import * as constants from '../constants/ActionTypes';
import _ from 'lodash'

const cuesheetSearchReducer = (state= {}, action) => {

	if ( !state.cuesheetSearch ) {
		state = Object.assign(state, {cuesheetSearch: { searchResult: []}, showDialog: false, addTitle: {id: "", title: ""}});
	}

	switch(action.type) {
		case constants.CUESHEET_CLOSE_DIALOG: {
			return Object.assign({}, state, {showDialog: false, addTitle: {id: "", title: ""}});
		}
		case constants.CUESHEET_ADD_TO_LIST_DIALOG: {
			return Object.assign({}, state, {showDialog: true, addTitle: {id: action.payload.id, title: action.payload.title}});
		}
		case constants.CUESHEET_RESULT: {
			return Object.assign({}, state, {searchResult: _.orderBy(action.payload.result, ['rhythm', 'title'], ['asc', 'asc'])})
		}
		default:
			return state;
	}
};

export default cuesheetSearchReducer;