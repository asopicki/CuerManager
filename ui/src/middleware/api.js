import * as constants from '../constants/ActionTypes';
import _ from 'lodash'

export default store => next => action => {
	switch(action.type) {
	    case constants.API: {
			console.log('Got an API request for ', action.meta.origin);
            let request = new Request(action.payload.url);
            const responseAction= data => {
                const finalAction = Object.assign({}, action, data)
                return finalAction
            }

			switch(action.payload.method) {
				case 'GET': {
					return fetch(request).then(
                        response => response.json().then(data => {
                            return next(responseAction({
                                searchResult: _.orderBy(data, ['score', 'phase', 'title'], ['desc', 'asc', 'asc']),
                                type: action.payload.success
                            }))
                        }, () => {
                            let resultAction = responseAction({
                                error: true
                            });
                            action.payload.error = new Error('Search return an error on data extraction!');
                            next(resultAction);
                        }),
                        error => {
                            let resultAction = responseAction({
                                payload: Object.assign({}, action.payload, {error: error}),
                                error: true
                            });

                            next(resultAction)
                        }
                    );
				}
				default: {
				}
			}
			break;
	    }
		default: {
			return next(action)
		}
	}
}